use std::{
    fs,
    io::{self, Write},
    process, thread,
};

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

/// Download .aac files concurrently.
pub fn download_files(
    urls: Vec<String>,
    dir: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut handles = vec![];
    let mut filenames = vec![];

    for url in urls {
        let filename = url.split('/').last().unwrap();
        filenames.push(filename.to_string());

        let path = format!("{}/{}", dir, filename);
        let handle = thread::spawn(move || {
            let mut file = fs::File::create(&path).unwrap();
            let mut res = reqwest::blocking::get(&url).unwrap();
            io::copy(&mut res, &mut file).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(filenames)
}

/// Download .aac files and concat them into one file.
pub fn concat_aac_files(
    filenames: Vec<String>,
    dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let input_path = format!("{}/resources", dir);
    let output_path = format!("{}/output.aac", dir);

    let mut input_file = fs::File::create(&input_path)?;
    for filename in filenames {
        let path = format!("{}/{}", dir, filename);
        input_file.write_all(format!("file '{}'\n", path).as_bytes())?;
    }

    process::Command::new("ffmpeg")
        .args(&[
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            &input_path,
            "-c",
            "copy",
            &output_path,
        ])
        .output()
        .expect("failed to execute process");

    Ok(format!("{}/output.aac", dir))
}
