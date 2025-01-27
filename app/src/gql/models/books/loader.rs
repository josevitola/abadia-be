use std::collections::HashMap;

use async_graphql::dataloader::*;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use super::Book;

pub struct BookLoader(PgPool);

impl BookLoader {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<Uuid> for BookLoader {
    type Value = Book;
    type Error = FieldError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        println!("load books by batch {:?}", keys);

        let book_hash_map = sqlx::query_as("SELECT * FROM books WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|book: Book| (book.id, book))
            .try_collect()
            .await?;

        Ok(book_hash_map)
    }
}
