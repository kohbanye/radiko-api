use crate::client;

use base64::{engine::general_purpose, Engine as _};
use reqwest::header::HeaderMap;

pub async fn get_auth_token(client: &client::Client) -> Result<String, Box<dyn std::error::Error>> {
    let auth_key = "bcd151073c03b352e1ef2fd66c32209da9ca0afa";

    let auth1_url = "https://radiko.jp/v2/api/auth1";
    let auth2_url = "https://radiko.jp/v2/api/auth2";

    // auth1
    let mut headers = HeaderMap::new();
    headers.insert("X-Radiko-App", "pc_html5".parse().unwrap());
    headers.insert("X-Radiko-App-Version", "0.0.1".parse().unwrap());
    headers.insert("X-Radiko-User", "dummy_user".parse().unwrap());
    headers.insert("X-Radiko-Device", "pc".parse().unwrap());

    let response = client.client.get(auth1_url).headers(headers).send().await?;

    // auth2
    let auth_token = response
        .headers()
        .get("X-Radiko-Authtoken")
        .unwrap()
        .to_str()?;
    let offset = response
        .headers()
        .get("X-Radiko-KeyOffset")
        .unwrap()
        .to_str()?
        .parse::<usize>()?;
    let length = response
        .headers()
        .get("X-Radiko-KeyLength")
        .unwrap()
        .to_str()?
        .parse::<usize>()?;
    let partial_key = general_purpose::STANDARD.encode(&auth_key[offset..offset + length]);

    let mut headers = HeaderMap::new();
    headers.insert("X-Radiko-Authtoken", auth_token.parse().unwrap());
    headers.insert("X-Radiko-Partialkey", partial_key.parse().unwrap());
    headers.insert("X-Radiko-User", "dummy_user".parse().unwrap());
    headers.insert("X-Radiko-Device", "pc".parse().unwrap());

    client.client.get(auth2_url).headers(headers).send().await?;

    Ok(auth_token.to_string())
}
