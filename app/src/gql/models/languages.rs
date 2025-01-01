use async_graphql::{Context, Object, SimpleObject};
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgPool, Postgres, Row,
};

use crate::gql::AppContext;

#[derive(SimpleObject)]
struct Language {
    pub iso693_3: String,
    pub name: String,
}

struct LanguageUtil;

impl LanguageUtil {
    async fn fetch(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<Language>, sqlx::Error> {
        let res = query
            .map(|row: PgRow| Language {
                iso693_3: row.get("iso693_3"),
                name: row.get("name"),
            })
            .fetch_all(pool)
            .await?;

        Ok(res)
    }
}

#[derive(Default)]
pub struct LanguageQuery;

#[Object]
impl LanguageQuery {
    async fn languages(&self, ctx: &Context<'_>) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM languages ORDER BY iso693_3");

        Ok(LanguageUtil::fetch(pool, query).await?)
    }

    async fn languages_by_name(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM languages WHERE name ILIKE $1 ORDER BY iso693_3")
            .bind(format!("%{keyword}%"));

        Ok(LanguageUtil::fetch(pool, query).await?)
    }
}
