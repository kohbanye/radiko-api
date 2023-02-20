#[tokio::test]
async fn test_get_stations() -> Result<(), Box<dyn std::error::Error>> {
    let client = radiko_sdk::client::Client::new().await?;

    let stations = radiko_sdk::program::get_stations(&client).await?;

    assert!(!stations[0].id.is_empty());
    assert!(!stations[0].name.is_empty());
    assert!(!stations[0].link.is_empty());

    for station in stations {
        if station.id == "JOAK-FM" {
            assert_eq!(station.name, "NHK-FM（東京）");
        }
    }

    Ok(())
}