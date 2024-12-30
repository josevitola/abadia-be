use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub(crate) async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let user = env::var("POSTGRES_USER").unwrap();
    let pwd = env::var("POSTGRES_PASSWORD").unwrap();
    let host = env::var("POSTGRES_HOST").unwrap();
    let port = env::var("POSTGRES_PORT").unwrap();
    let db = env::var("POSTGRES_DB").unwrap();

    let url = format!("postgres://{user}:{pwd}@{host}:{port}/{db}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;

    Ok(pool)
}