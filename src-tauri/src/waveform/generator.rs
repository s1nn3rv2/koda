use std::fs::File;
use std::path::Path;

use symphonia::core::audio::AudioBufferRef;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use super::models::WaveformData;

const WAVEFORM_POINTS: usize = 200;
const SAMPLE_SKIP: usize = 100; // skip every N frames for performance

pub fn generate(audio_path: &str) -> Result<WaveformData, String> {
    let file = File::open(audio_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // try to guess format from extension
    let mut hint = Hint::new();
    if let Some(extension) = Path::new(audio_path).extension() {
        if let Some(ext_str) = extension.to_str() {
            hint.with_extension(ext_str);
        }
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("Failed to probe file: {}", e))?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| "No audio track found".to_string())?;

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100) as f64;
    let n_frames = track.codec_params.n_frames.unwrap_or(0);

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| format!("Failed to create decoder: {}", e))?;

    let mut samples = Vec::new();
    let mut frame_count = 0u64;

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(_)) | Err(Error::ResetRequired) => break,
            Err(e) => return Err(format!("Error reading packet: {}", e)),
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                if frame_count % SAMPLE_SKIP as u64 == 0 {
                    let amplitude = calculate_amplitude(&decoded);
                    samples.push(amplitude);
                }

                frame_count += 1;
            }
            Err(Error::DecodeError(_)) => continue,
            Err(e) => return Err(format!("Decode error: {}", e)),
        }
    }

    let normalized_samples = normalize_sample_count(samples, WAVEFORM_POINTS);

    let duration = if n_frames > 0 && sample_rate > 0.0 {
        n_frames as f64 / sample_rate
    } else {
        0.0
    };

    Ok(WaveformData {
        samples: normalized_samples,
        duration,
    })
}

fn calculate_amplitude(buffer: &AudioBufferRef) -> f32 {
    match buffer {
        AudioBufferRef::F32(buf) => calculate_rms_f32(buf),
        AudioBufferRef::F64(buf) => calculate_rms_f64(buf),
        AudioBufferRef::S16(buf) => calculate_rms_s16(buf),
        AudioBufferRef::S32(buf) => calculate_rms_s32(buf),
        _ => 0.5, // fallback for less common formats
    }
}

fn calculate_rms_f32(buf: &symphonia::core::audio::AudioBuffer<f32>) -> f32 {
    let mut sum = 0.0f64;
    let mut count = 0;

    for plane in buf.planes().planes() {
        for &sample in plane.iter() {
            sum += (sample as f64).powi(2);
            count += 1;
        }
    }

    if count > 0 {
        (sum / count as f64).sqrt() as f32
    } else {
        0.0
    }
}

fn calculate_rms_f64(buf: &symphonia::core::audio::AudioBuffer<f64>) -> f32 {
    let mut sum = 0.0f64;
    let mut count = 0;

    for plane in buf.planes().planes() {
        for &sample in plane.iter() {
            sum += sample.powi(2);
            count += 1;
        }
    }

    if count > 0 {
        (sum / count as f64).sqrt() as f32
    } else {
        0.0
    }
}

fn calculate_rms_s16(buf: &symphonia::core::audio::AudioBuffer<i16>) -> f32 {
    let mut sum = 0.0f64;
    let mut count = 0;

    for plane in buf.planes().planes() {
        for &sample in plane.iter() {
            let normalized = sample as f64 / i16::MAX as f64;
            sum += normalized.powi(2);
            count += 1;
        }
    }

    if count > 0 {
        (sum / count as f64).sqrt() as f32
    } else {
        0.0
    }
}

fn calculate_rms_s32(buf: &symphonia::core::audio::AudioBuffer<i32>) -> f32 {
    let mut sum = 0.0f64;
    let mut count = 0;

    for plane in buf.planes().planes() {
        for &sample in plane.iter() {
            let normalized = sample as f64 / i32::MAX as f64;
            sum += normalized.powi(2);
            count += 1;
        }
    }

    if count > 0 {
        (sum / count as f64).sqrt() as f32
    } else {
        0.0
    }
}

fn normalize_sample_count(samples: Vec<f32>, target_len: usize) -> Vec<f32> {
    if samples.is_empty() {
        return vec![0.0; target_len];
    }

    if samples.len() == target_len {
        return samples;
    }

    let mut result = Vec::with_capacity(target_len);
    let ratio = samples.len() as f32 / target_len as f32;

    for i in 0..target_len {
        let source_idx = (i as f32 * ratio) as usize;
        let source_idx = source_idx.min(samples.len() - 1);
        result.push(samples[source_idx]);
    }

    result
}
