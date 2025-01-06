use super::BiographyEntry;
use crate::utils::db::{DBError, DBManager};
use async_graphql::InputObject;
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgConnection, PgPool, Postgres, Row,
};
use uuid::Uuid;

pub struct BiographyEntryDB;

#[derive(InputObject)]
pub struct BiographyEntryForm {
    pub human_id: String,
    pub entry: String,
    pub appears_in_id: Uuid,
}

impl DBManager<BiographyEntry, BiographyEntryForm> for BiographyEntryDB {
    async fn insert_one(
        tx: &mut PgConnection,
        input: BiographyEntryForm,
    ) -> Result<String, DBError> {
        let BiographyEntryForm {
            human_id,
            entry,
            appears_in_id,
        } = input;

        let res: Result<i32, sqlx::Error> = sqlx::query_scalar(
            "INSERT INTO bioentries (human_id, entry, appears_in_id) VALUES ($1, $2, $3) RETURNING ID",
        )
        .bind(human_id)
        .bind(entry)
        .bind(appears_in_id)
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
    ) -> Result<Vec<BiographyEntry>, sqlx::Error> {
        Ok(query.map(Self::to_struct).fetch_all(pool).await?)
    }

    async fn fetch_one(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<BiographyEntry, sqlx::Error> {
        Ok(query.map(Self::to_struct).fetch_one(pool).await?)
    }

    fn to_struct(row: PgRow) -> BiographyEntry {
        BiographyEntry {
            id: row.get("id"),
            human_id: row.get("human_id"),
            entry: row.get("entry"),
            appears_in_id: row.get("appears_in_id"),
        }
    }
}
