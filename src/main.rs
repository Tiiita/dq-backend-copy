use std::{sync::Arc, time::Instant};

use axum::{
    middleware,
    routing::{get, post},
    serve, Extension, Router,
};
use dotenv::dotenv;
use dq_backend::{
    config::{self, Config, DbConfig},
    endpoint::{beta, user},
    jwt, SurrealDb,
};
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    dotenv().expect("Failed to load enviroment file");
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_target(false)
        .init();

    info!("Booting up..");

    debug!("jwt: {}", jwt::gen_token("debugUser".into()).unwrap());
    let config = config::load();

    let db = connect_db(&config.db_cfg).await;

    let listener = TcpListener::bind(&config.server_addr)
        .await
        .expect("Failed to bind listener to address");


    let booting_time = start_time.elapsed().as_millis();
    info!("Listening on {} (took: {} ms)", &config.server_addr, booting_time);

    serve(listener, app(config, db))
        .await
        .expect("Failed to start server");
}
async fn connect_db(db_conf: &DbConfig) -> SurrealDb {
    let db = Surreal::new::<Ws>(&db_conf.addr)
        .await
        .expect("Failed to initalize database");
    db.signin(Root {
        username: &db_conf.username,
        password: &db_conf.password,
    })
    .await
    .expect("Failed to signin into database");

    db.use_ns(&db_conf.namespace)
        .await
        .expect("Failed to select namespace");
    db.use_db(&db_conf.database)
        .await
        .expect("Failed to select database");
    info!("Connection to database established");

    db
}

fn app(config: Config, db: SurrealDb) -> Router {
    let authenticated_router = Router::new()
        .route("/beta/new-key", post(beta::new_key))
        .route("/beta/remove-key", post(beta::remove_key))
        .route("/beta/is-valid", post(beta::is_valid))
        .route("/beta/get-key", get(beta::get_key))
        //.route("/beta/activate-key", post(beta::activate_key))
        .route("/user/auth", post(user::auth_user))
        .route("/user/ban", post(user::ban_user))
        .route("/user/unban", post(user::unban_user))
        .layer(middleware::from_fn(jwt::jwt_middleware));

    let unauthed_router = Router::new()
        .route("/user/register", post(user::register_user))
        .route("/user/login", post(user::login_user));

    Router::merge(authenticated_router, unauthed_router)
        .layer(Extension(Arc::new(config)))
        .layer(Extension(Arc::new(db)))
}
