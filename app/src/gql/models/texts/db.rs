use sqlx::PgConnection;

pub struct TextDB;

impl TextDB {
    pub async fn create_text(tx: &mut PgConnection, title: String) -> String {
        let res: Result<String, sqlx::Error> =
            sqlx::query_scalar("INSERT INTO texts (title) VALUES ($1) RETURNING ID")
                .bind(title)
                .fetch_one(tx)
                .await;

        match res {
            Ok(id) => id,
            Err(_) => String::from(""),
        }
    }
}
