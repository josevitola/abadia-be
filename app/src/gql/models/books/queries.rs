use super::{book::Book, db::BookDB};
use crate::gql::AppContext;
use crate::utils::db::DBManager;
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct BookQuery;

#[Object]
impl BookQuery {
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<Book>, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM books ORDER BY title");

        Ok(BookDB::fetch_many(pool, query).await?)
    }

    async fn book(&self, ctx: &Context<'_>, id: Uuid) -> Result<Book, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;
        let query = sqlx::query("SELECT * FROM books WHERE id = $1").bind(id);

        Ok(BookDB::fetch_one(pool, query).await?)
    }
}
