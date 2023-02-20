use radiko_sdk::client::Client;

#[tokio::test]
async fn test_auth() -> Result<(), Box<dyn std::error::Error>> {
    let mut client: Client = Default::default();
    client.auth().await?;

    assert!(!client.auth_token.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_get_area() -> Result<(), Box<dyn std::error::Error>> {
    let mut client: Client = Default::default();
    client.set_area_id().await?;

    assert!(client.area_id.contains("JP"));

    Ok(())
}
