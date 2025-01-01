use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

use super::authors::AuthorLoader;

pub(crate) struct AppDataLoaders {
    pub authors: DataLoader<AuthorLoader>,
}

pub(crate) struct AppContext {
    pub pool: PgPool,
    pub loaders: AppDataLoaders,
}
