use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    Pool, Postgres, Row,
};

pub(crate) struct Language {
    pub iso693_3: String,
    pub name: String,
}

impl Language {
    pub async fn list(pool: &Pool<Postgres>) -> Result<Vec<Language>, sqlx::Error> {
        Language::fetch(
            pool,
            sqlx::query("SELECT * FROM languages ORDER BY iso693_3"),
        )
        .await
    }

    pub async fn list_by_name(
        pool: &Pool<Postgres>,
        keyword: String,
    ) -> Result<Vec<Language>, sqlx::Error> {
        Language::fetch(
            pool,
            sqlx::query("SELECT * FROM languages WHERE name ILIKE $1 ORDER BY iso693_3")
                .bind(format!("%{keyword}%")),
        )
        .await
    }

    async fn fetch(
        pool: &Pool<Postgres>,
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
