use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Language {
    pub iso693: String,
    pub name: String,
}
