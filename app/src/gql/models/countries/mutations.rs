use async_graphql::{Context, Object};

use super::AppContext;

#[derive(Default)]
pub struct CountryMutation;

#[Object]
impl CountryMutation {
    async fn create_country(
        &self,
        ctx: &Context<'_>,
        iso3166_2: String,
        name: String,
    ) -> Result<u64, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let res = sqlx::query("INSERT INTO countries (iso3166_2, name) VALUES ($1, $2)")
            .bind(iso3166_2)
            .bind(name)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }
}
