use super::super::db::{authors::Author, countries::Country};
use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

#[Object]
impl Author {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn last_name(&self) -> &str {
        &self.last_name
    }

    async fn first_name(&self) -> &Option<String> {
        &self.first_name
    }

    async fn country1(&self) -> &Option<String> {
        &self.country1
    }

    async fn country2(&self) -> &Option<String> {
        &self.country2
    }

    async fn birthyear(&self) -> &Option<String> {
        &self.birthyear
    }

    async fn pseudonym(&self) -> &Option<String> {
        &self.pseudonym
    }

    async fn country(&self) -> Country {
        let country_list = vec![
            Country {
                iso3166: "AT".into(),
                name: "Austria".into(),
            },
            Country {
                iso3166: "CA".into(),
                name: "Canadá".into(),
            },
            Country {
                iso3166: "CO".into(),
                name: "Colombia".into(),
            },
            Country {
                iso3166: "DE".into(),
                name: "Alemania".into(),
            },
        ];

        country_list
            .into_iter()
            .find(|country| country.iso3166 == self.country1.as_ref().unwrap().as_str())
            .unwrap()
    }
}

#[derive(Default)]
pub(crate) struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn authors(&self, ctx: &Context<'_>) -> Result<Vec<Author>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        Ok(Author::list(pool).await?)
    }
}
