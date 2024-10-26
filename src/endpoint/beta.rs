use axum::{http::StatusCode, response::IntoResponse, Json};
use log::info;
use rand::{thread_rng, Rng};
use serde::Deserialize;

pub async fn new_key(Json(payload): Json<NewKeyRequest>) -> impl IntoResponse {
    let key = gen_beta_key();
    //Add to db

    info!("'{}' created new beta key: {}", payload.discord_id, key);
    (StatusCode::CREATED, key)
}

pub async fn get_key() -> impl IntoResponse {
    
}

pub async fn remove_key() -> impl IntoResponse {

}

pub async fn is_valid() -> impl IntoResponse {
    
}

#[derive(Deserialize)]
pub struct NewKeyRequest {
    discord_id: i64
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
