use crate::gql::AppContext;
use async_graphql::{Context, Object};

mod db;
mod language;

pub use db::LanguageDB;
pub use language::Language;

#[derive(Default)]
pub struct LanguageQuery;

#[Object]
impl LanguageQuery {
    async fn languages(&self, ctx: &Context<'_>) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM languages ORDER BY iso693");

        Ok(LanguageDB::fetch(pool, query).await?)
    }

    async fn languages_by_name(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM languages WHERE name ILIKE $1 ORDER BY iso693")
            .bind(format!("%{keyword}%"));

        Ok(LanguageDB::fetch(pool, query).await?)
    }
}
