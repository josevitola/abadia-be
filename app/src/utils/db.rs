use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgConnection, PgPool, Postgres, Row,
};

struct Bridge {
    bridge: String,
}

pub(crate) async fn get_bridge_ids(
    query: Query<'_, Postgres, PgArguments>,
    pool: &PgPool,
) -> Result<Vec<String>, sqlx::Error> {
    let res = query
        .map(|row: PgRow| Bridge {
            bridge: row.get("bridge"),
        })
        .fetch_all(pool)
        .await?;

    if res.is_empty() {
        ()
    }

    Ok(res.into_iter().map(|bridge| bridge.bridge).collect())
}

pub trait DBManager<T, C> {
    async fn insert_one(tx: &mut PgConnection, input: C) -> String;

    async fn fetch_many(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<T>, sqlx::Error>;

    async fn fetch_one(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<T, sqlx::Error>;

    fn to_struct(row: PgRow) -> T;
}
