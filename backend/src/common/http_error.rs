use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("401 Unathorized")]
    Unauthorized,
}
