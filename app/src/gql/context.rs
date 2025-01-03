use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

use super::models::{countries::CountryLoader, humans::HumanLoader};

pub(crate) struct AppDataLoaders {
    pub humans: DataLoader<HumanLoader>,
    pub countries: DataLoader<CountryLoader>,
}

pub(crate) struct AppContext {
    pub pool: PgPool,
    pub loaders: AppDataLoaders,
}
