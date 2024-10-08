use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kernel::model::id::BookId;
use registry::AppRegistry;
use shared::error::AppError;

use crate::model::book::{BookReponse, CreateBookRequest};

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}

pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<BookReponse>>, AppError> {
    registry
        .book_repository
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookReponse::from).collect::<Vec<_>>())
        .map(Json)
        .map_err(AppError::from)
}

pub async fn show_book(
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> Result<Json<BookReponse>, AppError> {
    registry
        .book_repository
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound("not found".into())),
        })
        .map_err(AppError::from)
}
