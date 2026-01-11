use crate::database;
use crate::exchange;
use axum::{Json, extract::Query, response::IntoResponse};
use chrono::{Datelike, Utc};
use database::{insert, last_record, last_records, new};
use exchange::get_ecb_rates;
//use rusqlite::{Connection, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
//use std::error::Error;
use gethostname::gethostname;

#[derive(Debug, Serialize, Default)]
struct Update {
    ok: bool,
    msg: String,
}

#[derive(Debug, Serialize)]
struct Rated {
    target_code: String,
    source_code: String,
    source_value: f64,
    target_value: f64,
    target_rate: f64,
    msg: String,
    date: i32,
}

#[derive(Debug, Default, Serialize)]
pub struct Rate {
    date: i32,
    jpy: f64,
    czk: f64,
    dkk: f64,
    gbp: f64,
    huf: f64,
    pln: f64,
    ron: f64,
    sek: f64,
    chf: f64,
    isk: f64,
    nok: f64,
    aud: f64,
    brl: f64,
    cad: f64,
    cny: f64,
    hkd: f64,
    idr: f64,
    ils: f64,
    inr: f64,
    krw: f64,
    mxn: f64,
    myr: f64,
    nzd: f64,
    php: f64,
    sgd: f64,
    thb: f64,
    zar: f64,
    usd: f64,
}

impl Rate {
    // Method to return field names as Vec<String>
    fn field_names() -> Vec<String> {
        let json_value: Value = serde_json::to_value(Self::default()).unwrap();
        //let json_value: Value = serde_json::to_value(Self::default()).unwrap();
        json_value
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.to_string())
            .collect()
    }
}

pub async fn daily_rate(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    //let y = ecb().await;

    let mut r = Rated {
        target_code: "".to_string(),
        source_code: "".to_string(),
        source_value: 0.0,
        target_value: 0.0,
        target_rate: 0.0,
        date: 0,
        msg: "".to_string(),
    };
    if params.contains_key("t") && params.contains_key("s") && params.contains_key("v") {
        r.source_code = params["s"].to_string();
        r.target_code = params["t"].to_string();
        r.source_value = params["v"].parse().unwrap();

        let _d = new().await;
        let mut last = last_record(&r.target_code).await.unwrap();

        if !last.is_empty().clone() {
            let now = Utc::now();
            let today: i32 = now.format("%Y%m%d16").to_string().parse().unwrap();
            let date: i32 = last[0] as i32;
            println!(
                "today: {0} | record: {1} | diff: {2}",
                today,
                date,
                today - date
            );
            if today - date >= 300 {
                let rates = get_ecb_rates().await.unwrap();
                let _l = insert(rates).await;
                last = last_record(&r.target_code).await.unwrap();
            }
        } else {
            let rates = get_ecb_rates().await.unwrap();
            let _l = insert(rates).await;
            last = last_record(&r.target_code).await.unwrap();
        }

        r.date = last[0] as i32;
        r.target_rate = last[1];
        r.target_value = r.target_rate * r.source_value;
    } else {
        //let x = gethostname().into_string();
        //println!("{:?}", x);
        r.msg = format!("Missing Parameters!  example usage: http://0.0.0.0:8087?s=eur&t=thb&v=2'");
    }
    Json(r)
}

pub async fn currencies() -> impl IntoResponse {
    let fields = Rate::field_names();
    Json(fields)
}

pub async fn daily_rates() -> impl IntoResponse {
    //let y = ecb().await;

    let mut rates = Rate::default();

    let _d = new().await;
    let row = last_records().await.unwrap();

    rates.date = row[0] as i32;
    rates.jpy = row[1];
    rates.czk = row[2];
    rates.dkk = row[3];
    rates.gbp = row[4];
    rates.huf = row[5];
    rates.pln = row[6];
    rates.ron = row[7];
    rates.sek = row[8];
    rates.chf = row[9];
    rates.isk = row[10];
    rates.nok = row[11];
    rates.aud = row[12];
    rates.brl = row[13];
    rates.cad = row[14];
    rates.cny = row[15];
    rates.hkd = row[16];
    rates.idr = row[17];
    rates.ils = row[18];
    rates.inr = row[19];
    rates.krw = row[20];
    rates.mxn = row[21];
    rates.myr = row[22];
    rates.nzd = row[23];
    rates.php = row[24];
    rates.sgd = row[25];
    rates.thb = row[26];
    rates.zar = row[27];
    rates.usd = row[28];
    //    println!("{:?}", rates);
    Json(rates)
}

pub async fn update_rates() -> impl IntoResponse {
    //let y = ecb().await;
    let mut u = Update::default();
    let _d = new().await;
    let last = last_records().await.unwrap();
    let now = Utc::now();
    let today: i32 = now.format("%Y%m%d16").to_string().parse().unwrap();
    let date: i32 = last[0] as i32;
    let weekday = &now.weekday().to_string();
    if weekday != "Sun" && weekday != "Sat" {
        if today != date {
            println!(
                "today: {0} | record: {1} | diff: {2} | weekday: {3} ",
                today,
                date,
                today - date,
                weekday
            );
            let rates = get_ecb_rates().await.unwrap();
            let _l = insert(rates).await;
            u.ok = true;
        }
    }
    Json(u)
}
