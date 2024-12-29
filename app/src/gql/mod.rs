use async_graphql::{MergedObject, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

mod countries;
mod languages;
mod authors;

use countries::CountryQuery;
use languages::LanguageQuery;
use authors::AuthorQuery;

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
    
#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(CountryQuery, LanguageQuery, AuthorQuery);