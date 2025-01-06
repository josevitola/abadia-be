use async_graphql::EmptySubscription;
use async_graphql::{MergedObject, Schema};

mod context;
pub mod models;

use models::bioentries::{BiographyEntryMutation, BiographyEntryQuery};
use models::books::{BookMutation, BookQuery};
use models::countries::{CountryMutation, CountryQuery};
use models::humans::{HumanMutation, HumanQuery};
use models::languages::LanguageQuery;
use models::publishers::PublisherQuery;
use models::texts::{TextMutation, TextQuery};

pub(crate) use context::{AppContext, AppDataLoaders};

pub(crate) type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(
    BookQuery,
    CountryQuery,
    LanguageQuery,
    HumanQuery,
    TextQuery,
    PublisherQuery,
    BiographyEntryQuery,
);

#[derive(MergedObject, Default)]
pub(crate) struct MutationRoot(
    BookMutation,
    CountryMutation,
    TextMutation,
    HumanMutation,
    BiographyEntryMutation,
);
