use crate::db::countries::Country;
use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

#[Object]
impl Country {
    async fn iso3166(&self) -> &str {
        &self.iso3166
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Default)]
pub(crate) struct CountryQuery;

#[Object]
impl CountryQuery {
    async fn countries(&self, ctx: &Context<'_>) -> Result<Vec<Country>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        Ok(Country::list(pool).await?)
    }
}
