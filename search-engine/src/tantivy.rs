// # Basic Example

// This example covers the basic functionalities of
// tantivy.

// We will :
// - define our schema
// - create an index in a directory
// - index a few documents into our index
// - search for the best document matching a basic query
// - retrieve the best document's original content.

// ---
// Importing tantivy...
use serde::Serialize;
use tantivy::collector::TopDocs;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::*;
use tantivy::tokenizer::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;
pub fn query_wrapper(query: String) -> tantivy::Result<Vec<String>> {
    let index_path = TempDir::new()?;

    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("vn_core")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING | STORED);
    schema_builder.add_text_field("title", TEXT);
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("summary", TEXT);
    schema_builder.add_text_field("url", TEXT);
    schema_builder.add_text_field("timestamp", TEXT);

    let schema = schema_builder.build();
    let index: Index = Index::create_in_dir(&index_path, schema.clone())?;
    let vn_tokenizer = VnCore::new();
    index.tokenizers().register("vn_core", vn_tokenizer);
    let mut index_writer = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let content = schema.get_field("content").unwrap();
    let id = schema.get_field("id").unwrap();

    // For convenience, tantivy also comes with a macro to
    // reduce the boilerplate above.
    index_writer.add_document(doc!(
        id => "1",
    title => "Những đồ thiết yếu cần lắp cho ô tô",
    content => "Mời các bác vào tham luận cho vui. cái gì nên or không nên lắp, tốt cho sử dụng lâu dài hay theo sở thích cá nhân
    Em trước - Tucson
    Cam hành trình ( em không lắp cam 360 - phụ thuộc công nghệ quá em không thích )
    Cảm biến as lố "
    ))?;

    // Multivalued field just need to be repeated.
    index_writer.add_document(doc!(
        id => "2",
    title => "Frankenstein",
    title => "Cảnh sát chưa có bằng chứng xác thực, dư luận Hàn Quốc xoay chiều, ủng hộ G-Dragon",
    content => "ủa tức là thằng này nhận tội rồi nhưng cảnh sát quay xe kêu ko đủ bằng chứng nên chưa khép tội được, hòa cả làng hả",
    ))?;

    index_writer.commit()?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    // We now need to acquire a searcher.
    //
    // A searcher points to a snapshotted, immutable version of the index.
    //
    // Some search experience might require more than
    // one query. Using the same searcher ensures that all of these queries will run on the
    // same version of the index.
    //
    // Acquiring a `searcher` is very cheap.
    //
    // You should acquire a searcher every time you start processing a request and
    // and release it right after your query is finished.
    let searcher = reader.searcher();

    // ### Query

    // The query parser can interpret human queries.
    // Here, if the user does not specify which
    // field they want to search, tantivy will search
    // in both title and content.
    let query_parser = QueryParser::for_index(&index, vec![title, content]);

    // `QueryParser` may fail if the query is not in the right
    // format. For user facing applications, this can be a problem.
    // A ticket has been opened regarding this problem.
    let query = query_parser.parse_query(&query)?;

    // A query defines a set of documents, as
    // well as the way they should be scored.
    //

    // We can now perform our query.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    // The actual documents still need to be
    // retrieved from Tantivy's store.
    //
    // Since the content field was not configured as stored,
    // the document returned will only contain
    // a title.
    let mut result: Vec<String> = Vec::new();
    let a = top_docs.len();
    for (_score, doc_address) in top_docs {
        println!("Found {} document(s).", a);
        let retrieved_doc = searcher.doc(doc_address)?;
        result.push(schema.to_json(&retrieved_doc));
    }
    Ok(result)
}
use crate::article::Article;
use crate::VnCoreNLP;
#[derive(Clone, Copy)]

// Tokenizer for search engine, calling VNCoreNLP Java lib
struct VnCore {}

impl Tokenizer for VnCore {
    type TokenStream<'a> = PreTokenizedStream;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        let vncore_nlp = VnCoreNLP::new_new().unwrap();
        let tokens = vncore_nlp
            .pipeline
            .segment(&vncore_nlp.jvm, text.to_string())
            .unwrap();
        let tokenized_string = PreTokenizedString {
            text: text.to_string(),
            tokens: tokens
                .iter()
                .map(|token| Token {
                    text: token.to_string(),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        };
        tokenized_string.into()
    }
}
impl VnCore {
    fn new() -> Self {
        Self {}
    }
}
