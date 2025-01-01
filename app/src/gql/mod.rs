use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::{MergedObject, Schema};

mod context;
pub mod models;

use models::authors::AuthorQuery;
use models::countries::CountryQuery;
use models::languages::LanguageQuery;

pub(crate) use context::{AppContext, AppDataLoaders};

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(CountryQuery, LanguageQuery, AuthorQuery);
