use sqlx::postgres::PgPool;
use tantivy::schema::IndexRecordOption;
use tantivy::{
    schema::{Schema, TextFieldIndexing, TextOptions, STORED, STRING, TEXT},
    Index,
};
pub mod article;
pub mod wrapper;
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub index: Index,
}
pub fn get_article_schema() -> Schema {
    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("custom")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("summary", TEXT | STORED);
    schema_builder.add_text_field("url", TEXT | STORED);
    schema_builder.add_text_field("created_time", TEXT);

    schema_builder.build()
}
