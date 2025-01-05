use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgPool, Postgres, Row,
};

use super::Language;

pub struct LanguageDB;

impl LanguageDB {
    pub async fn fetch<'a>(
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
