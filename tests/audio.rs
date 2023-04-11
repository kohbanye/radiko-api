use chrono::{Local, TimeZone};

#[tokio::test]
pub async fn test_get_audio_urls() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_api::client::Client::new().await.unwrap();

    let station_id = "TBS";
    let start_at =
        Local::datetime_from_str(&Local, "2023-02-20 05:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let urls = radiko_api::audio::get_audio_urls(&client, station_id, start_at)
        .await
        .unwrap();

    assert_eq!(urls.len(), 1080);
    for url in urls {
        assert!(url.ends_with(".aac"));
    }

    Ok(())
}

#[tokio::test]
pub async fn test_concat_aac_files() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_api::client::Client::new().await.unwrap();

    let station_id = "TBS";
    let start_at =
        Local::datetime_from_str(&Local, "2023-02-20 05:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let urls = radiko_api::audio::get_audio_urls(&client, station_id, start_at)
        .await
        .unwrap();

    let dir = "/tmp";
    let filenames = radiko_api::audio::download_files(urls, dir).unwrap();
    let aac_file = radiko_api::audio::concat_aac_files(filenames, dir).unwrap();

    assert!(aac_file.ends_with(".aac"));
    assert!(std::fs::metadata(aac_file).unwrap().len() > 0);

    Ok(())
}
