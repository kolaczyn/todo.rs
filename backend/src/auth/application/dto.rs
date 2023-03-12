use serde::Serialize;

#[derive(Serialize)]
pub struct UserDto {
    pub id: i32,
    pub email: String,
    pub jwt: String,
}
