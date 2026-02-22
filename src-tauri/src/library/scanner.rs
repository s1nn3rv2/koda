use std::path::{Path, PathBuf};
use std::time::Instant;

use lofty::config::{ParseOptions, ParsingMode};
use lofty::file::TaggedFileExt;
use lofty::prelude::AudioFile;
use lofty::probe::Probe;
use lofty::tag::Accessor;
use lofty::tag::ItemKey;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::database::Track;
use crate::settings::scanner::PARALLEL_BATCH_LIMIT;

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

pub fn find_audio_files(directories: &[PathBuf]) -> Vec<PathBuf> {
    let start = Instant::now();
    let mut all_files = Vec::new();

    for directory in directories {
        let files: Vec<PathBuf> = WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && is_audio_file(e.path()))
            .map(|e| e.path().to_path_buf())
            .collect();
        all_files.extend(files);
    }

    println!(
        "Found {} audio files in {:?}",
        all_files.len(),
        start.elapsed()
    );
    all_files
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

    let (
        title,
        artists,
        album,
        album_artist,
        genre,
        track_number,
        disc_number,
        release_date,
        cover_data,
    ): (
        Option<String>,
        Vec<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<Vec<u8>>,
    ) = if let Some(tag) = tag {
        let cover_data = tag.pictures().first().map(|pic| pic.data().to_vec());

        let artists_list: Vec<String> = tag
            .get_items(ItemKey::TrackArtist)
            .filter_map(|item| item.value().text())
            .map(|s| s.to_string())
            .collect();

        let album_artists_list: Vec<String> = tag
            .get_items(ItemKey::AlbumArtist)
            .filter_map(|item| item.value().text())
            .map(|s| s.to_string())
            .collect();

        (
            tag.title().map(|s| s.to_string()),
            artists_list,
            tag.album().map(|s| s.to_string()),
            if album_artists_list.is_empty() {
                None
            } else {
                Some(album_artists_list.join(";"))
            },
            tag.genre().map(|s| s.to_string()),
            tag.track().map(|n| n as i32),
            tag.disk().map(|n| n as i32),
            tag.get(ItemKey::ReleaseDate)
                .and_then(|i| i.value().text())
                .or_else(|| {
                    tag.get(ItemKey::RecordingDate)
                        .and_then(|i| i.value().text())
                })
                .or_else(|| {
                    tag.get(ItemKey::OriginalReleaseDate)
                        .and_then(|i| i.value().text())
                })
                .map(|s| s.to_string()),
            cover_data,
        )
    } else {
        (
            None,
            Vec::<String>::new(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    };

    let mut track = Track::new(path_str.to_string());
    track.title = title;
    track.artists = if artists.is_empty() {
        None
    } else {
        Some(artists.join(";"))
    };
    track.album = album;
    track.album_artist = album_artist;
    track.genre = genre;
    track.duration = duration;
    track.track_number = track_number;
    track.disc_number = disc_number;
    track.release_date = release_date;

    Some(ScanResult { track, cover_data })
}

pub fn scan_files_parallel(paths: &[PathBuf]) -> Vec<ScanResult> {
    let start = Instant::now();

    let results: Vec<ScanResult> = paths
        .par_chunks(PARALLEL_BATCH_LIMIT)
        .flat_map(|chunk| chunk.par_iter().filter_map(|path| scan_file(path)))
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
