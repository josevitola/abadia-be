use crate::utils::db::DBManager;
use crate::{
    gql::{
        models::{
            humans::{Human, HumanDB},
            texts::{Text, TextDB},
        },
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

        let query = sqlx::query(
            "
            SELECT t.*, bt.orderidx FROM book_texts bt RIGHT JOIN texts t ON (t.id = bt.text_id)
            WHERE bt.book_id = $1
            ORDER BY orderidx
            ",
        )
        .bind(&self.id);

        Ok(TextDB::fetch_many(pool, query).await?)
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

    async fn authors(&self, ctx: &Context<'_>) -> Result<Vec<Human>, async_graphql::Error> {
        let context = ctx.data::<AppContext>()?;
        let pool = &context.pool;

        let query = sqlx::query(
            "
        SELECT h.*
        FROM
        	books b
        	LEFT JOIN book_texts bt ON (b.id = bt.book_id)
        	LEFT JOIN texts t ON (bt.text_id = t.id)
        	LEFT JOIN text_authors ta ON (t.id = ta.text_id)
        	LEFT JOIN humans h ON (h.id = ta.author_id)
        WHERE b.id = $1
        GROUP BY h.id
        ORDER BY h.last_name ASC
        ",
        )
        .bind(&self.id);

        Ok(HumanDB::fetch_many(pool, query).await?)
    }
}
