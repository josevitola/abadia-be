use crate::gql::AppContext;
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use sqlx::{postgres::PgRow, Row};

use super::humans::Human;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Text {
    pub id: String,
    pub title: String,
}

#[ComplexObject]
impl Text {
    async fn authors(&self, ctx: &Context<'_>) -> Result<Vec<Human>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        struct AuthorId {
            author_id: String,
        }

        let text_authors: Vec<AuthorId> = sqlx::query("SELECT author_id FROM text_authors WHERE text_id = $1")
            .bind(&self.id)
            .map(|row: PgRow| AuthorId {
                author_id: row.get("author_id"),
            })
            .fetch_all(pool)
            .await?;

        if text_authors.is_empty() {
            ()
        }

        let author_loader = &context.loaders.humans;
        let author_ids: Vec<String> = text_authors.into_iter().map(|text_author| text_author.author_id).collect();

        let res = author_loader
            .load_many(author_ids)
            .await?.values().cloned().collect();

        Ok(res)
    }
}


#[derive(Default)]
pub(crate) struct TextQuery;

#[Object]
impl TextQuery {
    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Text>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let texts: Vec<Text> = sqlx::query("SELECT * FROM texts ORDER BY id")
            .map(|row: PgRow| Text {
                id: row.get("id"),
                title: row.get("title"),
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
                })
                .fetch_all(pool)
                .await?;

        Ok(texts)
    }
}
