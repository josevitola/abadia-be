use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
pub(super) struct Country {
    pub iso3166: String,
    pub name: String
}

#[derive(Default)]
pub(super) struct CountryQuery;

#[Object]
impl CountryQuery {
    async fn countries(&self) -> Vec<Country> {
        vec![
            Country { iso3166: "AT".into(), name: "Austria".into() },
            Country { iso3166: "CA".into(), name: "Canadá".into() },
            Country { iso3166: "CO".into(), name: "Colombia".into() },
            Country { iso3166: "DE".into(), name: "Alemania".into() },
        ]
    }
}