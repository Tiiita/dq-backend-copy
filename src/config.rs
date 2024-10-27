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
    pub db: DbCredentials,
}

#[derive(Deserialize)]
pub struct DbCredentials {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub port: i32,
}