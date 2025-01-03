use std::{collections::HashMap, hash::Hash};

use async_graphql::dataloader::*;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use axum::async_trait;
use sqlx::{postgres::PgRow, PgPool, Row};

use super::{super::AppContext, countries::Country};

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Human {
    pub id: String,
    pub last_name: String,
    #[sqlx(default)]
    pub first_name: Option<String>,
    #[sqlx(default)]
    pub country1_id: Option<String>,
    #[sqlx(default)]
    pub country2_id: Option<String>,
    #[sqlx(default)]
    pub birthyear: Option<i32>,
    #[sqlx(default)]
    pub pseudonym: Option<String>,
}

#[ComplexObject]
impl Human {
    async fn country1(&self, ctx: &Context<'_>) -> Result<Option<Country>> {
        let country_loader = &ctx.data::<AppContext>()?.loaders.countries;

        if self.country1_id.is_some() {
            country_loader
                .load_one(self.country1_id.clone().unwrap())
                .await
        } else {
            Ok(None)
        }
    }

    async fn country2(&self, ctx: &Context<'_>) -> Result<Option<Country>> {
        let country_loader = &ctx.data::<AppContext>()?.loaders.countries;

        if self.country2_id.is_some() {
            country_loader
                .load_one(self.country2_id.clone().unwrap())
                .await
        } else {
            Ok(None)
        }
    }

    async fn name(&self) -> String {
        if let Some(pseudonym) = &self.pseudonym {
            pseudonym.to_string()
        } else {
            let last_name = &self.last_name;
            let first_name_opt = &self.first_name;

            if first_name_opt.is_none() {
                last_name.to_string()
            } else {
                let first_name = first_name_opt.as_ref().unwrap();
                format!("{first_name} {last_name}")
            }
        }
    }
}

pub(crate) struct HumanLoader(PgPool);

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

#[derive(Default)]
pub struct HumanQuery;

#[Object]
impl HumanQuery {
    async fn humans(&self, ctx: &Context<'_>) -> Result<Vec<Human>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Human> = sqlx::query("SELECT * FROM humans ORDER BY last_name")
            .map(|row: PgRow| Human {
                id: row.get("id"),
                last_name: row.get("last_name"),
                first_name: row.get("first_name"),
                country1_id: row.get("country1_id"),
                country2_id: row.get("country2_id"),
                birthyear: row.get("birthyear"),
                pseudonym: row.get("pseudonym"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }

    async fn human(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<Human>, async_graphql::Error> {
        let context = &ctx.data_unchecked::<AppContext>().loaders.humans;
        context.load_one(id).await
    }
}
