use crate::wrapper::query_wrapper;
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, Row};

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
#[derive(Serialize)]
pub struct QueryArticleResponse {
    data: Vec<String>,
    article_count: usize,
}
// TODO: allow pagination
pub async fn query_article(
    State(app_state): State<AppState>,
    payload: Json<QueryArticle>,
) -> (StatusCode, Json<QueryArticleResponse>) {
    let query = payload.query.clone();
    let schema = app_state.index.schema();
    let (count, articles) = query_wrapper(app_state.index, query, schema).unwrap();
    let mut result: Vec<String> = Vec::new();
    for article in articles {
        result.push(article);
    }
    let result = QueryArticleResponse {
        article_count: count,
        data: result,
    };
    (StatusCode::CREATED, Json(result))
}
/// Errors that can happen when using the user repo.
#[derive(Debug)]
enum ArticleError {
    #[allow(dead_code)]
    NotFound,
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
