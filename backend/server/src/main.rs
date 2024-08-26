mod error;
mod routes;

use crate::routes::login::{get_login_url, login};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
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
    };

    log::info!("Server starting");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new().route("/login", get(get_login_url).post(login));
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
}
