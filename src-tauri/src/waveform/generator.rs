use std::fs::File;
use std::path::Path;

use symphonia::core::audio::{AudioBuffer, AudioBufferRef};
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample;

use super::models::WaveformData;
use crate::settings::waveform::WAVEFORM_POINTS;

pub fn generate(audio_path: &str) -> Result<WaveformData, String> {
    let file = File::open(audio_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

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
                let amplitude = rms_amplitude(&decoded);
                samples.push(amplitude);
            }
            Err(Error::DecodeError(_)) => continue,
            Err(e) => return Err(format!("Decode error: {}", e)),
        }
    }

    let downsampled = downsample_max(samples, WAVEFORM_POINTS);
    let normalized = normalize(downsampled);

    let duration = if n_frames > 0 && sample_rate > 0.0 {
        n_frames as f64 / sample_rate
    } else {
        0.0
    };

    Ok(WaveformData {
        samples: normalized,
        duration,
    })
}

fn rms_amplitude(buffer: &AudioBufferRef) -> f32 {
    match buffer {
        AudioBufferRef::F32(buf) => rms(buf, |s| s as f64),
        AudioBufferRef::F64(buf) => rms(buf, |s| s),
        AudioBufferRef::S16(buf) => rms(buf, |s| s as f64 / i16::MAX as f64),
        AudioBufferRef::S32(buf) => rms(buf, |s| s as f64 / i32::MAX as f64),
        _ => 0.0,
    }
}

fn rms<S: Sample, F: Fn(S) -> f64>(buf: &AudioBuffer<S>, to_f64: F) -> f32 {
    let mut sum = 0.0f64;
    let mut count = 0usize;

    for plane in buf.planes().planes() {
        for &sample in plane.iter() {
            let s = to_f64(sample);
            sum += s * s;
            count += 1;
        }
    }

    if count > 0 {
        (sum / count as f64).sqrt() as f32
    } else {
        0.0
    }
}

fn downsample_max(samples: Vec<f32>, target_len: usize) -> Vec<f32> {
    if samples.is_empty() {
        return vec![0.0; target_len];
    }

    if samples.len() <= target_len {
        let mut result = samples;
        result.resize(target_len, 0.0);
        return result;
    }

    let mut result = Vec::with_capacity(target_len);
    let bucket_size = samples.len() as f64 / target_len as f64;

    for i in 0..target_len {
        let start = (i as f64 * bucket_size) as usize;
        let end = (((i + 1) as f64) * bucket_size) as usize;
        let end = end.min(samples.len());

        let max = samples[start..end].iter().copied().fold(0.0f32, f32::max);

        result.push(max);
    }

    result
}

fn normalize(mut samples: Vec<f32>) -> Vec<f32> {
    let max = samples.iter().copied().fold(0.0f32, f32::max);

    if max > 0.0 {
        for sample in &mut samples {
            *sample /= max;
        }
    }

    samples
}
