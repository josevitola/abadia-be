use crate::gql::AppContext;
use async_graphql::{Context, Object};
use sqlx::{postgres::PgRow, Row};

mod publisher;

pub use publisher::Publisher;

#[derive(Default)]
pub(crate) struct PublisherQuery;

#[Object]
impl PublisherQuery {
    async fn publishers(&self, ctx: &Context<'_>) -> Result<Vec<Publisher>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let query: Vec<Publisher> = sqlx::query("SELECT * FROM publishers ORDER BY name")
            .map(|row: PgRow| Publisher {
                id: row.get("id"),
                name: row.get("name"),
                country1_id: row.get("country1_id"),
                country2_id: row.get("country2_id"),
                parent_id: row.get("parent_id"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }
}
