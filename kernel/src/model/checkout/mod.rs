use chrono::{DateTime, Utc};

use super::id::{BookId, CheckoutId, UserId};

pub mod event;

pub struct Checkout {
    pub id: CheckoutId,
    pub checkout_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBook,
}

pub struct CheckoutBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}
