use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;

pub(crate) async fn create_pool() -> PgPoolOptions {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database_name")
        .await?;
}