use anyhow::Ok;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::get,
    routing::post,
    Router,
};
use search_engine::alpha_only_filter::AlphaOnlyFilter;
use search_engine::*;
use sqlx::postgres::PgPoolOptions;
use std::fs::File;
use std::time::Duration;
use std::{net::SocketAddr, path::Path};
use tantivy::{
    directory::MmapDirectory,
    postings::Postings,
    tokenizer::{LowerCaser, RemoveLongFilter, SimpleTokenizer, TextAnalyzer},
    DocSet, Index,
};
use tower_http::cors::CorsLayer;
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

    let title_field = schema.get_field("title").unwrap();
    let content_field = schema.get_field("content").unwrap();
    let summary_field = schema.get_field("summary").unwrap();
    // let url_field = schema.get_field("url").unwrap();
    // let timestamp_field = schema.get_field("created_time").unwrap();
    // let id_field = schema.get_field("id").unwrap();
    // //  indexing articles in db
    // println!("Indexing articles in db");
    // let mut index_writer = app_state.index.writer(5_000_000_000)?;
    // let articles = sqlx::query_as::<_, Article>(r#"select * from "article""#)
    //     .fetch_all(&app_state.pool)
    //     .await?;

    // for article in articles {
    //     index_writer.add_document(doc!(
    //         title_field => article.title,
    //         content_field => article.content,
    //         summary_field => article.summary,
    //         url_field => article.url,
    //         timestamp_field => article.timestamp,
    //         id_field => article.id
    //     ))?;
    // }
    // index_writer.commit()?;
    // let _ = index_writer.wait_merging_threads();

    // Get term dictionary & posting list
    // get_term_dict_and_posting_list(&index, "title")?;
    // get_term_dict_and_posting_list(&index, "content")?;
    // get_term_dict_and_posting_list(&index, "summary")?;
    // build our application with some routes
    println!("Server is running on port 3030");
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
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

fn get_term_dict_and_posting_list(index: &Index, field: &str) -> anyhow::Result<()> {
    // Get term dictionary & posting list
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let field_instance = index.schema().get_field(field).unwrap();

    let segment_readers_list = searcher.segment_readers();
    println!("Number of segments: {}", segment_readers_list.len());

    let inverted_index = searcher.segment_reader(0).inverted_index(field_instance)?;
    let term_dict = inverted_index.terms().to_owned();
    let mut terms = term_dict.stream().unwrap();
    println!("Getting term dict and posting list of {}", field);
    let file: File = File::create(format!("{}.csv", field))?;
    let mut wtr = csv::Writer::from_writer(file);

    while let Some((term, term_info)) = terms.next() {
        let term_str = std::str::from_utf8(term)?;
        let doc_freq = term_info.doc_freq;
        let mut posting = inverted_index.read_postings_from_terminfo(
            term_info,
            tantivy::schema::IndexRecordOption::WithFreqsAndPositions,
        )?;
        let mut posting_list = Vec::new();
        let mut result = String::new();

        // Getting posting list for each term
        for _ in 0..doc_freq {
            posting.positions(&mut posting_list);
            let doc = posting.doc();
            let result_str = format!("{} -> {:?}, ", doc, posting_list);
            posting_list.clear();
            result.push_str(&result_str);
            posting.advance();
        }

        wtr.write_record([term_str, &doc_freq.to_string(), &result])?;
        result.clear();
    }
    println!("{}: {}", field, term_dict.num_terms());
    Ok(())
}
