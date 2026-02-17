use std::fs;
use std::path::PathBuf;

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use sha2::{Digest, Sha256};

const THUMB_SIZE: u32 = 128;

pub struct CoverArtCache {
    cache_dir: PathBuf,
}

impl CoverArtCache {
    pub fn new(cache_dir: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create cache dir: {}", e))?;
        Ok(Self { cache_dir })
    }

    pub fn hash_cover_data(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn get_thumbnail_path(&self, hash: &str) -> PathBuf {
        self.cache_dir.join(format!("{}_128.jpg", hash))
    }

    pub fn save_thumbnail_with_hash(&self, data: &[u8], hash: &str) -> Result<(), String> {
        let thumb_path = self.get_thumbnail_path(hash);

        if thumb_path.exists() {
            return Ok(());
        }

        let thumbnail = resize_image(data, THUMB_SIZE)?;
        fs::write(&thumb_path, thumbnail)
            .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

        Ok(())
    }

    pub fn read_cover_art(
        &self,
        hash: &str,
        data: Option<&[u8]>,
        size: Option<u32>,
    ) -> Result<Vec<u8>, String> {
        let requested_size = size.unwrap_or(THUMB_SIZE);

        if requested_size == THUMB_SIZE {
            let thumb_path = self.get_thumbnail_path(hash);
            return fs::read(&thumb_path).map_err(|e| format!("Failed to read thumbnail: {}", e));
        }

        // for any size > 128, generate on-demand from original data
        let original_data = data.ok_or("Original cover data required for non-thumbnail sizes")?;
        resize_image(original_data, requested_size)
    }

    pub fn has_thumbnail(&self, hash: &str) -> bool {
        self.get_thumbnail_path(hash).exists()
    }
}

fn resize_image(data: &[u8], size: u32) -> Result<Vec<u8>, String> {
    use image::ImageReader;
    use std::io::Cursor;

    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess format: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to load image: {}", e))?;

    let thumbnail = img.resize_to_fill(size, size, FilterType::CatmullRom);

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    let mut encoder = JpegEncoder::new_with_quality(&mut cursor, 80);
    encoder
        .encode(
            thumbnail.as_bytes(),
            thumbnail.width(),
            thumbnail.height(),
            thumbnail.color().into(),
        )
        .map_err(|e| format!("Failed to encode thumbnail: {}", e))?;

    Ok(buffer)
}
