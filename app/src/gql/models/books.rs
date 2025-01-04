use std::hash::Hash;

use crate::{gql::AppContext, utils::get_bridge_ids};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use super::{humans::Human, texts::Text};

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub publisher_id: String,
    pub npages: i16,
    pub dcr: DateTime<Utc>,
    pub isbn10: Option<i64>,
    pub isbn13: Option<i64>,
    pub year: Option<i16>,
    pub printed_in: String,
    pub is_compilation: bool,
}

#[ComplexObject]
impl Book {
    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Text>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        let query = sqlx::query("SELECT text_id AS bridge FROM book_texts WHERE book_id = $1")
            .bind(&self.id);

        let text_ids: Vec<String> = get_bridge_ids(query, pool).await?;
        if text_ids.is_empty() {
            ()
        }

        let text_loader = &context.loaders.texts;

        Ok(text_loader
            .load_many(text_ids)
            .await?
            .values()
            .cloned()
            .collect())
    }

    async fn editors(&self, ctx: &Context<'_>) -> Result<Vec<Human>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        let query = sqlx::query("SELECT editor_id AS bridge FROM book_editors WHERE book_id = $1")
            .bind(&self.id);

        let editor_ids: Vec<String> = get_bridge_ids(query, pool).await?;
        if editor_ids.is_empty() {
            ()
        }

        let human_loader = &context.loaders.humans;

        Ok(human_loader
            .load_many(editor_ids)
            .await?
            .values()
            .cloned()
            .collect())
    }
}

#[derive(Default)]
pub(crate) struct BookQuery;

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
}
