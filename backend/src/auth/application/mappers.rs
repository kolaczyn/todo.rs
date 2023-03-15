use crate::auth::repository::models::UserDb;

use super::dto::UserDto;

impl UserDb {
    pub fn to_dto(self, jwt: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            jwt,
        }
    }
}
