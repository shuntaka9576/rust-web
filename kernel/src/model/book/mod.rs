use serde::Serialize;

use super::{
    id::{BookId, UserId},
    user::BookOwner,
};

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
}

pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}
