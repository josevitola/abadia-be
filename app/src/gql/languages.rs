use super::super::db::languages::Language;
use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

#[Object]
impl Language {
    async fn iso693_3(&self) -> &str {
        &self.iso693_3
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Default)]
pub(super) struct LanguageQuery;

#[Object]
impl LanguageQuery {
    async fn languages(&self, ctx: &Context<'_>) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        Ok(Language::list(pool).await?)
    }

    async fn languages_by_name(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        Ok(Language::list_by_name(pool, keyword).await?)
    }
}
