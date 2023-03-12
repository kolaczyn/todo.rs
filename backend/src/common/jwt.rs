use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    email: String,
    id: i32,
    exp: usize,
}

// FIXME should read jwt secret from env :p
const JWT_SECRET: &[u8] = b"lol123";

#[derive(Error, Debug)]
pub enum ErrorCreateJwt {
    #[error("jwt token creation error")]
    JwtTokenCreationError,
}

pub fn create_jwt(id: i32, email: &String) -> Result<String, ErrorCreateJwt> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        email: String::from(email),
        id,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let jwt = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ErrorCreateJwt::JwtTokenCreationError)?;
    Ok(jwt)
}

pub fn read_jwt(jwt: &String) -> Result<Claims, ErrorCreateJwt> {
    let claims = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| ErrorCreateJwt::JwtTokenCreationError)?
    .claims;
    Ok(claims)
}
