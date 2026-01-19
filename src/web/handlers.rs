use crate::web_cache::WebCache;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_courses(State(cache): State<Arc<Mutex<WebCache>>>) -> impl IntoResponse {
    let cache_locked = cache.lock().await;
    let courses = cache_locked.get_all_courses();
    Json(courses)
}
