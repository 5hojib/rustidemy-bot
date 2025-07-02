use crate::web::handlers::{home, not_found};
use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new().route("/", get(home)).fallback(not_found)
}
