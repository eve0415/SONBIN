mod error;
mod routes;

use axum::http::StatusCode;
use axum::Router;
use game::manager::GameManager;
use oauth::DiscordOAuth;
use redis::Client;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let redis = Client::open(env::var("REDIS_URL").expect("REDIS_URL not set"))
        .expect("Failed to connect to Redis");

    let state = AppState {
        oauth: DiscordOAuth::new(
            env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID not set"),
            env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set"),
            format!(
                "{}/login",
                env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI not set")
            ),
            redis,
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

#[derive(Clone, Debug)]
struct AppState {
    oauth: DiscordOAuth,
    manager: GameManager,
}
