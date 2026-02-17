use std::path::{Path, PathBuf};
use std::time::Instant;

use lofty::config::{ParseOptions, ParsingMode};
use lofty::file::TaggedFileExt;
use lofty::prelude::AudioFile;
use lofty::probe::Probe;
use lofty::tag::Accessor;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::database::Track;

const AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "ogg", "m4a", "wav", "opus", "aac"];

#[derive(Debug)]
pub struct ScanResult {
    pub track: Track,
    pub cover_data: Option<Vec<u8>>,
}

fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn find_audio_files(directory: &Path) -> Vec<PathBuf> {
    let start = Instant::now();

    let files: Vec<PathBuf> = WalkDir::new(directory)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_audio_file(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect();

    println!("Found {} audio files in {:?}", files.len(), start.elapsed());
    files
}

pub fn scan_file(path: &Path) -> Option<ScanResult> {
    let path_str = path.to_str()?;

    let parse_options = ParseOptions::new()
        .parsing_mode(ParsingMode::Relaxed)
        .read_properties(true);

    let tagged_file = Probe::open(path).ok()?.options(parse_options).read().ok()?;

    let properties = tagged_file.properties();
    let duration = Some(properties.duration().as_secs() as i64);

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    let (title, artists, album, genre, track_number, cover_data) = if let Some(tag) = tag {
        let cover_data = tag.pictures().first().map(|pic| pic.data().to_vec());

        (
            tag.title().map(|s| s.to_string()),
            tag.artist().map(|s| s.to_string()),
            tag.album().map(|s| s.to_string()),
            tag.genre().map(|s| s.to_string()),
            tag.track().map(|n| n as i32),
            cover_data,
        )
    } else {
        (None, None, None, None, None, None)
    };

    let mut track = Track::new(path_str.to_string());
    track.title = title;
    track.artists = artists;
    track.album = album;
    track.genre = genre;
    track.duration = duration;
    track.track_number = track_number;

    Some(ScanResult { track, cover_data })
}

pub fn scan_files_parallel(paths: &[PathBuf]) -> Vec<ScanResult> {
    let start = Instant::now();

    let results: Vec<ScanResult> = paths
        .par_iter()
        .filter_map(|path| scan_file(path))
        .collect();

    println!(
        "Scanned {} files in {:?} ({:.2} files/sec)",
        results.len(),
        start.elapsed(),
        results.len() as f64 / start.elapsed().as_secs_f64()
    );

    results
}

pub fn get_cover_art_from_file(path: &Path) -> Option<Vec<u8>> {
    let parse_options = ParseOptions::new()
        .parsing_mode(ParsingMode::Relaxed)
        .read_properties(false);

    let tagged_file = Probe::open(path).ok()?.options(parse_options).read().ok()?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())?;

    tag.pictures()
        .first()
        .map(|picture| picture.data().to_vec())
}
