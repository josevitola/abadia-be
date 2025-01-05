use async_graphql::{Context, Object};

use crate::gql::AppContext;

#[derive(Default)]
pub struct BookMutation;

#[Object]
impl BookMutation {
    async fn add_texts_to_book(
        &self,
        ctx: &Context<'_>,
        book_id: String,
        text_ids: Vec<String>
    ) -> Result<u64, async_graphql::Error> {
        let pool = &ctx.data::<AppContext>()?.pool;

        let mut tx = pool.begin().await?;
        let conn = &mut *tx;

        let book_id_col: Vec<String> = vec![book_id; text_ids.len()];

        let text_authors_insert_res = 
            sqlx::query("INSERT INTO book_texts (book_id, text_id) SELECT * FROM UNNEST($1::uuid[], $2::text[])")
                .bind(book_id_col)
                .bind(&text_ids)
                .execute(conn)
                .await?;

        tx.commit().await?;

        Ok(text_authors_insert_res.rows_affected())
    }
}
