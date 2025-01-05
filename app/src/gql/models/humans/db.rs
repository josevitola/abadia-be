use async_graphql::InputObject;
use sqlx::{postgres::{PgArguments, PgRow}, query::Query, PgConnection, PgPool, Postgres, Row};

use super::Human;

pub struct HumanDB;

#[derive(InputObject)]
pub struct CreateHumanInput {
    last_name: String,
    first_name: Option<String>,
    country1_id: Option<String>,
    country2_id: Option<String>,
    birthyear: Option<i32>,
    pseudonym: Option<String>,
}

impl HumanDB {
    pub async fn insert_one(tx: &mut PgConnection, input: CreateHumanInput) -> Result<u64, async_graphql::Error> {
            let CreateHumanInput {
                birthyear,
                country1_id,
                country2_id,
                first_name,
                last_name,
                pseudonym
            } = input;

            let res = 
                sqlx::query("INSERT INTO humans (last_name, first_name, country1_id, country2_id, birthyear, pseudonym) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(last_name)
                    .bind(first_name)
                    .bind(country1_id)
                    .bind(country2_id)
                    .bind(birthyear)
                    .bind(pseudonym)
                    .execute(tx)
                    .await?;

            Ok(res.rows_affected())
        }

        
    pub async fn fetch_many(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<Human>, sqlx::Error> {
        Ok(query.map(HumanDB::to_struct).fetch_all(pool).await?)
    }

    pub fn to_struct(row: PgRow) -> Human {
        Human {
            id: row.get("id"),
            last_name: row.get("last_name"),
            first_name: row.get("first_name"),
            country1_id: row.get("country1_id"),
            country2_id: row.get("country2_id"),
            birthyear: row.get("birthyear"),
            pseudonym: row.get("pseudonym"),
        }
    }
}
