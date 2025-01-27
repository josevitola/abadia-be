use super::{db::TextDB, Text};
use crate::{gql::AppContext, utils::db::*};
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct TextQuery;

#[Object]
impl TextQuery {
    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM texts ORDER BY id");

        Ok(TextDB::fetch_many(pool, query).await?)
    }

    async fn text_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Text, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM texts WHERE id = $1").bind(id);

        Ok(TextDB::fetch_one(pool, query).await?)
    }

    async fn texts_by_author(
        &self,
        ctx: &Context<'_>,
        author_id: String,
    ) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query =
            sqlx::query("SELECT t.* FROM text_authors ta JOIN texts t ON (ta.text_id = t.id) WHERE ta.author_id = $1")
            .bind(author_id);

        Ok(TextDB::fetch_many(pool, query).await?)
    }

    async fn texts_by_book(
        &self,
        ctx: &Context<'_>,
        book_id: Uuid,
    ) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query =
            sqlx::query("SELECT t.* FROM book_texts bt JOIN texts t ON (bt.text_id = t.id) WHERE bt.book_id = $1")
            .bind(book_id);

        Ok(TextDB::fetch_many(pool, query).await?)
    }

    async fn texts_by_title(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let keyword = format!("%{keyword}%");
        let query = sqlx::query("SELECT * FROM texts WHERE title ILIKE $1").bind(keyword);

        Ok(TextDB::fetch_many(pool, query).await?)
    }
}
