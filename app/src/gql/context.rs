use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

use super::models::{
    books::BookLoader, countries::CountryLoader, humans::HumanLoader, texts::TextLoader,
};

pub(crate) struct AppDataLoaders {
    pub countries: DataLoader<CountryLoader>,
    pub humans: DataLoader<HumanLoader>,
    pub texts: DataLoader<TextLoader>,
    pub books: DataLoader<BookLoader>,
}

pub(crate) struct AppContext {
    pub pool: PgPool,
    pub loaders: AppDataLoaders,
}
