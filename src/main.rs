use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::get,
};

use libs::help::help;
use libs::rate::{currencies, daily_rate, daily_rates, update_rates};
use libs::test::test;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_origin("https://rate.myridia.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST]);
    let x: u8 = 3;

    let app = Router::new()
        .route("/", get(daily_rate))
        .route("/currencies", get(currencies))
        .route("/daily", get(daily_rates))
        .route("/update", get(update_rates))
        .route("/help", get(help))
        .route("/test", get(move || test(x)))
        .layer(cors)
        .layer(CorsLayer::permissive());

    println!("Server started successfully");
    let host = "127.0.0.1:8087";
    println!("http://{}/test", host);
    println!("http://{}?s=eur&t=thb&v=1", host);

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
