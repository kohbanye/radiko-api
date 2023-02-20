use chrono::Local;

#[tokio::test]
async fn test_get_stations() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_sdk::client::Client::new().await?;

    let stations = radiko_sdk::program::get_stations(&client.area_id).await?;

    assert!(!stations[0].id.is_empty());
    assert!(!stations[0].name.is_empty());
    assert!(!stations[0].url.is_empty());

    for station in stations {
        if station.id == "JOAK-FM" {
            assert_eq!(station.name, "NHK-FM（東京）");
            assert_eq!(station.url, "https://www.nhk.or.jp/radio/");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_get_programs_by_date() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_sdk::client::Client::new().await?;

    let stations = radiko_sdk::program::get_stations(&client.area_id).await?;
    let programs = radiko_sdk::program::get_programs_by_date(&stations[0].id, Local::now()).await?;

    assert!(!programs[0].title.is_empty());
    assert!(programs[0].start_at < programs[0].end_at);
    assert!(programs.len() > 10 && !programs[10].title.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_get_program_by_start_time() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_sdk::client::Client::new().await?;

    let stations = radiko_sdk::program::get_stations(&client.area_id).await?;
    let programs = radiko_sdk::program::get_programs_by_date(&stations[0].id, Local::now()).await?;
    let program =
        radiko_sdk::program::get_program_by_start_time(&stations[0].id, programs[0].start_at)
            .await?;

    assert_eq!(program.title, programs[0].title);
    assert_eq!(program.start_at, programs[0].start_at);
    assert_eq!(program.end_at, programs[0].end_at);

    Ok(())
}
