use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

use super::schema::create_schema;
use crate::context::Context;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn run_server() -> Result<(), std::io::Error> {
    let schema = create_schema().data(Context::new()).finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app).await?;

    println!("GraphiQL IDE: http://localhost:8000");

    Ok(())
}
