use chrono::{DateTime, Utc};
use serde::Serialize;

use super::id::{BookId, CheckoutId, UserId};

pub mod event;

#[derive(Debug)]
pub struct Checkout {
    pub id: CheckoutId,
    pub checkout_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBook,
}

#[derive(Serialize, Debug)]
pub struct CheckoutBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}
