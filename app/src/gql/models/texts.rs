use std::collections::HashMap;

use crate::gql::AppContext;
use async_graphql::dataloader::Loader;
use async_graphql::{ComplexObject, Context,     FieldError, Object, SimpleObject};
use async_graphql::futures_util::TryStreamExt;
use axum::async_trait;
use sqlx::{PgConnection, PgPool};
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


pub(crate) struct TextLoader(PgPool);

impl TextLoader {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<String> for TextLoader {
    type Value = Text;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load texts by batch {:?}", keys);

        let hash = sqlx::query_as("SELECT * FROM texts WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|text: Text| (text.id.clone(), text))
            .try_collect()
            .await?;

        Ok(hash)
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

#[derive(Default, SimpleObject)]
struct CreateTextWithAuthorsResponse {
    text_id: String,
    rows_affected: u64,
}

struct TextDB;

impl TextDB {
    async fn create_text(tx: &mut PgConnection, title: String) -> String {
        let res: Result<String, sqlx::Error> = sqlx::query_scalar("INSERT INTO texts (title) VALUES ($1) RETURNING ID")
                .bind(title)
                .fetch_one(tx)
                .await;

        match res {
            Ok(id) => id,
            Err(_) => String::from("")
        }
    }
}

#[derive(Default)]
pub struct TextMutation;

#[Object]
impl TextMutation {
    async fn create_text(&self, ctx: &Context<'_>, title: String) -> Result<String, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let res = TextDB::create_text(&mut *tx, title).await;
        tx.commit().await?;

        Ok(res)
    }

    async fn create_text_with_authors(
        &self, 
        ctx: &Context<'_>, 
        title: String, author_ids: Vec<String>
    ) -> Result<CreateTextWithAuthorsResponse, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let conn = &mut *tx;

        let new_text_id = TextDB::create_text(conn, title).await;

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

        Ok(CreateTextWithAuthorsResponse {
            rows_affected: text_authors_insert_res.rows_affected(),
            text_id: new_text_id
        })
    }
}