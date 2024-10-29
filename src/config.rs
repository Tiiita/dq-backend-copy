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

pub const BETA_KEY_TABLE: &'static str = "beta_keys";
pub const USER_TABLE: &'static str = "users";
pub const VIDEO_TABLE: &'static str = "videos";
pub const QUEST_TABLE: &'static str = "quests";


#[derive(Deserialize)]
pub struct DbConfig {
    //Credentials
    pub addr: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}