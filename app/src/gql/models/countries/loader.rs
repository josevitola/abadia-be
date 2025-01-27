use async_graphql::futures_util::TryStreamExt;
use async_graphql::{dataloader::Loader, FieldError};
use axum::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;

use super::Country;

pub struct CountryLoader(PgPool);

impl CountryLoader {
    pub fn new(postgres_pool: PgPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<String> for CountryLoader {
    type Value = Country;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load countries by batch {:?}", keys);

        let hash = sqlx::query_as("SELECT * FROM countries WHERE iso3166_2 = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|country: Country| (country.iso3166_2.clone(), country))
            .try_collect()
            .await?;

        Ok(hash)
    }
}
