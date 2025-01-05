use async_graphql::{Context, Object, SimpleObject};
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgPool, Postgres, Row,
};

use crate::gql::AppContext;

#[derive(SimpleObject)]
pub struct Language {
    pub iso693: String,
    pub name: String,
}

pub struct LanguageUtil;

impl LanguageUtil {
    pub async fn fetch(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<Language>, sqlx::Error> {
        let res = query
            .map(|row: PgRow| Language {
                iso693: row.get("iso693"),
                name: row.get("name"),
            })
            .fetch_all(pool)
            .await?;

        Ok(res)
    }

    pub async fn fetch_one(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Language, sqlx::Error> {
        let res = query
            .map(|row: PgRow| Language {
                iso693: row.get("iso693"),
                name: row.get("name"),
            })
            .fetch_one(pool)
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
        let query = sqlx::query("SELECT * FROM languages ORDER BY iso693");

        Ok(LanguageUtil::fetch(pool, query).await?)
    }

    async fn languages_by_name(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM languages WHERE name ILIKE $1 ORDER BY iso693")
            .bind(format!("%{keyword}%"));

        Ok(LanguageUtil::fetch(pool, query).await?)
    }
}
