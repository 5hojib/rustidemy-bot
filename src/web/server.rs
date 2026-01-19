use crate::{
    web::routes::create_routes,
    web_cache::WebCache,
};
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::Mutex;

pub struct ServerConfig {
    pub host: [u8; 4],
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");

        Self {
            host: [0, 0, 0, 0],
            port,
        }
    }

    pub fn to_socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}

pub async fn start_server(cache: Arc<Mutex<WebCache>>) {
    let config = ServerConfig::from_env();
    let app = create_routes(cache);

    let addr = config.to_socket_addr();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to web server address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start web server");
}
