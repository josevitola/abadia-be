use std::collections::HashMap;

use crate::gql::AppContext;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::{dataloader::Loader, Context, FieldError, Object, SimpleObject};
use axum::async_trait;
use sqlx::{postgres::PgRow, PgPool, Row};

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct Country {
    pub iso3166: String,
    pub name: String,
}

pub(crate) struct CountryLoader(PgPool);

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

        let hash = sqlx::query_as("SELECT * FROM countries WHERE iso3166 = ANY($1)")
            .bind(keys)
            .fetch(&self.0)
            .map_ok(|country: Country| (country.iso3166.clone(), country))
            .try_collect()
            .await?;

        Ok(hash)
    }
}

#[derive(Default)]
pub(crate) struct CountryQuery;

#[Object]
impl CountryQuery {
    async fn countries(&self, ctx: &Context<'_>) -> Result<Vec<Country>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Country> = sqlx::query("SELECT * FROM countries ORDER BY iso3166")
            .map(|row: PgRow| Country {
                iso3166: row.get("iso3166"),
                name: row.get("name"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }

    async fn country(
        &self,
        ctx: &Context<'_>,
        iso3166: String,
    ) -> Result<Option<Country>, async_graphql::Error> {
        let context = &ctx.data_unchecked::<AppContext>().loaders.countries;
        context.load_one(iso3166).await
    }
}

#[derive(Default)]
pub struct CountryMutation;

#[Object]
impl CountryMutation {
    async fn create_country(
        &self,
        ctx: &Context<'_>,
        iso3166: String,
        name: String,
    ) -> Result<u64, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let res = sqlx::query("INSERT INTO countries (iso3166, name) VALUES ($1, $2)")
            .bind(iso3166)
            .bind(name)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }
}
