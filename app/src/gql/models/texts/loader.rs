use async_graphql::futures_util::TryStreamExt;
use async_graphql::{dataloader::Loader, FieldError};
use axum::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;

use super::Text;

pub struct TextLoader(PgPool);

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
