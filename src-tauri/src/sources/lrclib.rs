use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LrclibResponse {
    pub id: i64,
    pub track_name: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
    pub duration: f64,
    pub instrumental: bool,
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

#[tauri::command]
pub async fn fetch_lyrics(
    track_name: String,
    artist_name: String,
    album_name: Option<String>,
    duration: Option<f64>,
) -> Result<Option<LrclibResponse>, String> {
    let client = Client::new();
    let mut url = format!(
        "https://lrclib.net/api/get?track_name={}&artist_name={}",
        urlencoding::encode(&track_name),
        urlencoding::encode(&artist_name)
    );

    if let Some(album) = album_name.as_ref() {
        if !album.is_empty() {
            url.push_str(&format!("&album_name={}", urlencoding::encode(album)));
        }
    }

    if let Some(d) = duration {
        url.push_str(&format!("&duration={}", d.round()));
    }

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let data: LrclibResponse = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        return Ok(Some(data));
    } else if response.status() == reqwest::StatusCode::NOT_FOUND {
        let search_url = format!(
            "https://lrclib.net/api/search?q={}",
            urlencoding::encode(&format!("{} {}", track_name, artist_name))
        );
        let search_response = client
            .get(&search_url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if search_response.status().is_success() {
            let results: Vec<LrclibResponse> = search_response
                .json()
                .await
                .map_err(|e| e.to_string())?;

            if !results.is_empty() {
                let target_duration = duration.unwrap_or(0.0);

                let mut best_match = None;
                if target_duration > 0.0 {
                    best_match = results.iter().find(|r| {
                        r.synced_lyrics.is_some() && (r.duration - target_duration).abs() <= 10.0
                    }).cloned();
                }

                if best_match.is_none() {
                    best_match = results.iter().find(|r| r.synced_lyrics.is_some()).cloned();
                }

                return Ok(best_match);
            }
        }
    }

    Ok(None)
}
