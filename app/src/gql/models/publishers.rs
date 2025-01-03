use crate::gql::AppContext;
use async_graphql::{Context, Object, SimpleObject};
use sqlx::{postgres::PgRow, Row};

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub country1_id: String,
    pub country2_id: Option<String>,
    pub parent_id: Option<String>,
}

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
