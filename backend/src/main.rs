use std::env;

use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, Validation};
use tide::security::CorsMiddleware;
use tide_jwt::JwtAuthenticationDecoder;

use crate::{
    auth::api::endpoints::auth_endpoints,
    categories::api::endpoints::categories_endpoints,
    common::{jwt::Claims, secrets::get_session_secret},
    state::State,
    todos::api::endpoints::todo_endpoints,
};

mod auth;
mod categories;
mod common;
mod state;
mod todos;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv()?;

    let state = State::new().await?;
    let mut app = tide::new();

    let jwt_secret = get_session_secret();
    app.with(JwtAuthenticationDecoder::<Claims>::new(
        Validation::default(),
        DecodingKey::from_base64_secret(&jwt_secret)?,
    ));

    app.with(CorsMiddleware::new().allow_origin("*"));

    tide::log::start();

    app.at("v1/auth").nest(auth_endpoints(state.clone()));
    app.at("/v1/todos").nest(todo_endpoints(state.clone()));
    app.at("/v1/categories").nest(categories_endpoints(state));

    let host = String::from("0.0.0.0");
    let port: u16 = env::var("PORT")?.parse()?;

    app.listen((host, port)).await?;
    Ok(())
}
