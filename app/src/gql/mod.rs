use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::{MergedObject, Schema};

mod authors;
mod countries;
mod languages;

use authors::AuthorQuery;
use countries::CountryQuery;
use languages::LanguageQuery;

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(CountryQuery, LanguageQuery, AuthorQuery);
