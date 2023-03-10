use tide::Request;

use crate::state::State;

async fn get_categories(req: Request<State>) -> tide::Result<String> {
    Ok(String::from("Hello world"))
}

pub fn categories_endpoints(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);
    api.at("/").get(get_categories);
    api
}
