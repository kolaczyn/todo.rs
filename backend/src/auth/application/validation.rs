// TODO these should return result instead of bool, and should be called something like "validate_password"
pub fn is_password_valid(password: &String) -> bool {
    password.len() >= 6
}

pub fn is_email_valid(email: &String) -> bool {
    // TODO add better validation :p
    email.contains('@')
}

pub fn are_credentials_valid(email: &String, password: &String) -> bool {
    is_email_valid(email) && is_password_valid(password)
}
