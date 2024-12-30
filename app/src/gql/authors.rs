use async_graphql::{ComplexObject, Object, SimpleObject};

use super::countries::Country;

#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub(super) struct Author {
    pub id: &'static str,
    pub last_name: String,
    pub first_name: String,
    pub country1: String,
    pub country2: Option<String>,
    pub birthyear: Option<String>,
    pub pseudonym: Option<String>,
}

#[ComplexObject]
impl Author {
    async fn country(&self) -> Country {
        let country_list = vec![
            Country { iso3166: "AT".into(), name: "Austria".into() },
            Country { iso3166: "CA".into(), name: "Canadá".into() },
            Country { iso3166: "CO".into(), name: "Colombia".into() },
            Country { iso3166: "DE".into(), name: "Alemania".into() },
        ];

        country_list
            .into_iter()
            .find(|country| country.iso3166 == self.country1).unwrap()
    }
}

#[derive(Default)]
pub(super) struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn authors(&self) -> Vec<Author> {
        vec![
            Author {
                id: "bb05284e-d760-430b-832d-5b88b3f05185",
                last_name: "Zweig".to_string(),
                first_name: "Stefan".to_string(),
                country1: "AT".to_string(),
                country2: Some("GB-ENG".to_string()),
                ..Default::default()
            },
        ]
    }
}