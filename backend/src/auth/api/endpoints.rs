use tide::Request;

use crate::{
    auth::repository::repository::{login_db, register_db},
    state::State,
};

use super::dto::{LoginFormDto, RegisterFormDto};

async fn register(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();

    let body: RegisterFormDto = req.body_json().await?;
    // TODO add validation (is email and password length)
    let email = body.email;
    let password = body.password;

    let user = register_db(&pool, email, password).await?;

    Ok(serde_json::to_string_pretty(&user)?)
}

async fn login(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();

    let body: LoginFormDto = req.body_json().await?;
    // TODO add validation (is email and password length)
    let username = body.email;
    let password = body.password;

    let user = login_db(&pool, username, password).await?;

    Ok(serde_json::to_string_pretty(&user)?)
}

pub fn auth_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/login").post(login);
    api.at("/register").post(register);

    api
}
