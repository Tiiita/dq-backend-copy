
use axum::body::Body;
use axum::http::Request;
use axum::http::Response;
use axum::http::StatusCode;
use axum::middleware::Next;
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use jsonwebtoken::errors::Error;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use serde::Deserialize;
use serde::Serialize;

const JWT_SECRET: &[u8] = b"45d3d362187eef42f672621e526fe87c3545467dcfd031a3f0d3e175cf01b0b0419d58fa41fa346a8f490b37b4e13af247fca9e3d397383c3af9adfec5365106f6b6357e004e9a89f878cf14d1861c2cde851fc9ebe2789d6ed9c9e8b36b4be9b74414220e31f8104f362055098e6dbad94ac5eaa20d7e22b00103c71f97a9b63e6a9b2477bd59e90e6fe98ee368f2879d18d57fb66264be167df672fcf91dad86777caade01e58e7e8e449ef9e2ac60ec27467e99059da95c99ebca83a4eeb32f7f57188f1e7f8cb2fcb39eebb744fadd1208a36c15d08bb1dced8a2bacc89b7808995a25a19d2de64e028c37b77d02c7a1f9326076ce07c7609668218ebdf2c6d2a8b6a7ddd35ade11fd341667115eb34875f41f3b818696493b314eaba70bf8eb7473ce663ce6be96c09a16a62e5e0808af195ede4083f481f99424fbb0a609b0a8b10b0b044c312030c984f0845e1d7a054075d459b3993cf58fc283187b3b98d633f794600e2e8d43fe43fade801106bb8d72b179c9caf06676661478546b9d2add0b57ccb7e97e18892215857eedcf23caeed10f26f08e8f4d1f8893e76d5adb2e35dd3a46d74ed22fea1d7e81c6cc20f3d2041cab43c92dac5de34dd223f064753edc40a6df40214b444a7ead33d3f0ea15e85e52e1082ccdb40d205089d57c47224c6a214e6f514331cbd4343384c7343e27ef0adb64c5689742a66a";

pub fn gen_token(user_id: String) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_ref());
    let claims = Claims {
        user_id,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now().timestamp() + Duration::days(365 * 200).num_seconds()) as usize,
    };

    encode(&Header::default(), &claims, &encoding_key)
}


pub fn extract_claims(token: &str) -> Result<Claims, Error> {
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )?
    .claims;

    Ok(claims)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: String, //uuid v4
    pub iat: usize,
    pub exp: usize,
}

pub async fn jwt_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let auth_header = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = match extract_claims(auth_header) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}