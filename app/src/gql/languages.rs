use async_graphql::{Context, Object, SimpleObject};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

#[derive(SimpleObject)]
pub(super) struct Language {
    pub iso693_3: String,
    pub name: String
}

#[derive(Default)]
pub(super) struct LanguageQuery;

#[Object]
impl LanguageQuery {
    async fn languages(&self, ctx: &Context<'_>) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        let query: Vec<Language> = sqlx::query("SELECT * FROM languages ORDER BY iso693_3")
            .map(|row: PgRow| Language {
                iso693_3: row.get("iso693_3"),
                name: row.get("name"),
            })
            .fetch_all(pool).await?;

        Ok(query)
    }

    async fn languages_by_name(&self, ctx: &Context<'_>, keyword: String) -> Result<Vec<Language>, async_graphql::Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        let query: Vec<Language> = sqlx::query("SELECT * FROM languages WHERE name ILIKE $1 ORDER BY iso693_3")
            .bind(format!("%{keyword}%"))
            .map(|row: PgRow| Language {
                iso693_3: row.get("iso693_3"),
                name: row.get("name"),
            })
            .fetch_all(pool).await?;

        Ok(query)
    }
}