use async_graphql::{Context, Object, SimpleObject};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

#[derive(SimpleObject)]
pub(crate) struct Country {
    pub iso3166: String,
    pub name: String
}

#[derive(Default)]
pub(crate) struct CountryQuery;

#[Object]
impl CountryQuery {
    async fn countries(&self, ctx: &Context<'_>) -> Result<Vec<Country>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        let query: Vec<Country> = sqlx::query("SELECT * FROM countries ORDER BY iso3166")
            .map(|row: PgRow| Country {
                iso3166: row.get("iso3166"),
                name: row.get("name"),
            })
            .fetch_all(pool).await?;

        Ok(query)
    }
}