use super::{
    db::{BiographyEntryDB, BiographyEntryForm},
    AppContext,
};
use crate::utils::db::{DBError, DBManager};
use async_graphql::{Context, Error, Object};

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
        let conn = &mut *tx;

        let new_id_res = BiographyEntryDB::insert_one(conn, input).await;

        new_id_res.map_err(|e| match e {
            DBError::Insert(insert_msg) => Error {
                message: insert_msg,
                extensions: None,
                source: None,
            },
        })
    }
}
