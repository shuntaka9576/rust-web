use chrono::{DateTime, Utc};
use kernel::model::{id::UserId, role::Role, user::User};
use shared::error::AppError;
use std::str::FromStr; // これがないとroleでエラーになる

#[derive(Debug)]
pub struct UserRow {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub role_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = AppError;
    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        let UserRow {
            user_id,
            name,
            email,
            role_name,
            created_at,
            updated_at,
        } = value;

        Ok(User {
            id: user_id,
            name,
            email,
            role: Role::from_str(role_name.as_str())
                .map_err(|e| AppError::ConversionEntityError(e.to_string()))?,
        })
    }
}
