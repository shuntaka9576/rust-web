use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        auth::event,
        checkout::{
            event::{CreateCheckout, UpdateReturned},
            Checkout,
        },
        id::{BookId, CheckoutId, UserId},
    },
    repository::checkout::CheckoutRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{model::checkout::CheckoutStateRow, ConnectionPool};

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}

impl CheckoutRepositoryImpl {
    async fn set_transaction_serializable(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> AppResult<()> {
        sqlx::query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut **tx)
            .await
            .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        self.set_transaction_serializable(&mut tx).await?;

        {
            // 指定の蔵書IDを持つ蔵書が存在するか
            // 存在した場合、この蔵書は貸出中ではないか
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
            SELECT
                b.book_id,
                c.checkout_id AS "checkout_id?: CheckoutId",
                NULL AS "user_id?: UserId"
            FROM books AS b
            LEFT OUTER JOIN checkouts AS c USING(book_id)
            WHERE book_id = $1;
                "#,
                event.book_id as _
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::SpecificOperationError)?;

            match res {
                None => {
                    return Err(AppError::EntityNotFound(format!(
                        "書籍({})が見つかりませんでした。",
                        event.book_id
                    )))
                }
                Some(CheckoutStateRow {
                    checkout_id: Some(_),
                    book_id,
                    user_id,
                }) => {
                    return Err(AppError::UnprocessableEntity(format!(
                        "書籍({})に対する貸出がすでに存在します。",
                        event.book_id
                    )))
                }
                _ => {}
            }
        }

        let checkout_id = CheckoutId::new();
        let res = sqlx::query!(
            r#"
              INSERT INTO checkouts
              (checkout_id, book_id, user_id, checked_out_at)
              VALUES ($1, $2, $3, $4)
              ;
            "#,
            checkout_id as _,
            event.book_id as _,
            event.checked_out_by as _,
            event.checked_out_at as _,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No returning record has been updated".into(),
            ));
        }

        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        todo!()
    }

    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>> {
        todo!()
    }

    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {
        todo!()
    }

    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>> {
        todo!()
    }
}
