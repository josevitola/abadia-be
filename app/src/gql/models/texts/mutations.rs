use async_graphql::{Context, Object, SimpleObject};

use crate::{gql::AppContext, utils::db::*};
use super::db::{CreateTextInput, TextDB};

#[derive(Default, SimpleObject)]
struct CreateTextWithAuthorsResponse {
    text_id: String,
    rows_affected: u64,
}

#[derive(Default)]
pub struct TextMutation;

#[Object]
impl TextMutation {
    async fn create_text(&self, ctx: &Context<'_>, title: String) -> Result<String, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let res = TextDB::insert_one(&mut *tx, CreateTextInput { title }).await;
        tx.commit().await?;

        Ok(res)
    }

    async fn create_text_with_authors(
        &self, 
        ctx: &Context<'_>, 
        title: String,
        author_ids: Vec<String>
    ) -> Result<CreateTextWithAuthorsResponse, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let conn = &mut *tx;

        let new_text_id = TextDB::insert_one(conn, CreateTextInput { title }).await;

        if new_text_id.is_empty() {
            return Err(async_graphql::Error {
                message: "Text could not be created".to_string(),
                source: None,
                extensions: None
            });
        }

        let text_id_col: Vec<String> = vec![new_text_id.clone(); author_ids.len()];

        // following https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts
        let text_authors_insert_res = 
            sqlx::query("INSERT INTO text_authors (text_id, author_id) SELECT * FROM UNNEST($1::text[], $2::text[])")
                .bind(text_id_col)
                .bind(&author_ids)
                .execute(conn)
                .await?;

        let rows_affected = text_authors_insert_res.rows_affected();

        if rows_affected != author_ids.len() as u64 {
            tx.rollback().await?;
            return Err(async_graphql::Error {
                message: "Could not create bridge rows (in text_authors)".into(),
                source: None,
                extensions: None
            })
        }

        tx.commit().await?;

        Ok(CreateTextWithAuthorsResponse {
            rows_affected: text_authors_insert_res.rows_affected(),
            text_id: new_text_id
        })
    }

    async fn add_author_to_texts(
        &self, 
        ctx: &Context<'_>, 
        author_id: String,
        text_ids: Vec<String>
    ) -> Result<u64, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let conn = &mut *tx;

        let author_id_col: Vec<String> = vec![author_id.clone(); text_ids.len()];

        // following https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts
        let text_authors_insert_res = 
            sqlx::query("INSERT INTO text_authors (text_id, author_id) SELECT * FROM UNNEST($1::text[], $2::text[])")
                .bind(&text_ids)
                .bind(author_id_col)
                .execute(conn)
                .await?;

        tx.commit().await?;

        Ok(text_authors_insert_res.rows_affected())
    }
}