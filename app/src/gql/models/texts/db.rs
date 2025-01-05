use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgConnection, PgPool, Postgres, Row,
};

use crate::utils::db::*;

use super::Text;

pub struct CreateTextInput {
    pub title: String,
}

pub struct TextDB;

impl DBInsertOne<CreateTextInput> for TextDB {
    async fn insert_one(tx: &mut PgConnection, input: CreateTextInput) -> String {
        let CreateTextInput { title } = input;
        let res: Result<String, sqlx::Error> =
            sqlx::query_scalar("INSERT INTO texts (title) VALUES ($1) RETURNING ID")
                .bind(title)
                .fetch_one(tx)
                .await;

        match res {
            Ok(id) => id,
            Err(_) => String::from(""),
        }
    }

    // async fn fetch_one(
    //     pool: &PgPool,
    //     query: Query<'_, Postgres, PgArguments>,
    // ) -> Result<Text, sqlx::Error> {
    //     let res = query
    //         .map(|row: PgRow| Text {
    //             id: row.get("id"),
    //             title: row.get("title"),
    //             orig_language_id: row.get("orig_language_id"),
    //         })
    //         .fetch_one(pool)
    //         .await?;

    //     Ok(res)
    // }
}

impl DBReadMany<Text> for TextDB {
    async fn fetch_many(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<Text>, sqlx::Error> {
        let res = query
            .map(|row: PgRow| Text {
                id: row.get("id"),
                title: row.get("title"),
                orig_language_id: row.get("orig_language_id"),
            })
            .fetch_all(pool)
            .await?;

        Ok(res)
    }
}
