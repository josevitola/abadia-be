use super::book::Book;
use crate::utils::db::*;
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgConnection, PgPool, Postgres, Row,
};

pub struct CreateBookPayload {
    pub title: String,
    pub npages: String,
}

pub struct BookDB;

impl DBManager<Book, CreateBookPayload> for BookDB {
    async fn insert_one(
        tx: &mut PgConnection,
        input: CreateBookPayload,
    ) -> Result<String, DBError> {
        let CreateBookPayload { title, npages } = input;

        let res: Result<String, sqlx::Error> =
            sqlx::query_scalar("INSERT INTO texts (title, npages) VALUES ($1, $2) RETURNING ID")
                .bind(title)
                .bind(npages)
                .fetch_one(tx)
                .await;

        match res {
            Ok(id) => Ok(id.to_string()),
            Err(err) => {
                println!("{:?}", err);
                Err(DBError::Insert(err.to_string()))
            }
        }
    }

    async fn fetch_many(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<Book>, sqlx::Error> {
        Ok(query.map(BookDB::to_struct).fetch_all(pool).await?)
    }

    async fn fetch_one(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Book, sqlx::Error> {
        Ok(query.map(BookDB::to_struct).fetch_one(pool).await?)
    }

    fn to_struct(row: PgRow) -> Book {
        Book {
            id: row.get("id"),
            dcr: row.get("dcr"),
            title: row.get("title"),
            publisher_id: row.get("publisher_id"),
            is_compilation: row.get("is_compilation"),
            isbn10: row.get("isbn10"),
            isbn13: row.get("isbn13"),
            npages: row.get("npages"),
            year: row.get("year"),
            printed_in: row.get("printed_in"),
        }
    }
}
