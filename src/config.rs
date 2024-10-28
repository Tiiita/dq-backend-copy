use figment::{providers::{Format, Json}, Figment};
use serde::Deserialize;


pub fn load() -> Config {
    let config = Figment::new()
    .merge(Json::file("config.json"));

    let config = config.extract().expect("Failed to load config..");
    config
}

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,

    #[serde(rename = "surrealdb")]
    pub db_cfg: DbConfig,
}

#[derive(Deserialize)]
pub struct DbConfig {
    //Credentials
    pub addr: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,

    //Tables
    pub beta_key_table: String,
    pub user_table: String,
    pub video_table: String,
    pub quest_table: String,
}