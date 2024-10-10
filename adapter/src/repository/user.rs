use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;
use kernel::{
    model::{
        id::UserId,
        role::Role,
        user::{
            event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
            User,
        },
    },
    repository::user::UserRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

pub struct UserRow {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub role_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
        SELECT
            u.user_id,
            u.name,
            u.email,
            r.name as role_name,
            u.created_at,
            u.updated_at
        FROM users AS u
        INNER JOIN roles AS r USING(role_id)
        WHERE u.user_id = $1
            "#,
            current_user_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(User::try_from(r)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            UserRow,
            r#"
        SELECT
            u.user_id,
            u.name,
            u.email,
            u.name as role_name,
            u.created_at,
            u.updated_at
        FROM users AS u
        INNER JOIN roles AS r USING(role_id)
        ORDER BY u.created_at DESC
            "#
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .into_iter()
        .filter_map(|row| User::try_from(row).ok())
        .collect();

        Ok(users)
    }

    async fn create(&self, event: CreateUser) -> AppResult<User> {
        todo!()
    }

    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()> {
        todo!()
    }

    async fn update_role(&self, event: UpdateUserRole) -> AppResult<()> {
        todo!()
    }

    async fn delete(&self, event: DeleteUser) -> AppResult<()> {
        todo!()
    }
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
