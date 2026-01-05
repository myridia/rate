use crate::database;
use crate::exchange;
use axum::{Json, extract, extract::Query, response::IntoResponse};
use chrono::Utc;
use database::{last, new};
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

pub async fn daily(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    //let y = ecb().await;

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
    let d = new().await;
    let l = last(&r.target_code).await;
    println!("{:?}", l);
    /*


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

    }

    // let x = process_request().await;
    //println!("{:?}", x.await);
    */
    Json(r)
}
