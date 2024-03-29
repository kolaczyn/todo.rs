use tide::Request;

use crate::{
    auth::application::application::{login_app, register_app},
    common::jwt::Claims,
    state::State,
};

use super::form::{LoginFormDto, RegisterFormDto};

async fn register(mut req: Request<State>) -> tide::Result<String> {
    let body: RegisterFormDto = req.body_json().await?;
    let pool = &req.state().pool;

    let user = register_app(&pool, &body.email, &body.password).await?;
    Ok(serde_json::to_string_pretty(&user)?)
}

async fn login(mut req: Request<State>) -> tide::Result<String> {
    let body: LoginFormDto = req.body_json().await?;
    let pool = &req.state().pool;

    let user = login_app(pool, &body.email, &body.password).await?;
    Ok(serde_json::to_string_pretty(&user)?)
}

async fn me(req: Request<State>) -> tide::Result<String> {
    let claims = req.ext::<Claims>();

    Ok(serde_json::to_string_pretty(&claims)?)
}

pub fn auth_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/login").post(login);
    api.at("/register").post(register);
    api.at("/me").post(me);

    api
}
