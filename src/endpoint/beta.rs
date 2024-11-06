use std::sync::Arc;

use axum::http::StatusCode;
use axum::Extension;
use axum::{response::IntoResponse, Json};
use log::{error, info, warn};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::config::BETA_KEY_TABLE;
use crate::SurrealDb;
pub async fn new_key(
    Extension(db): Extension<Arc<SurrealDb>>,
    Json(payoad): Json<NewBetaKeyRequest>,
) -> (StatusCode, String) {
    let key_model = BetaKeyModel {
        beta_key: gen_beta_key(),
        used: false,
    };

    let record_id = (BETA_KEY_TABLE, payoad.discord_id);

    match db.select::<Option<BetaKeyModel>>(record_id).await {
        Ok(res) => {
            if res.is_some() {
                return (
                    StatusCode::NOT_ACCEPTABLE,
                    "That user already has a beta key registered".into(),
                );
            }
        }
        Err(why) => {
            error!("Failed checking existence of user: {:?}", why);
            return (StatusCode::INTERNAL_SERVER_ERROR, "".into());
        }
    }

    if let Err(why) = db
        .insert::<Option<BetaKeyModel>>(record_id)
        .content(key_model.clone())
        .await
    {
        error!("Failed to write to db: {:?}", why);
        return (StatusCode::INTERNAL_SERVER_ERROR, "".into());
    }

    info!(
        "{} ({}) -> created new beta key: {}",
        payoad.discord_id, payoad.name, key_model.beta_key
    );

    (StatusCode::CREATED, key_model.beta_key)
}

#[derive(Serialize, Deserialize)]
pub struct NewBetaKeyRequest {
    pub discord_id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BetaKeyModel {
    pub beta_key: String,
    pub used: bool,
}

pub async fn get_key() -> impl IntoResponse {}
pub async fn remove_key() -> impl IntoResponse {}

#[derive(Deserialize)]
pub struct IsValidRequest {
    pub key: String,
}

pub async fn is_valid(
    Extension(db): Extension<Arc<SurrealDb>>,
    Json(payoad): Json<IsValidRequest>,
) -> impl IntoResponse {
    match db.query("SELECT * FROM type::table($table) WHERE beta_key = $key")
    .bind(("table", BETA_KEY_TABLE))
    .bind(("key", payoad.key.clone())).await {
        Ok(mut res) => {
            if let Some(key_model) = res.take::<Option<BetaKeyModel>>(0).unwrap() {
                let status = if key_model.used { StatusCode::IM_USED } else { StatusCode::OK };
                let message = if key_model.used { "Beta key is already in active use" } else { "Code can be used" };

                info!("'{}' got checked. Result -> {message}", payoad.key);
                return (status, message);
            } else {
                warn!("Unknown key '{}' was checked", payoad.key);
                return (StatusCode::NOT_FOUND, "Key not found");
            }
        }
        Err(why) => {
            error!("Failed fetching beta key: {:?}", why);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error retrieving key information");
        }
    }
}

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
