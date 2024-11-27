use std::future;
use chrono::{Duration, Utc};
use actix_web::{FromRequest, HttpMessage};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, TokenData, Validation};
use serde::{Serialize, Deserialize};
use crate::utils::constants;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub id: i32,
    pub email: String,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            None => future::ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}

pub fn encode_jwt(id: i32, email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::days(1);

    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        id,
        email,
    };

    let secret = (*constants::SECRET_KEY).clone();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = (*constants::SECRET_KEY).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    claim_data
}