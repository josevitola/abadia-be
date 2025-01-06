use super::{
    db::{BiographyEntryDB, BiographyEntryForm},
    AppContext,
};
use crate::utils::db::DBManager;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct BiographyEntryMutation; // glorious evolution!

#[Object]
impl BiographyEntryMutation {
    async fn create_biography_entry(
        &self,
        ctx: &Context<'_>,
        input: BiographyEntryForm,
    ) -> Result<String, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let new_id_res = BiographyEntryDB::insert_one(&mut *tx, input).await?;
        tx.commit().await?;

        Ok(new_id_res)
    }
}
