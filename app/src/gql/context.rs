use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

use super::models::{authors::AuthorLoader, countries::CountryLoader};

pub(crate) struct AppDataLoaders {
    pub authors: DataLoader<AuthorLoader>,
    pub countries: DataLoader<CountryLoader>,
}

pub(crate) struct AppContext {
    pub pool: PgPool,
    pub loaders: AppDataLoaders,
}
