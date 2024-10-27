use std::sync::Arc;

use env_logger::Builder;
use log::{info, LevelFilter};

use axum::{
    middleware, routing::{get, post}, serve, Router
};
use dq_backend::{config::{self, Config}, endpoint::{beta, user}, jwt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_target(false)
        .init();


    let config = config::load();
    let listener = TcpListener::bind(&config.server_addr)
        .await
        .expect("Failed to bind listener to address");
    info!("Listening on {}", &config.server_addr);

    serve(listener, app(config))
        .await
        .expect("Failed to start server");
}

fn app(config: Config) -> Router {
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
    .with_state(Arc::new(config))
}