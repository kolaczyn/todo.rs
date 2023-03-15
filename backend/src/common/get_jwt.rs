use tide_jwt::jwtsign_secret;

use super::{jwt::Claims, secrets::get_session_secret};

pub fn get_jwt(email: &String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        email: email.clone(),
        id,
        exp: 10000000000,
    };
    jwtsign_secret(&claims, &get_session_secret())
}
