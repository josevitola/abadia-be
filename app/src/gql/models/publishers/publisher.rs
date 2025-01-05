use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub country1_id: String,
    pub country2_id: Option<String>,
    pub parent_id: Option<String>,
}
