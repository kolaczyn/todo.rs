use tide::{Next, Request};

use crate::{common::jwt::Claims, state::State};

async fn _require_authorization_middleware<'a>(
    req: Request<State>,
    next: Next<'a, State>,
) -> tide::Result {
    let claims = req.ext::<Claims>();

    if claims.is_none() {
        return Err(tide::Error::from_str(
            tide::StatusCode::Unauthorized,
            "Unauthorized",
        ));
    }

    Ok(next.run(req).await)
}

async fn hello_world(mut _req: Request<State>) -> tide::Result<String> {
    Ok("Hello, world!".to_string())
}

pub fn friends_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    // api.with(require_authorization_middleware);
    api.at("/hello").get(hello_world);

    api
}
