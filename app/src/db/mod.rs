use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub(crate) mod authors;
pub(crate) mod countries;
pub(crate) mod languages;

pub(crate) async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap().as_str())
        .await?;

    Ok(pool)
}
