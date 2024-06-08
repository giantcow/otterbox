use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{routing::get, Router};
use config::{Config, ConfigError, FileFormat};
use tracing::{debug, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = Arc::new(get_config().expect("failed to get config"));
    debug!("Loaded config: {:?}", config);

    let app: Router = Router::new().route("/", get(root));

    let socket_addr = SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        config.get("port").expect("Port must be defined"),
    );
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind socket");

    info!("Listening on {:?}", socket_addr);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    debug!("root");
    "Hello, World!"
}

fn get_config() -> Result<Config, ConfigError> {
    Ok(Config::builder()
        // .set_default(
        //     v1::content_cache::CACHED_CONTENT_DIRECTORY_CONFIG_KEY,
        //     "/tmp/ezv-content-cache".to_string(),
        // )?
        .add_source(config::File::new("ob-server-config.toml", FileFormat::Toml))
        .build()?)
}
