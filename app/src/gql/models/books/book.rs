use crate::{
    gql::{
        models::{humans::Human, texts::Text},
        AppContext,
    },
    utils::db::get_bridge_ids,
};
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Hash, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub publisher_id: String,
    pub npages: i16,
    pub dcr: DateTime<Utc>,
    pub isbn10: Option<i64>,
    pub isbn13: Option<i64>,
    pub year: Option<i16>,
    pub printed_in: String,
    pub is_compilation: bool,
}

#[ComplexObject]
impl Book {
    async fn texts(&self, ctx: &Context<'_>) -> Result<Vec<Text>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        let query = sqlx::query("SELECT text_id AS bridge FROM book_texts WHERE book_id = $1")
            .bind(&self.id);

        let text_ids: Vec<String> = get_bridge_ids(query, pool).await?;
        if text_ids.is_empty() {
            ()
        }

        let text_loader = &context.loaders.texts;

        Ok(text_loader
            .load_many(text_ids)
            .await?
            .values()
            .cloned()
            .collect())
    }

    async fn editors(&self, ctx: &Context<'_>) -> Result<Vec<Human>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        let query = sqlx::query("SELECT editor_id AS bridge FROM book_editors WHERE book_id = $1")
            .bind(&self.id);

        let editor_ids: Vec<String> = get_bridge_ids(query, pool).await?;
        if editor_ids.is_empty() {
            ()
        }

        let human_loader = &context.loaders.humans;

        Ok(human_loader
            .load_many(editor_ids)
            .await?
            .values()
            .cloned()
            .collect())
    }
}
