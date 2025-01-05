use async_graphql::InputObject;
use sqlx::PgConnection;

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
}
