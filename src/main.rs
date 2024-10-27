use env_logger::Builder;
use log::{debug, info, LevelFilter};

use axum::{
    middleware, routing::{get, post}, serve, Router
};
use dq_backend::{endpoint::{beta, user}, jwt};
use tokio::net::TcpListener;

const ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_target(false)
        .init();

    let listener = TcpListener::bind(ADDR)
        .await
        .expect("Failed to bind listener to address");
    info!("Listening on {ADDR}");
    debug!("{}", jwt::gen_token("testid".to_string()).unwrap());
    serve(listener, app())
        .await
        .expect("Failed to start server");
}

fn app() -> Router {
    let authenticated_router = Router::new()
        .route("/beta/new-key", post(beta::new_key))
        .route("/beta/remove-key", post(beta::remove_key))
        .route("/beta/is-valid", post(beta::is_valid))
        .route("/beta/get-key", get(beta::get_key))
        .route("/user/auth", post(user::auth_user))
        .route("/user/ban", post(user::ban_user))
        .route("/user/unban", post(user::unban_user))
        .layer(middleware::from_fn(jwt::jwt_middleware));

    let unauthed_router = Router::new()
        .route("/user/register", post(user::register_user))
        .route("/user/login", post(user::login_user));

    Router::merge(authenticated_router, unauthed_router)
}