use axum::{Json, response::IntoResponse};

pub async fn help() -> impl IntoResponse {
    // http://127.0.0.1:8889/help
    let r = serde_json::json!([
        {
            "api": "help",
            "example" : "https://rate.myridia.com?s=en&t=de&v=hello"
        }
    ]);
    Json(r)
}
