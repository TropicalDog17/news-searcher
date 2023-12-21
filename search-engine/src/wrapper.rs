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
use tantivy::collector::TopDocs;
use tantivy::collector::{Count, MultiCollector};
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, ReloadPolicy};

pub fn query_wrapper(
    index: Index,
    query: String,
    schema: Schema,
) -> tantivy::Result<(usize, Vec<String>)> {
    let title_field = schema.get_field("title").unwrap();
    let content_field = schema.get_field("content").unwrap();
    let summary_field = schema.get_field("summary").unwrap();
    // let url_field = schema.get_field("url").unwrap();
    // let timestamp_field = schema.get_field("created_time").unwrap();
    // let id_field = schema.get_field("id").unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    // ### Query

    // The query parser can interpret human queries.
    // Here, if the user does not specify which
    // field they want to search, tantivy will search
    // in both title and content.
    let query_parser =
        QueryParser::for_index(&index, vec![title_field, summary_field, content_field]);
    // `QueryParser` may fail if the query is not in the right
    // format. For user facing applications, this can be a problem.
    // A ticket has been opened regarding this problem.
    let query = query_parser.parse_query(&query)?;

    // A query defines a set of documents, as
    // well as the way they should be scored.
    //

    // We can now perform our query.
    let mut collectors = MultiCollector::new();
    let top_docs_handle = collectors.add_collector(TopDocs::with_limit(25000));
    let count_handle = collectors.add_collector(Count);
    let mut multi_fruit = searcher.search(&query, &collectors)?;
    let mut result: Vec<String> = Vec::new();

    let top_docs = top_docs_handle.extract(&mut multi_fruit);
    let count = count_handle.extract(&mut multi_fruit);

    println!("Total hits: {}", count);
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        result.push(schema.to_json(&retrieved_doc));
    }
    Ok((count, result))
}
