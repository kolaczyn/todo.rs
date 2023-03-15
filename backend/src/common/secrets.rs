use std::env;

pub fn get_session_secret() -> String {
    env::var("SESSION_SECRET").expect("SESSION_SECRET should be set")
}
