const AREA_URL: &str = "https://radiko.jp/area";

pub async fn get_area_id() -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get(AREA_URL).await.expect("failed to get area");

    let body = res.text().await.expect("failed to get area");

    let re = regex::Regex::new(r#""(.*?)""#).unwrap();
    let area_id = re
        .captures(&body)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    Ok(area_id)
}
