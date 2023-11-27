use crate::wrapper::query_wrapper;
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, types::Uuid};
use sqlx::{FromRow, PgPool, Row};
pub async fn create_article(
    State(state): State<AppState>,
    payload: Json<CreateArticle>,
) -> (StatusCode, Json<CreateArticleResponse>) {
    let article_id: Uuid = sqlx::query_scalar(
        r#"insert into "article" (title, summary, content, article_url) values ($1, $2, $3, $4) returning article_id"#,
    ).bind(payload.title.clone()).bind(payload.summary.clone()).bind(payload.content.clone()).bind(payload.url.clone()).bind(payload.timestamp.clone()).fetch_one(&state.pool).await.unwrap();

    let article = CreateArticleResponse {
        id: article_id.to_string(),
    };
    (StatusCode::CREATED, Json(article))
}

pub async fn get_article(
    State(state): State<AppState>,
    payload: Json<GetArticle>,
) -> (StatusCode, Json<Article>) {
    // TODO: handle not found article id
    let article = sqlx::query_as::<_, Article>(r#"select * from "article" where article_url = $1"#)
        .bind(payload.article_id.clone())
        .fetch_one(&state.pool)
        .await;
    match article {
        Ok(article) => (StatusCode::CREATED, Json(article)),
        Err(_) => panic!("Not found"),
    }
}
// Query using tantivy
#[derive(Deserialize)]
pub struct QueryArticle {
    query: String,
}
pub async fn query_article(
    State(app_state): State<AppState>,
    payload: Json<QueryArticle>,
) -> (StatusCode, Json<Vec<String>>) {
    let query = payload.query.clone();
    let schema = app_state.index.schema();
    let articles = query_wrapper(app_state.index, query, schema).unwrap();
    let mut result: Vec<String> = Vec::new();
    for article in articles {
        result.push(article);
    }
    (StatusCode::CREATED, Json(result))
}
/// Errors that can happen when using the user repo.
#[derive(Debug)]
enum ArticleError {
    #[allow(dead_code)]
    NotFound,
}
#[derive(Deserialize)]
pub struct CreateArticle {
    title: String,
    summary: String,
    content: String,
    url: String,
    timestamp: String,
}
#[derive(Deserialize)]
pub struct GetArticle {
    article_id: String,
}
// public fields
#[derive(Serialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub url: String,
    pub timestamp: String,
}
#[derive(Serialize)]
pub struct CreateArticleResponse {
    id: String,
}
impl<'r> FromRow<'r, PgRow> for Article {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let article = Article {
            id: row.get("id"),
            title: row.try_get("title").unwrap(),
            summary: row.try_get("summary").unwrap(),
            content: row.try_get("content").unwrap(),
            url: row.try_get("url").unwrap(),
            timestamp: row.get::<DateTime<Utc>, _>("created_time").to_string(),
        };
        Ok(article)
    }
}
