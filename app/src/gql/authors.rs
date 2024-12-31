use std::{collections::HashMap, hash::Hash};

use async_graphql::dataloader::*;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use axum::async_trait;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct AuthorQL {
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

pub struct AuthorLoader(Pool<Postgres>);

impl AuthorLoader {
    fn new(postgres_pool: Pool<Postgres>) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<String> for AuthorLoader {
    type Value = AuthorQL;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load book by batch {:?}", keys);

        let a = sqlx::query_as("SELECT id, name, author FROM books WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|author: AuthorQL| (author.id.clone(), author))
            .try_collect()
            .await?;

        Ok(a)
    }
}

#[derive(Default)]
pub(crate) struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn authors(&self, ctx: &Context<'_>) -> Result<Vec<AuthorQL>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        let query: Vec<AuthorQL> = sqlx::query("SELECT * FROM authors ORDER BY last_name")
            .map(|row: PgRow| AuthorQL {
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
}
