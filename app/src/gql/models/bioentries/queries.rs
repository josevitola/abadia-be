use super::{db::BiographyEntryDB, AppContext, BiographyEntry};
use crate::utils::db::DBManager;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct BiographyEntryQuery;

#[Object]
impl BiographyEntryQuery {
    async fn bios_for_human(
        &self,
        ctx: &Context<'_>,
        human_id: String,
    ) -> Result<Vec<BiographyEntry>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let res = BiographyEntryDB::fetch_many(
            pool,
            sqlx::query("SELECT * FROM bioentries WHERE human_id = $1").bind(human_id),
        )
        .await?;

        Ok(res)
    }
}
