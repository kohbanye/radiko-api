use crate::client::V3_URL;
use chrono::{DateTime, Local, TimeZone};

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
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
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

pub async fn get_stations(area_id: &str) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let url = format!("{}station/list/{}.xml", V3_URL, area_id);

    let res = reqwest::get(url).await?;
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

        let ft = program.attribute("ft").unwrap();
        let to = program.attribute("to").unwrap();
        let fmt = "%Y%m%d%H%M%S";

        programs.push(Program {
            title: title.to_string(),
            url: url.to_string(),
            start_at: Local.datetime_from_str(ft, fmt)?,
            end_at: Local.datetime_from_str(to, fmt)?,
            ft: program.attribute("ft").unwrap().to_string(),
            to: program.attribute("to").unwrap().to_string(),
        });
    }

    Ok(programs)
}

/// Get all programs of the date
pub async fn get_programs_by_date(
    station_id: &str,
    date: DateTime<Local>,
) -> Result<Vec<Program>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}program/station/date/{}/{}.xml",
        V3_URL,
        date.format("%Y%m%d"),
        station_id
    );

    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    let programs = parse_programs_xml(&body)?;

    Ok(programs)
}

/// Get a program by start time
pub async fn get_program_by_start_time(
    station_id: &str,
    start_at: DateTime<Local>,
) -> Result<Program, Box<dyn std::error::Error>> {
    let programs = get_programs_by_date(station_id, start_at).await?;
    let program = programs
        .into_iter()
        .find(|p| p.start_at == start_at)
        .ok_or("Program not found")?;

    Ok(program)
}
