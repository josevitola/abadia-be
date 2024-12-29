use crate::model::QueryRoot;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, middleware, routing::get, Router, Server};
use dotenv::dotenv;
use std::future::ready;
use std::env;

// mod db;
mod model;
mod observability;
mod routes;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

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
}