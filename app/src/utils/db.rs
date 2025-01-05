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

// pub trait DBReadOne<T> {
//     async fn fetch_one(
//         pool: &PgPool,
//         query: Query<'_, Postgres, PgArguments>,
//     ) -> Result<T, sqlx::Error>;
// }

pub trait DBReadMany<T> {
    async fn fetch_many(
        pool: &PgPool,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Vec<T>, sqlx::Error>;
}

pub trait DBInsertOne<T> {
    async fn insert_one(tx: &mut PgConnection, input: T) -> String;
}

// pub trait DBManager<T>: DBReadMany<T> + DBReadOne<T> + DBInsertOne<T> {}
