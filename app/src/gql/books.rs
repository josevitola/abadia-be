use std::collections::HashMap;

use async_graphql::{
    dataloader::{DataLoader, Loader},
    futures_util::TryStreamExt,
    Context, FieldError, Object, Result, SimpleObject,
};
use axum::async_trait;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct Book {
    id: i32,
    name: String,
    author: String,
}

pub struct BookLoader(PgPool);

impl BookLoader {
    fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<i32> for BookLoader {
    type Value = Book;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        println!("load book by batch {:?}", keys);

        if keys.contains(&9) {
            return Err("MOCK DBError".into());
        }

        Ok(
            sqlx::query_as("SELECT id, name, author FROM books WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.0)
                .map_ok(|book: Book| (book.id, book))
                .try_collect()
                .await?,
        )
    }
}

pub(crate) struct BookQuery;

#[Object]
impl BookQuery {
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Book>> {
        println!("pre load book by id {:?}", id);
        ctx.data_unchecked::<DataLoader<BookLoader>>()
            .load_one(id)
            .await
    }
}
