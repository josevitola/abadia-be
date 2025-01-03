use crate::gql::QueryRoot;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, middleware, routing::get, Router, Server};
use dotenv::dotenv;
use gql::models::countries::CountryLoader;
use gql::models::texts::TextLoader;
use gql::{models::humans::HumanLoader, AppContext, AppDataLoaders};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::future::ready;

mod gql;
mod observability;
mod routes;

// Built following https://oliverjumpertz.com/blog/how-to-build-a-powerful-graphql-api-with-rust/

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap().as_str())
        .await?;

    let humanloader = HumanLoader::new(pool.clone());
    let countryloader = CountryLoader::new(pool.clone());
    let textloader = TextLoader::new(pool.clone());

    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(AppContext {
            pool: pool.clone(),
            loaders: AppDataLoaders {
                countries: DataLoader::new(countryloader, tokio::spawn),
                humans: DataLoader::new(humanloader, tokio::spawn),
                texts: DataLoader::new(textloader, tokio::spawn),
            },
        })
        .finish();

    let prometheus_recorder = create_prometheus_recorder();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        .route("/metrics", get(move || ready(prometheus_recorder.render()))) // (1)
        .route_layer(middleware::from_fn(track_metrics)) // (2)
        .layer(Extension(schema));

    Server::bind(&env::var("URL").unwrap().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
