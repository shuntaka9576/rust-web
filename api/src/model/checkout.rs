use chrono::{DateTime, Utc};
use kernel::model::{
    checkout::{Checkout, CheckoutBook},
    id::{BookId, CheckoutId, UserId},
};
use serde::Serialize;
use utoipa::ToSchema;

#[cfg_attr(debug_assertions, derive(ToSchema))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutsResponse {
    pub items: Vec<CheckoutResponse>,
}

impl From<Vec<Checkout>> for CheckoutsResponse {
    fn from(value: Vec<Checkout>) -> Self {
        Self {
            items: value
                .into_iter()
                .map(CheckoutResponse::from)
                .collect::<Vec<CheckoutResponse>>(),
        }
    }
}

#[cfg_attr(debug_assertions, derive(ToSchema))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutResponse {
    pub id: CheckoutId,
    pub checkout_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBook,
}

impl From<Checkout> for CheckoutResponse {
    fn from(value: Checkout) -> Self {
        let Checkout {
            id,
            checkout_out_by,
            checked_out_at,
            returned_at,
            book,
        } = value;

        Self {
            id,
            checkout_out_by,
            checked_out_at,
            returned_at,
            book,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(ToSchema))]
pub struct CheckoutBookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutBook> for CheckoutBookResponse {
    fn from(value: CheckoutBook) -> Self {
        let CheckoutBook {
            book_id,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
        }
    }
}
