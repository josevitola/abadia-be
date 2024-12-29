use async_graphql::{MergedObject, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

mod countries;
mod languages;

use countries::CountryQuery;
use languages::LanguageQuery;

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
    
#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(CountryQuery, LanguageQuery);