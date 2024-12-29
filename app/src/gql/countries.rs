use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
pub(crate) struct Country {
    pub iso3166: &'static str,
    pub name: String
}

#[derive(Default)]
pub(crate) struct CountryQuery;

#[Object]
impl CountryQuery {
    async fn countries(&self) -> Vec<Country> {
        vec![
            Country { iso3166: "AT", name: "Austria".into() },
            Country { iso3166: "CA", name: "Canadá".into() },
            Country { iso3166: "CO", name: "Colombia".into() },
            Country { iso3166: "DE", name: "Alemania".into() },
        ]
    }
}