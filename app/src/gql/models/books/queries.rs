use super::{book::Book, db::BookDB};
use crate::gql::AppContext;
use crate::utils::db::DBManager;
use async_graphql::{Context, Object};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Default)]
pub struct BookQuery;

#[Object]
impl BookQuery {
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<Book>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Book> = sqlx::query("SELECT * FROM books ORDER BY title")
            .map(|row: PgRow| Book {
                id: row.get("id"),
                title: row.get("title"),
                publisher_id: row.get("publisher_id"),
                npages: row.get("npages"),
                dcr: row.get("dcr"),
                isbn10: row.get("isbn10"),
                isbn13: row.get("isbn13"),
                year: row.get("year"),
                printed_in: row.get("printed_in"),
                is_compilation: row.get("is_compilation"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }

    async fn book(&self, ctx: &Context<'_>, id: Uuid) -> Result<Book, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM books WHERE id = $1").bind(id);

        Ok(BookDB::fetch_one(pool, query).await?)
    }
}
