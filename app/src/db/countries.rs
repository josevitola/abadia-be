use sqlx::{postgres::PgRow, Pool, Postgres, Row};

#[derive(Debug, Clone)]
pub(crate) struct Country {
    pub iso3166: String,
    pub name: String
}

impl Country {
    pub async fn list(pool: &Pool<Postgres>) -> Result<Vec<Country>, sqlx::Error> {
        let query: Vec<Country> = sqlx::query("SELECT * FROM countries ORDER BY iso3166")
            .map(|row: PgRow| Country {
                iso3166: row.get("iso3166"),
                name: row.get("name"),
            })
            .fetch_all(pool).await?;

        Ok(query)
    }
}