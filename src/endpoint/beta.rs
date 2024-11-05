use std::sync::Arc;

use axum::http::StatusCode;
use axum::Extension;
use axum::{response::IntoResponse, Json};
use log::{error, info};
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
    };

    let record_id = (BETA_KEY_TABLE, payoad.discord_id);

    match db.select::<Option<BetaKeyModel>>(record_id).await {
        Ok(res) => {
            if res.is_some() {
                return (StatusCode::NOT_ACCEPTABLE, "That user already has a beta key registered".into());
            }
        },
        Err(why) => {
            error!("Failed checking existence of user: {:?}", why);
           return (StatusCode::INTERNAL_SERVER_ERROR, "".into());
        },
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

#[derive(Deserialize)]
pub struct NewBetaKeyResponse {
    pub message: String,
    pub key: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
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
