use tide::Request;

use crate::{
    auth::application::application::{login_app, register_app},
    common::jwt::read_jwt,
    state::State,
};

use super::dto::{LoginFormDto, MeFormDto, RegisterFormDto};

async fn register(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let body: RegisterFormDto = req.body_json().await?;

    let user = register_app(&pool, &body.email, &body.password).await?;
    Ok(serde_json::to_string_pretty(&user)?)
}

async fn login(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let body: LoginFormDto = req.body_json().await?;

    let user = login_app(&pool, &body.email, &body.password).await?;
    Ok(serde_json::to_string_pretty(&user)?)
}

async fn me(mut req: Request<State>) -> tide::Result<String> {
    let body: MeFormDto = req.body_json().await?;
    let jwt = body.jwt;

    let claims = read_jwt(&jwt)?;

    Ok(serde_json::to_string_pretty(&claims)?)
}

pub fn auth_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/login").post(login);
    api.at("/register").post(register);
    api.at("/me").post(me);

    api
}
