use crate::web::handlers::{home, not_found};
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/", get(home)).fallback(not_found)
}
