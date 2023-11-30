use anyhow::Ok;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    routing::post,
    Router,
};
use search_engine::article::Article;
use search_engine::wrapper::VnCore;
use search_engine::*;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use std::time::Duration;
use tantivy::{doc, Index};
use tempfile::TempDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let index_path = TempDir::new()?;

    let schema = get_article_schema();
    let index: Index = Index::create_in_dir(&index_path, schema.clone())?;
    let vn_tokenizer = VnCore::default();
    index.tokenizers().register("vn_core", vn_tokenizer);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:tropical@localhost/articles".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app_state = AppState { pool, index };
    //  indexing articles in db
    println!("Indexing articles in db");
    let mut index_writer = app_state.index.writer(50_000_000)?;
    let articles = sqlx::query_as::<_, Article>(r#"select * from "article""#)
        .fetch_all(&app_state.pool)
        .await?;
    let title_field = schema.get_field("title").unwrap();
    let content_field = schema.get_field("content").unwrap();
    let summary_field = schema.get_field("summary").unwrap();
    let url_field = schema.get_field("url").unwrap();
    let timestamp_field = schema.get_field("created_time").unwrap();
    let id_field = schema.get_field("id").unwrap();
    for article in articles {
        index_writer.add_document(doc!(
            title_field => article.title,
            content_field => article.content,
            summary_field => article.summary,
            url_field => article.url,
            timestamp_field => article.timestamp,
            id_field => article.id
        ))?;
    }
    index_writer.commit()?;

    // build our application with some routes
    println!("Server is running on port 3000");
    let app = Router::new()
        .route("/api/articles", post(article::create_article))
        .route("/api/articles", get(article::get_article))
        .route("/api/articles/query", get(article::query_article))
        .with_state(app_state);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

// // we can extract the connection pool with `State`
// async fn using_connection_pool_extractor(
//     State(pool): State<PgPool>,
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
// }

// // we can also write a custom extractor that grabs a connection from the pool
// // which setup is appropriate depends on your application
// struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

// #[async_trait]
// impl<S> FromRequestParts<S> for DatabaseConnection
// where
//     PgPool: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let pool = PgPool::from_ref(state);

//         let conn = pool.acquire().await.map_err(internal_error)?;

//         Ok(Self(conn))
//     }
// }

// async fn using_connection_extractor(
//     DatabaseConnection(mut conn): DatabaseConnection,
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&mut *conn)
//         .await
//         .map_err(internal_error)
// }

// /// Utility function for mapping any error into a `500 Internal Server Error`
// /// response.
// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
