use crate::client::{Client, V3_URL};
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename = "station")]
pub struct Station {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "href")]
    pub link: String,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename = "stations")]
struct StationsXML {
    #[serde(rename = "station")]
    stations: Vec<Station>,
}

pub async fn get_stations(client: &Client) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let url = format!("{}station/list/{}.xml", V3_URL, client.area_id);

    let req = client.request(reqwest::Method::GET, &url);
    let res = req.send().await?;
    let body = res.text().await?;

    let stations_xml: StationsXML = quick_xml::de::from_str(&body)?;

    Ok(stations_xml.stations)
}
