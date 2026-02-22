use crate::settings::api::{
    ITUNES_HIGH_RES_COVER_TARGET, ITUNES_LOW_RES_COVER_SOURCE, ITUNES_OG_IMAGE_DIMENSION_SOURCE,
};
use crate::sources::utils::normalize_string;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ItunesResponse {
    results: Vec<AlbumResult>,
}

#[derive(Deserialize, Debug)]
struct ItunesArtistResponse {
    results: Vec<ArtistResult>,
}

#[derive(Deserialize, Debug)]
struct ArtistResult {
    #[serde(rename = "artistLinkUrl")]
    artist_link_url: String,
}

#[derive(Deserialize, Debug)]
struct AlbumResult {
    #[serde(rename = "artistName")]
    artist_name: String,
    #[serde(rename = "collectionName")]
    album_name: String,
    #[serde(rename = "artworkUrl100")]
    artwork_url: String,
    #[serde(rename = "releaseDate")]
    release_date: String,
}

pub async fn search_album_metadata(artist: &str, album: &str) -> Option<(String, String)> {
    println!(
        "[iTunes] Searching for album metadata: \"{}\" by \"{}\"",
        album, artist
    );

    let search_term = format!("{} {}", artist, album);
    let url = format!(
        "https://itunes.apple.com/search?term={}&entity=album&limit=5",
        urlencoding::encode(&search_term)
    );
    println!("[iTunes] Sending API request: {}", url);

    let client = reqwest::Client::builder()
        .user_agent("Koda/1.0.0 (https://github.com/s1nn3rv2/koda)")
        .build()
        .ok()?;

    let response: ItunesResponse = client.get(url).send().await.ok()?.json().await.ok()?;

    println!("[iTunes] Stage 1: Checking for exact match...");
    let target_artist_lower = artist.to_lowercase();
    let target_album_lower = album.to_lowercase();

    for result in &response.results {
        let res_artist_lower = result.artist_name.to_lowercase();
        let res_album_lower = result.album_name.to_lowercase();

        if res_artist_lower == target_artist_lower && res_album_lower == target_album_lower {
            let high_res_url = result
                .artwork_url
                .replace(ITUNES_LOW_RES_COVER_SOURCE, ITUNES_HIGH_RES_COVER_TARGET);
            println!(
                "[iTunes] Found strict match: \"{}\" by \"{}\"",
                result.album_name, result.artist_name
            );
            return Some((high_res_url, result.release_date.clone()));
        }
    }

    println!("[iTunes] Stage 1 failed. Stage 2: Checking for fuzzy match...");
    let norm_artist = normalize_string(artist);
    let norm_album = normalize_string(album);

    for result in response.results {
        let res_artist = normalize_string(&result.artist_name);
        let res_album = normalize_string(&result.album_name);

        let artist_match = res_artist.contains(&norm_artist) || norm_artist.contains(&res_artist);
        let album_match = res_album.contains(&norm_album) || norm_album.contains(&res_album);

        if artist_match && album_match {
            let high_res_url = result
                .artwork_url
                .replace(ITUNES_LOW_RES_COVER_SOURCE, ITUNES_HIGH_RES_COVER_TARGET);
            println!(
                "[iTunes] Found fuzzy match: \"{}\" by \"{}\"",
                result.album_name, result.artist_name
            );
            return Some((high_res_url, result.release_date));
        }
    }

    println!(
        "[iTunes] No results found for \"{}\" by \"{}\"",
        album, artist
    );
    None
}

pub async fn search_artist_image(artist: &str) -> Option<String> {
    println!("[iTunes] Searching for artist image: \"{}\"", artist);

    let search_url = format!(
        "https://itunes.apple.com/search?term={}&entity=musicArtist&limit=1",
        urlencoding::encode(artist)
    );
    println!("[iTunes] Sending API request: {}", search_url);

    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0",
        )
        .build()
        .ok()?;

    let search_res: ItunesArtistResponse = client
        .get(search_url)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    let artist_url = &search_res.results.first()?.artist_link_url;
    println!("[iTunes] Found artist URL: {}", artist_url);

    let html = client
        .get(artist_url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    let re = Regex::new(r#"<meta property="og:image" content="([^"]+)""#).ok()?;

    if let Some(caps) = re.captures(&html) {
        if let Some(matched) = caps.get(1) {
            let mut img_url = matched.as_str().to_string();

            let replacement_target =
                format!("/{}orig$1", ITUNES_HIGH_RES_COVER_TARGET.replace("bb", "")); // e.g., /1000x1000orig

            if let Ok(dim_re) = Regex::new(r"/[\d]+x[\d]+[^/]*(\.[a-zA-Z]+)$") {
                img_url = dim_re
                    .replace(&img_url, replacement_target.as_str())
                    .to_string();
            } else if img_url.contains(ITUNES_OG_IMAGE_DIMENSION_SOURCE) {
                // Fallback (though regex should work)
                img_url = img_url.replace(
                    ITUNES_OG_IMAGE_DIMENSION_SOURCE,
                    ITUNES_HIGH_RES_COVER_TARGET,
                );
            }

            println!("[iTunes] Found scraped artwork URL: {}", img_url);
            return Some(img_url);
        }
    }

    println!("[iTunes] No image found in metadata");
    None
}
