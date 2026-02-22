use std::fs;
use std::path::PathBuf;

use crate::settings::cache::THUMB_SIZE;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use sha2::{Digest, Sha256};

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

    fn get_main_thumb_path(&self, hash: &str) -> PathBuf {
        self.cache_dir.join(format!("{}_thumb.jpg", hash))
    }

    pub fn save_thumbnail_with_hash(&self, data: &[u8], hash: &str) -> Result<(), String> {
        let thumb_path = self.get_main_thumb_path(hash);

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
        let main_path = self.get_main_thumb_path(hash);
        if main_path.exists() {
            return fs::read(&main_path)
                .map_err(|e| format!("Failed to read main thumbnail: {}", e));
        }

        let old_path = self.cache_dir.join(format!("{}_128.jpg", hash));
        if old_path.exists() {
            return fs::read(&old_path).map_err(|e| format!("Failed to read old thumbnail: {}", e));
        }

        if let Some(s) = size {
            let size_path = self.cache_dir.join(format!("{}_{}.jpg", hash, s));
            if size_path.exists() {
                return fs::read(&size_path)
                    .map_err(|e| format!("Failed to read sized thumbnail: {}", e));
            }
        }

        if let Some(original_data) = data {
            let requested_size = size.unwrap_or(THUMB_SIZE);
            return resize_image(original_data, requested_size);
        }

        Err("Cover art not found and original data missing".to_string())
    }

    pub fn has_thumbnail(&self, hash: &str) -> bool {
        self.get_main_thumb_path(hash).exists()
            || self.cache_dir.join(format!("{}_128.jpg", hash)).exists()
    }

    #[allow(dead_code)]
    pub async fn save_external_image(&self, url: &str) -> Result<String, String> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Failed to download image: {}", e))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to get image bytes: {}", e))?;

        let hash = self.hash_cover_data(&bytes);
        self.save_thumbnail_with_hash(&bytes, &hash)?;

        Ok(hash)
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

    let rgb_img = thumbnail.to_rgb8();

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    let mut encoder = JpegEncoder::new_with_quality(&mut cursor, 90);
    encoder
        .encode(
            &rgb_img,
            rgb_img.width(),
            rgb_img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| format!("Failed to encode thumbnail: {}", e))?;

    Ok(buffer)
}
