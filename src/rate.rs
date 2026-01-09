use crate::database;
use crate::exchange;
use axum::{Json, extract::Query, response::IntoResponse};
use chrono::Utc;
use database::{insert, last_record, new};
use exchange::get_ecb_rates;
//use rusqlite::{Connection, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
//use std::error::Error;
use gethostname::gethostname;
#[derive(Debug, Serialize)]
struct Rated {
    target_code: String,
    source_code: String,
    source_value: f64,
    target_value: f64,
    target_rate: f64,
    msg: String,
    date: String,
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

pub async fn daily(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    //let y = ecb().await;

    let mut r = Rated {
        target_code: "".to_string(),
        source_code: "".to_string(),
        source_value: 0.0,
        target_value: 0.0,
        target_rate: 0.0,
        date: "".to_string(),
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

        r.target_rate = last[1];
        r.target_value = r.target_rate * r.source_value;
    } else {
        r.msg = format!("Missing Parameters example usage: http://0.0.0.0:8087?s=eur&t=thb&v=2'");
    }
    Json(r)
}

pub async fn currencies(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let mut v: Vec<&str> = vec![];
    let fields = Rate::field_names();
    Json(fields)
}
