use std::collections::HashMap;

use async_graphql::dataloader::*;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use axum::async_trait;
use sqlx::{postgres::PgRow, PgPool, Row};

mod human;

pub use human::Human;

use super::super::AppContext;
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

#[derive(InputObject)]
struct CreateHumanInput {
    last_name: String,
    first_name: Option<String>,
    country1_id: Option<String>,
    country2_id: Option<String>,
    birthyear: Option<i32>,
    pseudonym: Option<String>
}

#[derive(Default)]
pub struct HumanMutation; // glorious evolution!

#[Object]
impl HumanMutation {
    async fn create_human(
        &self,
        ctx: &Context<'_>,
        input: CreateHumanInput
    ) -> Result<u64, async_graphql::Error> {
            let pool = &ctx.data::<AppContext>()?.pool;

            let CreateHumanInput {
                birthyear,
                country1_id,
                country2_id,
                first_name,
                last_name,
                pseudonym
            } = input;

            let res = 
                sqlx::query("INSERT INTO humans (last_name, first_name, country1_id, country2_id, birthyear, pseudonym) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(last_name)
                    .bind(first_name)
                    .bind(country1_id)
                    .bind(country2_id)
                    .bind(birthyear)
                    .bind(pseudonym)
                    .execute(pool)
                    .await?;

            Ok(res.rows_affected())
        }
}