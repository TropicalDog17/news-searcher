use sqlx::postgres::PgPool;
use tantivy::{
    schema::{Schema, TextFieldIndexing, TextOptions, STORED, STRING, TEXT},
    Index,
};

pub mod article;
pub mod vncore;
pub mod wrapper;
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub index: Index,
}
pub fn get_article_schema() -> Schema {
    let text_field_indexing = TextFieldIndexing::default().set_tokenizer("en_stem");
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING);
    schema_builder.add_text_field("title", text_options.clone());
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("summary", TEXT);
    schema_builder.add_text_field("url", TEXT);
    schema_builder.add_text_field("created_time", TEXT);

    schema_builder.build()
}
