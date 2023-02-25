use chrono::{DateTime, Local};

use crate::{
    client::{Client, V2_URL},
    program::get_program_by_start_time,
};

/// Get URLs of .aac files from m3u8 playlist.
pub async fn get_audio_urls(
    client: &Client,
    station_id: &str,
    start_at: DateTime<Local>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let program = get_program_by_start_time(station_id, start_at).await?;

    let url = format!("{}api/ts/playlist.m3u8", V2_URL);
    let req = client.request(reqwest::Method::GET, &url).query(&[
        ("station_id", station_id),
        ("ft", &program.ft),
        ("to", &program.to),
        ("l", "15"),
    ]);
    let res = req.send().await?;
    let body = res.text().await?;

    let bytes = body.as_bytes().to_vec();
    let playlist = m3u8_rs::parse_playlist_res(&bytes).unwrap();

    let master_playlist = match playlist {
        m3u8_rs::Playlist::MasterPlaylist(p) => p,
        _ => return Err("Not a master playlist".into()),
    };

    let m3u8_url = if master_playlist.variants[0].uri.is_empty() {
        return Err("No variants".into());
    } else {
        master_playlist.variants[0].uri.clone()
    };

    let res = reqwest::get(m3u8_url).await?;
    let body = res.text().await?;

    let bytes = body.as_bytes().to_vec();
    let playlist = m3u8_rs::parse_playlist_res(&bytes).unwrap();

    let media_playlist = match playlist {
        m3u8_rs::Playlist::MediaPlaylist(p) => p,
        _ => return Err("Not a media playlist".into()),
    };

    let mut audio_urls = vec![];
    for segment in media_playlist.segments {
        if !segment.uri.is_empty() {
            audio_urls.push(segment.uri);
        }
    }

    Ok(audio_urls)
}
