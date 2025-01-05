use async_graphql::{Context, Object};
use sqlx::{postgres::PgRow, Row};

use crate::gql::AppContext;

use super::Text;

#[derive(Default)]
pub struct TextQuery;

#[Object]
impl TextQuery {
    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let texts: Vec<Text> = sqlx::query("SELECT * FROM texts ORDER BY id")
            .map(|row: PgRow| Text {
                id: row.get("id"),
                title: row.get("title"),
                orig_language_id: row.get("orig_language_id"),
            })
            .fetch_all(pool)
            .await?;

        Ok(texts)
    }

    async fn texts_by_author(
        &self,
        ctx: &Context<'_>,
        author_id: String,
    ) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let texts= 
            sqlx::query("SELECT t.* FROM text_authors ta JOIN texts t ON (ta.text_id = t.id) WHERE ta.author_id = $1")
                .bind(author_id)
                .map(|row: PgRow| Text {
                    id: row.get("id"),
                    title: row.get("title"),
                    orig_language_id: row.get("orig_language_id"),
                })
                .fetch_all(pool)
                .await?;

        Ok(texts)
    }
}