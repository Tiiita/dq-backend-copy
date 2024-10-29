use axum::Extension;
use axum::{response::IntoResponse, Json};
use axum::http::StatusCode;
use log::info;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::config::{Config, BETA_KEY_TABLE};
use crate::Db;



pub async fn new_key(
    Extension(db): Extension<Db>, 
    Extension(cfg): Extension<Config>, 
    Json(payoad): Json<NewBetaKeyRequest>) 
    -> Result<impl IntoResponse, ()> {

    let key_model = BetaKeyModel {
        beta_key: gen_beta_key(),
    };

    db.insert((BETA_KEY_TABLE, payoad.discord_id)).content(key_model);

    info!("'{}' ({}) -> created new beta key: {}", payoad.discord_id, payoad.name, key_model.beta_key);
    Ok((StatusCode::CREATED, key_model.beta_key))
}

#[derive(Serialize, Deserialize)]
pub struct NewBetaKeyRequest {
    pub discord_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct BetaKeyModel {
    pub beta_key: String,
}

pub async fn get_key() -> impl IntoResponse {}

pub async fn remove_key() -> impl IntoResponse {}

pub async fn is_valid() -> impl IntoResponse {}

pub fn gen_beta_key() -> String {
    const LENGTH: i32 = 12;
    let options: Vec<char> = ('A'..='Z').chain('0'..='9').collect();
    let mut key = String::new();

    for i in 0..LENGTH {
        let random_num = thread_rng().gen_range(0..options.len());
        let char = options[random_num];

        if i % 4 == 0 && i != 0 {
            key.push('-');
        }

        key.push(char);
    }

    key
}
