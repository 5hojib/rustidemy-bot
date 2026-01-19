use crate::{web::handlers::get_courses, web_cache::WebCache};
use axum::{Router, routing::get};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

pub fn create_routes(cache: Arc<Mutex<WebCache>>) -> Router {
    let api_router = Router::new()
        .route("/courses", get(get_courses))
        .with_state(cache);

    Router::new().nest("/api", api_router).fallback_service(
        ServeDir::new("app/dist").not_found_service(ServeFile::new("app/dist/index.html")),
    )
}
