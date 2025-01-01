use std::{collections::HashMap, hash::Hash};

use async_graphql::dataloader::*;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use axum::async_trait;
use sqlx::{postgres::PgRow, PgPool, Row};

use super::AppContext;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct Author {
    pub id: String,
    pub last_name: String,
    #[sqlx(default)]
    pub first_name: Option<String>,
    #[sqlx(default)]
    pub country1: Option<String>,
    #[sqlx(default)]
    pub country2: Option<String>,
    #[sqlx(default)]
    pub birthyear: Option<i32>,
    #[sqlx(default)]
    pub pseudonym: Option<String>,
}

pub(crate) struct AuthorLoader(PgPool);

impl AuthorLoader {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<String> for AuthorLoader {
    type Value = Author;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load authors by batch {:?}", keys);

        let author_hash_map = sqlx::query_as("SELECT * FROM authors WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|author: Author| (author.id.clone(), author))
            .try_collect()
            .await?;

        Ok(author_hash_map)
    }
}

#[derive(Default)]
pub(super) struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn authors(&self, ctx: &Context<'_>) -> Result<Vec<Author>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Author> = sqlx::query("SELECT * FROM authors ORDER BY last_name")
            .map(|row: PgRow| Author {
                id: row.get("id"),
                last_name: row.get("last_name"),
                first_name: row.get("first_name"),
                country1: row.get("country1"),
                country2: row.get("country2"),
                birthyear: row.get("birthyear"),
                pseudonym: row.get("pseudonym"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }

    async fn author(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<Author>, async_graphql::Error> {
        let context = &ctx.data_unchecked::<AppContext>().loaders.authors;
        context.load_one(id).await
    }
}
