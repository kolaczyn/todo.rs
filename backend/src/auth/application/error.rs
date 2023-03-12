use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorAuth {
    #[error("Email or password don't meet the requirements")]
    Validation,
    #[error("The user doesn't exist or the password is invalid")]
    InvalidCredentials,
    #[error("Couldn't create the JWT")]
    JwtCreation,
}
