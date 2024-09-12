use crate::AppState;
use axum::routing::get;
use axum::Router;

mod authenticate;

pub(crate) fn route() -> Router<AppState> {
    Router::new().route(
        "/login",
        get(authenticate::get_login_url).post(authenticate::login),
    )
}
