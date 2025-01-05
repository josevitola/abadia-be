use std::collections::HashMap;

use async_graphql::dataloader::*;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use axum::async_trait;
use sqlx::PgPool;

use super::Human;

pub struct HumanLoader(PgPool);

impl HumanLoader {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<String> for HumanLoader {
    type Value = Human;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load humans by batch {:?}", keys);

        let human_hash_map = sqlx::query_as("SELECT * FROM humans WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|human: Human| (human.id.clone(), human))
            .try_collect()
            .await?;

        Ok(human_hash_map)
    }
}
