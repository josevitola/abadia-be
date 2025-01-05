use async_graphql::{Context, Object};
use sqlx::{postgres::PgRow, Row};

use super::{AppContext, Human};

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

    async fn humans_by_name(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<Human>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let keyword = format!("%{keyword}%");

        let query: Vec<Human> = sqlx::query(
            "
            SELECT * FROM public.humans
            WHERE last_name ILIKE $1 OR first_name ILIKE $1 OR pseudonym ILIKE $1
            ORDER BY id ASC 
        ",
        )
        .bind(keyword)
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
}
