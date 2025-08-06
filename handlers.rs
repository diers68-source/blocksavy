use axum::{Json, response::IntoResponse};
use serde_json::json;
use crate::services::{data_fetch, signals};

pub async fn health_check() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

pub async fn fetch_prices() -> impl IntoResponse {
    let prices = data_fetch::fetch_bitcoin_prices().await;
    Json(prices)
}

pub async fn get_signals() -> impl IntoResponse {
    let signal = signals::analyze().await;
    Json(signal)
}
