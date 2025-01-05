use async_graphql::SimpleObject;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
pub struct Country {
    pub iso3166_2: String,
    pub name: String,
}
