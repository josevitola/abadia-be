use sqlx::{postgres::PgRow, prelude::FromRow, Pool, Postgres, Row};

#[derive(FromRow)]
pub(crate) struct Author {
    pub id: String,
    pub last_name: String,
    #[sqlx(default)]
    pub first_name: Option<String>,
    #[sqlx(default)]
    pub country1: Option<String>,
    #[sqlx(default)]
    pub country2: Option<String>,
    #[sqlx(default)]
    pub birthyear: Option<String>,
    #[sqlx(default)]
    pub pseudonym: Option<String>,
}

impl Author {
    pub async fn list(pool: &Pool<Postgres>) -> Result<Vec<Author>, sqlx::Error> {
        let query: Vec<Author> = sqlx::query("SELECT * FROM authors ORDER BY last_name")
            .map(|row: PgRow| Author {
                id: row.get("id"),
                last_name: row.get("last_name"),
                first_name: row.get("first_name"),
                country1: row.get("country1"),
                country2: row.get("country2"),
                birthyear: row.get("birthyear"),
                pseudonym: row.get("pseudonym"),
            })
            .fetch_all(pool)
            .await?;

        Ok(query)
    }
}
