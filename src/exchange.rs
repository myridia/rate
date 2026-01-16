//use axum::{Json, extract, extract::Query, response::IntoResponse};
//use chrono::Utc;
//use reqwest::*;
//use rusqlite::{Connection, Result};
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::collections::HashMap;

use std::error::Error;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Envelope {
    #[serde(rename = "Cube")]
    pub cube_root: CubeRoot,
}

#[derive(Debug, Deserialize)]
struct CubeRoot {
    // This targets the <Cube time="..."> level
    #[serde(rename = "Cube")]
    pub time_entries: Vec<TimeEntry>,
}

#[derive(Debug, Deserialize)]
struct TimeEntry {
    pub time: String,
    // This targets the <Cube currency="..." rate="..."> level
    #[serde(rename = "Cube")]
    pub rates: Vec<RateEntry>,
}

#[derive(Debug, Deserialize)]
struct RateEntry {
    pub currency: String,
    pub rate: f64,
}

pub async fn get_ecb_rates() -> Result<HashMap<String, f64>, Box<dyn Error>> {
    print!("...get_ecb_rates fn");
    let url = format!("http://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml");
    let client = reqwest::Client::new();
    let _res = client
        .get(url)
        .header("User-Agent", "Myridia Rate")
        .send()
        .await;
    let res = _res.unwrap().text().await.unwrap().to_string();

    let mut rates: HashMap<String, f64> = HashMap::new();

    match from_str::<Envelope>(&res) {
        Ok(envelope) => {
            for time_entry in envelope.cube_root.time_entries {
                let date = format!("{0}16", time_entry.time.to_string().replace("-", ""));
                rates.insert("date".to_string(), date.parse().unwrap());
                rates.insert("rub".to_string(), 90.0);
                rates.insert("eur".to_string(), 1.0);
                for r in time_entry.rates {
                    if r.currency.to_lowercase() != "try" {
                        rates.insert(r.currency.to_lowercase(), r.rate);
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    //    println!("{:?}", rates);
    Ok(rates)
}
