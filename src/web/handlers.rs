use axum::response::Html;

pub async fn home() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn not_found() -> Html<&'static str> {
    Html(include_str!("../../templates/404.html"))
}
