use crate::sources::utils::normalize_string;
use musicbrainz_rs::FetchCoverart;
use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::entity::artist::Artist as MbArtist;
use musicbrainz_rs::entity::release::Release as MbRelease;
use musicbrainz_rs::prelude::*;

#[derive(Debug, Clone)]
pub struct MbAlbumMetadata {
    pub mbid: String,
    pub release_date: Option<String>,
    #[allow(dead_code)]
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MbArtistMetadata {
    pub mbid: String,
    pub image_url: Option<String>,
}

pub async fn search_album_metadata(
    client: &MusicBrainzClient,
    album_name: &str,
    artist_name: &str,
) -> Option<MbAlbumMetadata> {
    let clean_album = album_name.replace("\"", "").replace(":", "");
    let clean_artist = artist_name.replace("\"", "").replace(":", "");

    println!(
        "[MusicBrainz] Stage 1: Searching for exact match: \"{}\" by \"{}\"",
        album_name, artist_name
    );
    let strict_query = format!(
        "release:\"{}\" AND artist:\"{}\"",
        clean_album, clean_artist
    );

    client.wait_for_ratelimit().await;
    let stage1_json = manual_search("release", &strict_query).await;

    if let Some(json) = stage1_json {
        if let Ok(result) =
            serde_json::from_value::<musicbrainz_rs::entity::search::SearchResult<MbRelease>>(json)
        {
            println!(
                "[MusicBrainz] Stage 1 returned {} entities",
                result.entities.len()
            );
            let target_artist_lower = artist_name.to_lowercase();
            let target_album_lower = album_name.to_lowercase();

            for release in result.entities {
                let res_artist = release
                    .artist_credit
                    .as_ref()
                    .and_then(|ac| ac.first())
                    .map(|credit| credit.artist.name.to_lowercase())
                    .unwrap_or_default();
                let res_album = release.title.to_lowercase();

                if res_artist == target_artist_lower && res_album == target_album_lower {
                    println!(
                        "[MusicBrainz] Found strict match: {} ({})",
                        release.title, release.id
                    );
                    let release_date = release.date.as_ref().map(|d| d.0.clone());
                    let cover_url = fetch_album_cover_by_id(client, &release.id).await;

                    return Some(MbAlbumMetadata {
                        mbid: release.id.clone(),
                        release_date,
                        cover_url,
                    });
                }
            }
        }
    }

    println!("[MusicBrainz] Stage 1 failed. Stage 2: Searching for fuzzy match...");
    let fuzzy_query = format!(
        "release:\"{}\"~ AND artist:\"{}\"~",
        clean_album, clean_artist
    );

    client.wait_for_ratelimit().await;
    let stage2_json = manual_search("release", &fuzzy_query).await;

    if let Some(json) = stage2_json {
        if let Ok(result) =
            serde_json::from_value::<musicbrainz_rs::entity::search::SearchResult<MbRelease>>(json)
        {
            println!(
                "[MusicBrainz] Stage 2 returned {} entities",
                result.entities.len()
            );
            let norm_target_artist = normalize_string(artist_name);
            let norm_target_album = normalize_string(album_name);

            for release in result.entities {
                let res_artist = release
                    .artist_credit
                    .as_ref()
                    .and_then(|ac| ac.first())
                    .map(|credit| normalize_string(&credit.artist.name))
                    .unwrap_or_default();
                let res_album = normalize_string(&release.title);

                let artist_match = res_artist.contains(&norm_target_artist)
                    || norm_target_artist.contains(&res_artist);
                let album_match = res_album.contains(&norm_target_album)
                    || norm_target_album.contains(&res_album);

                if artist_match && album_match {
                    println!(
                        "[MusicBrainz] Found fuzzy match: {} ({})",
                        release.title, release.id
                    );
                    let release_date = release.date.as_ref().map(|d| d.0.clone());
                    let cover_url = fetch_album_cover_by_id(client, &release.id).await;

                    return Some(MbAlbumMetadata {
                        mbid: release.id.clone(),
                        release_date,
                        cover_url,
                    });
                }
            }
        }
    }

    None
}

pub async fn fetch_album_cover_by_id(client: &MusicBrainzClient, mbid: &str) -> Option<String> {
    println!("[MusicBrainz] Fetching cover art for MBID: {}", mbid);

    client.wait_for_ratelimit().await;
    let response = MbRelease::fetch_coverart()
        .id(mbid)
        .front()
        .res_1200()
        .execute_with_client(client)
        .await;

    match response {
        Ok(CoverartResponse::Url(url)) => {
            println!("[MusicBrainz] Found direct 1200px URL: {}", url);
            Some(url)
        }
        Ok(CoverartResponse::Json(ca)) => {
            let found = ca
                .images
                .iter()
                .find(|img| img.front)
                .and_then(|img| {
                    img.thumbnails
                        .res_1200
                        .clone()
                        .or(img.thumbnails.res_500.clone())
                })
                .or_else(|| {
                    ca.images.first().and_then(|img| {
                        img.thumbnails
                            .res_1200
                            .clone()
                            .or(img.thumbnails.res_500.clone())
                    })
                });

            if let Some(ref url) = found {
                println!("[MusicBrainz] Found URL via JSON: {}", url);
            }
            found
        }
        Err(e) => {
            println!(
                "[MusicBrainz] Front/1200 fetch failed: {:?}, trying generic fetch...",
                e
            );
            client.wait_for_ratelimit().await;
            if let Ok(gen_response) = MbRelease::fetch_coverart()
                .id(mbid)
                .execute_with_client(client)
                .await
            {
                match gen_response {
                    CoverartResponse::Json(ca) => {
                        let found = ca
                            .images
                            .iter()
                            .find(|img| img.front)
                            .and_then(|img| {
                                img.thumbnails
                                    .res_1200
                                    .clone()
                                    .or(img.thumbnails.res_500.clone())
                            })
                            .or_else(|| {
                                ca.images.first().and_then(|img| {
                                    img.thumbnails
                                        .res_1200
                                        .clone()
                                        .or(img.thumbnails.res_500.clone())
                                })
                            });

                        if let Some(ref url) = found {
                            println!("[MusicBrainz] Found URL via generic JSON: {}", url);
                        }
                        found
                    }
                    CoverartResponse::Url(url) => {
                        println!("[MusicBrainz] Found generic URL: {}", url);
                        Some(url)
                    }
                }
            } else {
                let fallback = format!("https://coverartarchive.org/release/{}/front-500", mbid);
                println!(
                    "[MusicBrainz] All fetch attempts failed, using fallback: {}",
                    fallback
                );
                Some(fallback)
            }
        }
    }
}

pub async fn search_artist_metadata(
    client: &MusicBrainzClient,
    artist_name: &str,
) -> Option<MbArtistMetadata> {
    println!("[MusicBrainz] Searching for artist: \"{}\"", artist_name);

    let clean_artist = artist_name.replace("\"", "").replace(":", "");

    let strict_query = format!("artist:\"{}\"", clean_artist);
    println!(
        "[MusicBrainz] Sending Stage 1 artist query: {}",
        strict_query
    );

    client.wait_for_ratelimit().await;
    let stage1_json = manual_search("artist", &strict_query).await;

    if let Some(json) = stage1_json {
        if let Ok(result) =
            serde_json::from_value::<musicbrainz_rs::entity::search::SearchResult<MbArtist>>(json)
        {
            let target_lower = artist_name.to_lowercase();
            for artist in result.entities {
                if artist.name.to_lowercase() == target_lower {
                    println!(
                        "[MusicBrainz] Found strict artist match: {} ({})",
                        artist.name, artist.id
                    );
                    return fetch_artist_details(client, artist.id).await;
                }
            }
        }
    }

    println!("[MusicBrainz] Stage 1 failed. Stage 2: Searching for fuzzy artist match...");
    let fuzzy_query = format!("artist:\"{}\"~", clean_artist);
    println!(
        "[MusicBrainz] Sending Stage 2 artist query: {}",
        fuzzy_query
    );

    client.wait_for_ratelimit().await;
    let stage2_json = manual_search("artist", &fuzzy_query).await;

    if let Some(json) = stage2_json {
        if let Ok(result) =
            serde_json::from_value::<musicbrainz_rs::entity::search::SearchResult<MbArtist>>(json)
        {
            let norm_target = normalize_string(artist_name);

            for artist in result.entities {
                let res_name = normalize_string(&artist.name);
                if res_name.contains(&norm_target) || norm_target.contains(&res_name) {
                    println!(
                        "[MusicBrainz] Found fuzzy artist match: {} ({})",
                        artist.name, artist.id
                    );
                    return fetch_artist_details(client, artist.id).await;
                }
            }
        }
    }

    None
}

async fn fetch_artist_details(client: &MusicBrainzClient, id: String) -> Option<MbArtistMetadata> {
    let artist_details = MbArtist::fetch()
        .id(&id)
        .with_url_relations()
        .execute_with_client(client)
        .await
        .ok()?;

    let mut image_url = None;
    if let Some(ref relations) = artist_details.relations {
        for rel in relations {
            if rel.relation_type == "image" || rel.relation_type == "wikimedia commons" {
                match &rel.content {
                    musicbrainz_rs::entity::relations::RelationContent::Url(url) => {
                        println!("[MusicBrainz] Found artist image URL: {}", url.resource);
                        image_url = Some(url.resource.clone());
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    Some(MbArtistMetadata {
        mbid: id,
        image_url,
    })
}

pub async fn download_image(url: &str) -> Option<Vec<u8>> {
    println!("[Network] Downloading image: {}", url);
    let client = reqwest::Client::builder()
        .user_agent("Koda/1.0.0 (https://github.com/s1nn3rv2/koda)")
        .build()
        .ok()?;

    match client.get(url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                let bytes = resp.bytes().await.ok().map(|b| b.to_vec());
                if let Some(ref b) = bytes {
                    println!("[Network] Download successful ({} bytes)", b.len());
                }
                bytes
            } else {
                println!("[Network] Download failed with status: {}", resp.status());
                None
            }
        }
        Err(e) => {
            println!("[Network] Download request failed: {:?}", e);
            None
        }
    }
}

async fn manual_search(entity_type: &str, query: &str) -> Option<serde_json::Value> {
    let url = format!(
        "https://musicbrainz.org/ws/2/{}?query={}&fmt=json",
        entity_type,
        urlencoding::encode(query)
    );
    println!("[MusicBrainz] Manual search request: {}", url);

    let client = reqwest::Client::builder()
        .user_agent("Koda/1.0.0 (https://github.com/s1nn3rv2/koda)")
        .build()
        .ok()?;

    let response = client.get(url).send().await.ok()?;
    if !response.status().is_success() {
        println!(
            "[MusicBrainz] Manual search failed with status: {}",
            response.status()
        );
        return None;
    }

    response.json().await.ok()
}
