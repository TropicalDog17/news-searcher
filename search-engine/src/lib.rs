use sqlx::postgres::PgPool;
use tantivy::schema::IndexRecordOption;
use tantivy::{
    schema::{Schema, TextFieldIndexing, TextOptions, STRING, TEXT},
    tokenizer::Token,
    Index,
};
pub mod alpha_only_filter;
pub mod article;
pub mod wrapper;
#[derive(Debug, Clone)]
pub struct AppState {
    // pub pool: PgPool,
    pub index: Index,
}
pub fn get_article_schema() -> Schema {
    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("custom")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions)
        .set_fieldnorms(true);
    let text_options = TextOptions::default().set_indexing_options(text_field_indexing);
    let text_option_stored = text_options.clone().set_stored();
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING | TEXT);
    schema_builder.add_text_field("title", text_option_stored.clone());
    schema_builder.add_text_field("content", text_options.clone());
    schema_builder.add_text_field("summary", text_option_stored.clone());
    schema_builder.add_text_field("url", text_option_stored.clone());
    schema_builder.add_text_field("created_time", text_options.clone());

    schema_builder.build()
}

pub fn assert_token(token: &Token, position: usize, text: &str, from: usize, to: usize) {
    assert_eq!(
        token.position, position,
        "expected position {position} but {token:?}"
    );
    assert_eq!(token.text, text, "expected text {text} but {token:?}");
    assert_eq!(
        token.offset_from, from,
        "expected offset_from {from} but {token:?}"
    );
    assert_eq!(token.offset_to, to, "expected offset_to {to} but {token:?}");
}
