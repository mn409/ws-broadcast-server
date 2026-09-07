use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing");

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing");

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}