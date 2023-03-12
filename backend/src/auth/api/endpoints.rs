use tide::Request;

use crate::{
    auth::repository::repository::{login_db, register_db},
    common::jwt::{create_jwt, read_jwt},
    state::State,
};

use super::dto::{LoginFormDto, MeFormDto, RegisterFormDto, UserDto};

async fn register(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();

    let body: RegisterFormDto = req.body_json().await?;
    // TODO add validation (is email and password length)
    let email = body.email;
    let password = body.password;

    let user_db = register_db(&pool, email, password).await?;
    let jwt = create_jwt(user_db.id, &user_db.email)?;
    let user_dto = UserDto {
        email: user_db.email.clone(),
        id: user_db.id,
        jwt,
    };

    Ok(serde_json::to_string_pretty(&user_dto)?)
}

async fn login(mut req: Request<State>) -> tide::Result<String> {
    let pool = req.state().pool.clone();

    let body: LoginFormDto = req.body_json().await?;
    // TODO add validation (is email and password length)
    let username = body.email;
    let password = body.password;

    let user_db = login_db(&pool, username, password).await?;
    let jwt = create_jwt(user_db.id, &user_db.email)?;
    let user_dto = UserDto {
        email: user_db.email.clone(),
        id: user_db.id,
        jwt,
    };

    Ok(serde_json::to_string_pretty(&user_dto)?)
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
