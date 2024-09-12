use crate::AppState;
use axum::routing::post;
use axum::Router;

mod create;

pub(crate) fn route() -> Router<AppState> {
    Router::new().route("/new", post(create::new_game))
}
