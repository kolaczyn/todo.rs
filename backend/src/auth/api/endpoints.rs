use tide::Request;

use crate::{auth::repository::repository::register_db, state::State};

async fn login(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let username = String::from("John");
    let password = String::from("123456");

    let user = register_db(&pool, username, password).await?;

    Ok(serde_json::to_string_pretty(&user)?)
}

async fn register(req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();
    let username = String::from("John");
    let password = String::from("123456");

    let user = register_db(&pool, username, password).await?;

    Ok(serde_json::to_string_pretty(&user)?)
}

pub fn auth_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/login").post(login);
    api.at("/register").post(register);

    api
}
