use async_graphql::{Context, InputObject, Object};

use super::AppContext;

#[derive(InputObject)]
struct CreateHumanInput {
    last_name: String,
    first_name: Option<String>,
    country1_id: Option<String>,
    country2_id: Option<String>,
    birthyear: Option<i32>,
    pseudonym: Option<String>
}

#[derive(Default)]
pub struct HumanMutation; // glorious evolution!

#[Object]
impl HumanMutation {
    async fn create_human(
        &self,
        ctx: &Context<'_>,
        input: CreateHumanInput
    ) -> Result<u64, async_graphql::Error> {
            let pool = &ctx.data::<AppContext>()?.pool;

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
                    .execute(pool)
                    .await?;

            Ok(res.rows_affected())
        }
}