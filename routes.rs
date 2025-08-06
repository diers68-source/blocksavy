use axum::{Router, routing::get};
use crate::handlers::{health_check, fetch_prices, get_signals};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/prices", get(fetch_prices))
        .route("/signals", get(get_signals))
}
