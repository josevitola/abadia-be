use crate::gql::AppContext;
use async_graphql::{Context, Object, SimpleObject};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
}

#[derive(Default)]
pub(crate) struct TextQuery;

#[Object]
impl TextQuery {
    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Book>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Book> = sqlx::query("SELECT * FROM texts ORDER BY id")
            .map(|row: PgRow| Book {
                id: row.get("id"),
                title: row.get("title"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }
}
