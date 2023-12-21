use anyhow::Ok;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{header::CONTENT_TYPE, request::Parts, HeaderValue, Method, StatusCode},
    routing::get,
    routing::post,
    Router,
};
use search_engine::*;
use search_engine::{alpha_only_filter::AlphaOnlyFilter, article::Article};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::fs::File;
use std::io::Write;
use std::{net::SocketAddr, path::Path};
use std::{path::PathBuf, time::Duration};
use tantivy::{
    directory::MmapDirectory,
    doc,
    termdict::{self, TermDictionary},
    tokenizer::{
        AlphaNumOnlyFilter, AsciiFoldingFilter, LowerCaser, RemoveLongFilter, SimpleTokenizer,
        TextAnalyzer, WhitespaceTokenizer,
    },
    Directory, Index, SegmentReader,
};
use tempfile::TempDir;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mmap: MmapDirectory = MmapDirectory::open(Path::new("index"))?;
    let schema = get_article_schema();
    let index: Index = Index::open_or_create(mmap.clone(), schema.clone())?;
    // tokenizer is defined and registered.
    let tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(RemoveLongFilter::limit(10))
        .filter(LowerCaser)
        .filter(AlphaOnlyFilter)
        .build();
    index.tokenizers().register("custom", tokenizer);
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

    let app_state = AppState {
        pool,
        index: index.clone(),
    };
    //  indexing articles in db
    println!("Indexing articles in db");
    let mut index_writer = app_state.index.writer(5_000_000_000)?;
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
    let _ = index_writer.wait_merging_threads();
    let reader = index.reader()?;
    let seacher = reader.searcher();
    let term_dict = seacher.segment_reader(0).inverted_index(summary_field)?;
    let term_dict = term_dict.terms().to_owned();
    let mut terms = term_dict.stream().unwrap();
    let mut file: File = File::create("summary.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    while let Some((term, term_info)) = terms.next() {
        let term = std::str::from_utf8(term)?;
        let term_freq = term_info.doc_freq;
        wtr.write_record([term, &term_freq.to_string()])?;
    }
    println!("{}", term_dict.num_terms());
    // build our application with some routes
    println!("Server is running on port 3030");
    // let segment = index.new_segment();
    // let segment_reader = SegmentReader::open(&segment)?;
    // let ivt_idx_reader = segment_reader.inverted_index(title_field)?;
    // let term_dict = ivt_idx_reader.terms();
    // let term_list = term_dict.num_terms();
    // println!("Number of terms in title {}", term_list);
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/articles", post(article::create_article))
        .route("/api/articles", get(article::get_article))
        .route("/api/articles/query", post(article::query_article))
        .layer(cors)
        .with_state(app_state);
    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
