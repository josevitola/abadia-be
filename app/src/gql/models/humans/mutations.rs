use async_graphql::{Context, Object};

use super::{db::CreateHumanInput, AppContext, HumanDB};

#[derive(Default)]
pub struct HumanMutation; // glorious evolution!

#[Object]
impl HumanMutation {
    async fn create_human(
        &self,
        ctx: &Context<'_>,
        input: CreateHumanInput,
    ) -> Result<String, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let conn = &mut *tx;

        let id = HumanDB::insert_one(conn, input).await?;

        tx.commit().await?;

        Ok(id)
    }
}
