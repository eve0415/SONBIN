mod error;
mod routes;

use axum::http::StatusCode;
use axum::Router;
use game::manager::GameManager;
use oauth::security::SecurityManager;
use oauth::DiscordOAuth;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let oauth_security: Arc<Mutex<dyn SecurityManager>> = match env::var("REDIS_URL") {
        Ok(str) => Arc::new(Mutex::new(oauth::security::redis::RedisSecurityManager {
            redis_client: redis::Client::open(str).expect("Failed to connect to Redis"),
        })),
        Err(_) => Arc::new(Mutex::new(
            oauth::security::memory::InMemorySecurityManager::default(),
        )),
    };

    let state = AppState {
        oauth: *DiscordOAuth::new(
            env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID not set"),
            env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set"),
            format!(
                "{}/login",
                env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI not set")
            ),
            oauth_security,
        ),
        manager: GameManager::new(),
    };

    log::info!("Server starting");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .merge(routes::login::route())
        .nest("/game", routes::game::route());

    axum::serve(
        listener,
        Router::new()
            .nest("/api", router)
            .fallback(|| async { StatusCode::NOT_FOUND })
            .with_state(state),
    )
    .await
    .unwrap();
}

#[derive(Clone)]
struct AppState {
    oauth: DiscordOAuth,
    manager: GameManager,
}
