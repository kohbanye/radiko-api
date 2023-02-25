use chrono::{Local, TimeZone};

#[tokio::test]
pub async fn test_get_audio_urls() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_sdk::client::Client::new().await.unwrap();

    let station_id = "TBS";
    let start_at =
        Local::datetime_from_str(&Local, "2023-02-20 05:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let urls = radiko_sdk::audio::get_audio_urls(&client, station_id, start_at)
        .await
        .unwrap();

    assert_eq!(urls.len(), 1080);
    for url in urls {
        assert!(url.ends_with(".aac"));
    }

    Ok(())
}
