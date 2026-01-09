//use axum::{Json, extract, extract::Query, response::IntoResponse};
//use chrono::Utc;
//use reqwest::*;
//use serde::{Deserialize, Serialize};
//use serde_xml_rs::from_str;
use std::collections::HashMap;

use rusqlite::{Connection, Result, params};
//use std::error::Error;

#[derive(Debug)]
struct Ratex {
    date: i32,
    target_value: f64,
}

pub async fn last_record(target_code: &str) -> Result<Vec<f64>> {
    println!("...last_record fn");
    let mut v: Vec<f64> = vec![];
    let conn = Connection::open("rate.db").unwrap();

    let mut stmt = conn
        .prepare(&format!(
            "SELECT date, {0} FROM rate ORDER BY date desc LIMIT 1 ",
            target_code
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
        v.push(f64::from(rx.date));
        v.push(rx.target_value);
    }

    Ok(v)
}

pub async fn last_records() -> Result<Vec<f64>> {
    println!("...last_record fn");

    let mut v: Vec<f64> = vec![];
    let conn = Connection::open("rate.db").unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT date,
jpy,
czk,
dkk,
gbp,
huf,
pln,
ron,
sek,
chf,
isk,
nok,
aud,
brl,
cad,
cny,
hkd,
idr,
ils,
inr,
krw,
mxn,
myr,
nzd,
php,
sgd,
thb,
zar,
usd FROM rate ORDER BY date desc LIMIT 1 ",
        )
        .unwrap();

    let c = stmt.column_count().clone();

    let mut row = stmt
        .query_row(params![], |r| {
            let mut vec: Vec<f64> = vec![];
            for i in 0..c {
                let v: f64 = r.get(i).unwrap();
                vec.push(v);
            }
            Ok(vec)
        })
        .unwrap();

    //println!("{:?}", rates);
    Ok(row)
}

pub async fn insert(r: HashMap<String, f64>) -> Result<()> {
    let conn = Connection::open("rate.db").unwrap();

    let k: Vec<String> = r.keys().map(|key| key.clone()).collect();
    let v: Vec<String> = r.values().map(|key| key.to_string()).collect();

    let sql = format!(
        "INSERT INTO rate ({0}) VALUES ('{1}')",
        k.join(","),
        v.join("','")
    );
    let _r = conn.execute(&sql, []);
    Ok(())
}

pub async fn new() -> Result<()> {
    let conn = Connection::open("rate.db").unwrap();
    conn.execute(
        "CREATE TABLE rate (
date INTEGER PRIMARY KEY,
jpy real not null default 0.0,
czk real not null default 0.0,
dkk real not null default 0.0,
gbp real not null default 0.0,
huf real not null default 0.0,
pln real not null default 0.0,
ron real not null default 0.0,
sek real not null default 0.0,
chf real not null default 0.0,
isk real not null default 0.0,
nok real not null default 0.0,
try real not null default 0.0,
aud real not null default 0.0,
brl real not null default 0.0,
cad real not null default 0.0,
cny real not null default 0.0,
hkd real not null default 0.0,
idr real not null default 0.0,
ils real not null default 0.0,
inr real not null default 0.0,
krw real not null default 0.0,
mxn real not null default 0.0,
myr real not null default 0.0,
nzd real not null default 0.0,
php real not null default 0.0,
sgd real not null default 0.0,
thb real not null default 0.0,
zar real not null default 0.0,
rub real not null default 0.0,
usd real not null default 0.0

        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}
