use super::super::languages::{Language, LanguageDB};
use crate::gql::context::AppContext;
use crate::gql::models::humans::Human;
use async_graphql::{ComplexObject, Context, SimpleObject};
use sqlx::{postgres::PgRow, Row};

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Text {
    pub id: String,
    pub title: String,
    pub orig_language_id: String,
}

#[ComplexObject]
impl Text {
    async fn authors(&self, ctx: &Context<'_>) -> Result<Vec<Human>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        struct AuthorId {
            author_id: String,
        }

        let text_authors: Vec<AuthorId> =
            sqlx::query("SELECT author_id FROM text_authors WHERE text_id = $1")
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
        let author_ids: Vec<String> = text_authors
            .into_iter()
            .map(|text_author| text_author.author_id)
            .collect();

        let res = author_loader
            .load_many(author_ids)
            .await?
            .values()
            .cloned()
            .collect();

        Ok(res)
    }

    async fn original_language(&self, ctx: &Context<'_>) -> Result<Language, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query =
            sqlx::query("SELECT * FROM languages WHERE iso693 = $1").bind(&self.orig_language_id);

        Ok(LanguageDB::fetch_one(&pool, query).await?)
    }
}
