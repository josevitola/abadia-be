use async_graphql::SimpleObject;
use uuid::Uuid;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct BiographyEntry {
    pub id: i32,
    pub human_id: String,
    pub entry: String,
    pub appears_in_id: Uuid,
}
