use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{
    book::{event::DeleteBook, Book, BookListOptions},
    id::BookId,
    list::PaginatedList,
};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::book::{
        BookListQuery, BookReponse, CreateBookRequest, PaginatedBookResponse, UpdateBookRequest,
        UpdateBookRequestWithIds,
    },
};

pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&());

    registry
        .book_repository
        .create(req.into(), user.user.id)
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    _user: AuthorizedUser,
    Query(query): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<PaginatedBookResponse>> {
    query.validate(&());

    registry
        .book_repository
        .find_all(query.into())
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
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

pub async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&());

    let update_book = UpdateBookRequestWithIds::new(book_id, user.id(), req);

    registry
        .book_repository
        .update(update_book.into())
        .await
        .map(|_| StatusCode::OK)
}

pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let delete_book = DeleteBook {
        book_id,
        requested_user: user.id(),
    };

    registry
        .book_repository
        .delete(delete_book)
        .await
        .map(|_| StatusCode::OK)
}
