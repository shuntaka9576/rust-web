use async_trait::async_trait;
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

use crate::database::{model::user::UserRow, ConnectionPool};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
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
        let user_id = UserId::new();
        let hashed_password = hashed_password(&event.password)?;
        let role = Role::User;

        let res = sqlx::query!(
            r#"
               INSERT INTO users(user_id, name, email, password_hash, role_id)
               SELECT $1, $2, $3, $4, role_id FROM roles WHERE name = $5;
            "#,
            user_id as _,
            event.name,
            event.email,
            hashed_password,
            role.as_ref()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError);

        todo!();
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

fn hashed_password(password: &str) -> AppResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(AppError::from)
}

fn verify_password(password: &str, hash: &str) -> AppResult<()> {
    let valid = bcrypt::verify(password, hash)?;
    if !valid {
        return Err(AppError::UnauthenticatedErrorError);
    }

    Ok(())
}
