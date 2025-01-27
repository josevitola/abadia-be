use super::{AppContext, Country};
use async_graphql::{Context, Object};
use sqlx::{postgres::PgRow, Row};

#[derive(Default)]
pub struct CountryQuery;

#[Object]
impl CountryQuery {
    async fn countries(&self, ctx: &Context<'_>) -> Result<Vec<Country>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Country> = sqlx::query("SELECT * FROM countries ORDER BY name")
            .map(|row: PgRow| Country {
                iso3166_2: row.get("iso3166_2"),
                name: row.get("name"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }

    async fn country(
        &self,
        ctx: &Context<'_>,
        iso3166_2: String,
    ) -> Result<Option<Country>, async_graphql::Error> {
        let context = &ctx.data_unchecked::<AppContext>().loaders.countries;
        context.load_one(iso3166_2).await
    }
}
