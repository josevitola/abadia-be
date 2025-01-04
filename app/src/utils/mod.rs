use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    PgPool, Postgres, Row,
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
