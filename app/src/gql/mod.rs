use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

mod objects;

use objects::Country;

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello test!"
    }

    async fn countries(&self, _ctx: &Context<'_>) -> Vec<Country> {
        vec![
            Country { iso3166: "AT".into(), name: "Austria".into() },
            Country { iso3166: "CA".into(), name: "Canadá".into() },
            Country { iso3166: "CO".into(), name: "Colombia".into() },
            Country { iso3166: "DE".into(), name: "Alemania".into() },
        ]
    }
}