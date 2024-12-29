use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub(crate) struct Country {
    pub iso3166: String,
    pub name: String
}