use super::Text;
use crate::utils::db::*;
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgConnection, PgPool, Postgres, Row,
};

pub struct CreateTextInput {
    pub title: String,
    pub orig_language_id: String,
}

pub struct TextDB;

impl DBManager<Text, CreateTextInput> for TextDB {
    async fn insert_one(tx: &mut PgConnection, input: CreateTextInput) -> String {
        let CreateTextInput {
            title,
            orig_language_id,
        } = input;

        let res: Result<String, sqlx::Error> = sqlx::query_scalar(
            "INSERT INTO texts (title, orig_language_id) VALUES ($1, $2) RETURNING ID",
        )
        .bind(title)
        .bind(orig_language_id)
        .fetch_one(tx)
        .await;

        match res {
            Ok(id) => id,
            Err(_) => String::from(""),
        }
    }

    async fn fetch_many(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<Text>, sqlx::Error> {
        Ok(query.map(TextDB::to_struct).fetch_all(pool).await?)
    }

    async fn fetch_one(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Text, sqlx::Error> {
        Ok(query.map(TextDB::to_struct).fetch_one(pool).await?)
    }

    fn to_struct(row: PgRow) -> Text {
        Text {
            id: row.get("id"),
            title: row.get("title"),
            orig_language_id: row.get("orig_language_id"),
            dcr: row.get("dcr"),
        }
    }
}
