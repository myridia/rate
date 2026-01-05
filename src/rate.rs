use crate::exchange;
use axum::{Json, extract, extract::Query, response::IntoResponse};
use chrono::Utc;
//use exchange::*;
use exchange::ecb;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
#[derive(Debug, Serialize)]
struct Rated {
    target_code: String,
    source_code: String,
    source_value: f64,
    target_value: f64,
    msg: String,
    date: String,
}

#[derive(Debug)]
struct Rate {
    date: i32,
    JPY: f64,
    CZK: f64,
    DKK: f64,
    GBP: f64,
    HUF: f64,
    PLN: f64,
    RON: f64,
    SEK: f64,
    CHF: f64,
    ISK: f64,
    NOK: f64,
    TRY: f64,
    AUD: f64,
    BRL: f64,
    CAD: f64,
    CNY: f64,
    HKD: f64,
    IDR: f64,
    ILS: f64,
    INR: f64,
    KRW: f64,
    MXN: f64,
    MYR: f64,
    NZD: f64,
    PHP: f64,
    SGD: f64,
    THB: f64,
    ZAR: f64,
}

#[derive(Debug)]
struct Ratex {
    date: i32,
    target_value: f64,
}
pub async fn daily(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let y = ecb().await;

    let mut r = Rated {
        target_code: "".to_string(),
        source_code: "".to_string(),
        source_value: 0.0,
        target_value: 0.0,
        date: "".to_string(),
        msg: "".to_string(),
    };
    if params.contains_key("t") && params.contains_key("s") && params.contains_key("v") {
        r.source_code = params["s"].to_string();
        r.target_code = params["t"].to_string();
        r.source_value = params["v"].parse().unwrap();
    }

    let conn = Connection::open("rate.db").unwrap();

    conn.execute(
        "CREATE TABLE rate (
date INTEGER PRIMARY KEY,
JPY REAL NOT NULL DEFAULT 0.0,
CZK REAL NOT NULL DEFAULT 0.0,
DKK REAL NOT NULL DEFAULT 0.0,
GBP REAL NOT NULL DEFAULT 0.0,
HUF REAL NOT NULL DEFAULT 0.0,
PLN REAL NOT NULL DEFAULT 0.0,
RON REAL NOT NULL DEFAULT 0.0,
SEK REAL NOT NULL DEFAULT 0.0,
CHF REAL NOT NULL DEFAULT 0.0,
ISK REAL NOT NULL DEFAULT 0.0,
NOK REAL NOT NULL DEFAULT 0.0,
TRY REAL NOT NULL DEFAULT 0.0,
AUD REAL NOT NULL DEFAULT 0.0,
BRL REAL NOT NULL DEFAULT 0.0,
CAD REAL NOT NULL DEFAULT 0.0,
CNY REAL NOT NULL DEFAULT 0.0,
HKD REAL NOT NULL DEFAULT 0.0,
IDR REAL NOT NULL DEFAULT 0.0,
ILS REAL NOT NULL DEFAULT 0.0,
INR REAL NOT NULL DEFAULT 0.0,
KRW REAL NOT NULL DEFAULT 0.0,
MXN REAL NOT NULL DEFAULT 0.0,
MYR REAL NOT NULL DEFAULT 0.0,
NZD REAL NOT NULL DEFAULT 0.0,
PHP REAL NOT NULL DEFAULT 0.0,
SGD REAL NOT NULL DEFAULT 0.0,
THB REAL NOT NULL DEFAULT 0.0,
ZAR REAL NOT NULL DEFAULT 0.0,
RUB REAL NOT NULL DEFAULT 0.0
        )",
        (), // empty list of parameters.
    );

    let d = Rate {
        date: 2026010516,
        JPY: "38.88".parse().unwrap(),
        CZK: "38.88".parse().unwrap(),
        DKK: "38.88".parse().unwrap(),
        GBP: "38.88".parse().unwrap(),
        HUF: "38.88".parse().unwrap(),
        PLN: "38.88".parse().unwrap(),
        RON: "38.88".parse().unwrap(),
        SEK: "38.88".parse().unwrap(),
        CHF: "38.88".parse().unwrap(),
        ISK: "38.88".parse().unwrap(),
        NOK: "38.88".parse().unwrap(),
        TRY: "38.88".parse().unwrap(),
        AUD: "38.88".parse().unwrap(),
        BRL: "38.88".parse().unwrap(),
        CAD: "38.88".parse().unwrap(),
        CNY: "38.88".parse().unwrap(),
        HKD: "38.88".parse().unwrap(),
        IDR: "38.88".parse().unwrap(),
        ILS: "38.88".parse().unwrap(),
        INR: "38.88".parse().unwrap(),
        KRW: "38.88".parse().unwrap(),
        MXN: "38.88".parse().unwrap(),
        MYR: "38.88".parse().unwrap(),
        NZD: "38.88".parse().unwrap(),
        PHP: "38.88".parse().unwrap(),
        SGD: "38.99".parse().unwrap(),
        THB: "38.99".parse().unwrap(),
        ZAR: "38.88".parse().unwrap(),
    };
    /*
    conn.execute(
        "INSERT INTO rate (date, THB) VALUES (?1, ?2)",
        (&d.date, &d.THB),
    )
    .unwrap();
     */

    let now = Utc::now();
    let today: i32 = now.format("%Y%m%d00").to_string().parse().unwrap();

    let mut stmt = conn
        .prepare(&format!(
            "SELECT date, {0} FROM rate ORDER BY date desc LIMIT 1 ",
            r.target_code
        ))
        .unwrap();
    let rate_iter = stmt
        .query_map([], |row| {
            Ok(Ratex {
                date: row.get(0)?,
                target_value: row.get(1)?,
            })
        })
        .unwrap();

    for rate in rate_iter {
        let rx = rate.unwrap();
        r.target_value = rx.target_value * r.source_value;
        if today < rx.date {

            //let rate = ecb().await;
            //println!("date: {0} | rate: {1}", rx.date, rx.target_value);

            //let url = format!("http://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml");
            //let response = reqwest::get(url).await;
            //let client = reqwest::Client::new();

            //println!("...get request to qb {url}");
            //let res = client.get(url).header("User-Agent", "Sapir").send().await;
            //let r = res.unwrap().text().await.unwrap().to_string();
            //let inv: Invoice = serde_json::from_str(&r).unwrap();
            ////println!("{:?}", inv);

            //for i in inv.InvoiceQueryRs.InvoiceRet.iter() {
            //    let mut row: Row<String> = Default::default();
            //    row.number = i.RefNumber.parse().unwrap();
            //}
        }
    }

    // let x = process_request().await;
    //println!("{:?}", x.await);

    Json(r)
}
