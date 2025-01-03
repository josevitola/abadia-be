use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::{MergedObject, Schema};

mod context;
pub mod models;

use models::countries::CountryQuery;
use models::humans::HumanQuery;
use models::languages::LanguageQuery;
use models::publishers::PublisherQuery;
use models::texts::TextQuery;

pub(crate) use context::{AppContext, AppDataLoaders};

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(
    CountryQuery,
    LanguageQuery,
    HumanQuery,
    TextQuery,
    PublisherQuery,
);
