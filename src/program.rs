use crate::client::{Client, V3_URL};
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Program {
    pub title: String,
    pub url: String,
    pub ft: String,
    pub to: String,
}

fn parse_stations_xml(xml_str: &str) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let doc = roxmltree::Document::parse(xml_str)?;
    let mut stations = vec![];

    for station in doc.descendants().filter(|n| n.has_tag_name("station")) {
        let get_child_text = |name: &str| -> Option<&str> {
            station
                .children()
                .find(|n| n.has_tag_name(name))
                .and_then(|n| n.text())
        };
        let id = get_child_text("id").unwrap();
        let name = get_child_text("name").unwrap();
        let url = get_child_text("href").unwrap_or("");

        stations.push(Station {
            id: id.to_string(),
            name: name.to_string(),
            url: url.to_string(),
        });
    }

    Ok(stations)
}

pub async fn get_stations(client: &Client) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let url = format!("{}station/list/{}.xml", V3_URL, client.area_id);

    let req = client.request(reqwest::Method::GET, &url);
    let res = req.send().await?;
    let body = res.text().await?;

    let stations = parse_stations_xml(&body)?;

    Ok(stations)
}

fn parse_programs_xml(xml_str: &str) -> Result<Vec<Program>, Box<dyn std::error::Error>> {
    let doc = roxmltree::Document::parse(xml_str)?;
    let mut programs = vec![];

    for program in doc.descendants().filter(|n| n.has_tag_name("prog")) {
        let get_child_text = |name: &str| -> Option<&str> {
            program
                .children()
                .find(|n| n.has_tag_name(name))
                .and_then(|n| n.text())
        };
        let title = get_child_text("title").unwrap();
        let url = get_child_text("url").unwrap_or("");

        programs.push(Program {
            title: title.to_string(),
            url: url.to_string(),
            ft: program.attribute("ft").unwrap().to_string(),
            to: program.attribute("to").unwrap().to_string(),
        });
    }

    Ok(programs)
}

pub async fn get_programs_by_date(
    client: &Client,
    station_id: &str,
    date: DateTime<Local>,
) -> Result<Vec<Program>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}program/station/date/{}/{}.xml",
        V3_URL,
        date.format("%Y%m%d"),
        station_id
    );

    let req = client.request(reqwest::Method::GET, &url);
    let res = req.send().await?;
    let body = res.text().await?;

    let programs = parse_programs_xml(&body)?;

    Ok(programs)
}
