pub mod event;

use serde::{Deserialize, Serialize};
#[cfg(debug_assertions)]
use utoipa::ToSchema;

use super::{id::UserId, role::Role};

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookOwner {
    pub id: UserId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
pub struct CheckoutUser {
    pub id: UserId,
    pub name: String,
}
