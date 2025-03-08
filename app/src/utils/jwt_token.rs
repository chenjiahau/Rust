use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::utils::constants;
use crate::models::unauth_models::TokenModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize, // expiration time
    pub iat: usize, // start time
    pub id: Uuid,
    pub email: String,
}

pub fn encode_jwt(id: Uuid, email: String) -> Result<TokenModel, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::days(1);

    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        id,
        email,
    };

    let secret = (*constants::SECRET_KEY).clone();
    let jsonwebtoken = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );

    if jsonwebtoken.is_err() {
        return Err(jsonwebtoken.err().unwrap());
    }

    let token = jsonwebtoken.unwrap();
    let token_model = TokenModel {
        token,
        expiration_time: claims.exp,
        start_time: claims.iat,
    };

    Ok(token_model)
}