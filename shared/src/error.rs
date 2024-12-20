use axum::http::StatusCode;
use axum::response::IntoResponse;
use garde::Report;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(String),
    #[error("トランザクションを実行できませんでした。")]
    TransactionError(#[source] sqlx::Error),
    #[error("データベース処理実行中にエラーが発生しました。")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("No rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error("{0}")]
    KeyValueStoreError(#[from] redis::RedisError),
    #[error("{0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("ログインに失敗しました")]
    UnauthorizedError,
    #[error("認可情報が誤っています")]
    UnauthenticatedErrorError,
    #[error("許可されていない操作です")]
    ForbiddenOperation,
    #[error("{0}")]
    ConversionEntityError(String),
    #[error("Validation failed: {0}")]
    GardeValidationError(String),
}

// NOTE: 独自判断で追加
impl From<Report> for AppError {
    fn from(report: Report) -> Self {
        AppError::GardeValidationError(report.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_)
            | AppError::ConvertToUuidError(_)
            | AppError::GardeValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::UnauthenticatedErrorError | AppError::ForbiddenOperation => {
                StatusCode::FORBIDDEN
            }
            AppError::UnauthorizedError => StatusCode::UNAUTHORIZED,
            e @ (AppError::TransactionError(_)
            | AppError::SpecificOperationError(_)
            | AppError::NoRowsAffectedError(_)
            | AppError::KeyValueStoreError(_)
            | AppError::BcryptError(_)
            | AppError::ConversionEntityError(_)) => {
                tracing::error!(
                error.cause_chain = ?e,
                error.messsage = %e,
                "Unexpected error happend"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
