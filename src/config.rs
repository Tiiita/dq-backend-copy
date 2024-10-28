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
    #[serde(rename = "serverAddr")]
    pub server_addr: String,
    pub db_config: DbConfig,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}