
use std::sync::Arc;

use axum::{middleware, routing::{get, post}, serve, Router};
use env_logger::Builder;
use log::{info, LevelFilter};
use dq_backend::{config::{self, Config, DbConfig}, endpoint::{beta, user}, jwt};
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Error, Surreal};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_target(false)
        .init();

    let config = config::load();

    let db = connect_db(&config.db_config).await.expect("Failed to establish database connection");

    let listener = TcpListener::bind(&config.server_addr)
        .await
        .expect("Failed to bind listener to address");
    info!("Listening on {}", &config.server_addr);

    serve(listener, app(config, db))
        .await
        .expect("Failed to start server");
}

async fn connect_db(db_conf: &DbConfig) -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>(&db_conf.addr).await?;
    db.signin(Root {
        username: &db_conf.username,
        password: &db_conf.password,
    }).await.expect("Failed to signin into database");

    db.use_ns(&db_conf.namespace).await.expect("Failed to select namespace");
    db.use_db(&db_conf.database).await.expect("Failed to select database");
    info!("Connected to database");

    Ok(db)
}

fn app(config: Config, db: Surreal<Client>) -> Router {
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
    .with_state(Arc::new(db))
}