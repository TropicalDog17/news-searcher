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
use tantivy::schema::{self, *};
use tantivy::tokenizer::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;
pub fn query_wrapper(index: Index, query: String, schema: Schema) -> tantivy::Result<Vec<String>> {
    let vn_tokenizer = VnCore::default();
    index.tokenizers().register("vn_core", vn_tokenizer);

    let title_field = schema.get_field("title").unwrap();
    let content_field = schema.get_field("content").unwrap();
    let summary_field = schema.get_field("summary").unwrap();
    let url_field = schema.get_field("url").unwrap();
    let timestamp_field = schema.get_field("created_time").unwrap();
    let id_field = schema.get_field("id").unwrap();

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
    let query_parser = QueryParser::for_index(&index, vec![content_field]);

    // `QueryParser` may fail if the query is not in the right
    // format. For user facing applications, this can be a problem.
    // A ticket has been opened regarding this problem.
    let query = query_parser.parse_query(&query)?;

    // A query defines a set of documents, as
    // well as the way they should be scored.
    //

    // We can now perform our query.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    let mut result: Vec<String> = Vec::new();

    let a = top_docs.len();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        result.push(schema.to_json(&retrieved_doc));
    }
    Ok(result)
}
use crate::article::Article;
use crate::VnCoreNLP;
use unicode_segmentation::UnicodeSegmentation; // 1.6.0

#[derive(Clone, Default)]

// Tokenizer for search engine, calling VNCoreNLP Java lib
struct VnCore {
    token: Token,
}
pub struct SimpleTokenStream<'a> {
    text: &'a str,
    token: &'a mut Token,
    segmented_text: Vec<String>,
}

impl Tokenizer for VnCore {
    type TokenStream<'a> = SimpleTokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> SimpleTokenStream<'a> {
        self.token.reset();
        let vncore_nlp = VnCoreNLP::new_new().unwrap();
        let segmented_text = vncore_nlp
            .pipeline
            .segment(&vncore_nlp.jvm, text.to_string())
            .unwrap();
        SimpleTokenStream {
            text,
            token: &mut self.token,
            segmented_text,
        }
    }
}
impl<'a> TokenStream for SimpleTokenStream<'a> {
    fn advance(&mut self) -> bool {
        self.token.text.clear();
        self.token.position = self.token.position.wrapping_add(1);

        // advance based on segmented text
        if self.segmented_text.is_empty() {
            self.token.text = self.segmented_text.remove(0);
            return true;
        }
        false
    }

    fn token(&self) -> &Token {
        self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use tantivy::tokenizer::{SimpleTokenizer, TextAnalyzer, Token};

    use super::VnCore;
    /// This is a function that can be used in tests and doc tests
    /// to assert a token's correctness.
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
    #[test]
    fn test_simple_tokenizer() {
        let tokens = token_stream_helper("Ông Nguyễn Khắc Chúc đang làm việc tại Đại học Quốc gia Hà Nội. Bà Lan, vợ ông Chúc, cũng làm việc tại đây.");
        assert_eq!(tokens.len(), 21);
        assert_token(&tokens[0], 0, "Ông", 0, 4);
        assert_token(&tokens[1], 1, "Nguyễn Khắc Chúc", 5, 22);
        assert_token(&tokens[2], 2, "đang", 23, 28);
        assert_token(&tokens[3], 3, "làm việc", 29, 38);
        assert_token(&tokens[4], 4, "tại", 39, 43);
        assert_token(&tokens[5], 5, "Đại học", 44, 52);
        assert_token(&tokens[6], 6, "Quốc gia", 53, 62);
        assert_token(&tokens[7], 7, "Hà Nội", 63, 69);
        assert_token(&tokens[8], 8, ".", 69, 71);
        assert_token(&tokens[9], 9, "Bà", 72, 75);
        assert_token(&tokens[10], 10, "Lan", 76, 80);
        assert_token(&tokens[11], 11, ",", 80, 82);
        assert_token(&tokens[12], 12, "vợ", 83, 86);
        assert_token(&tokens[13], 13, "ông", 87, 91);
        assert_token(&tokens[14], 14, "Chúc", 92, 96);
        assert_token(&tokens[15], 15, ",", 96, 98);
        assert_token(&tokens[16], 16, "cũng", 99, 103);
        assert_token(&tokens[17], 17, "làm việc", 104, 113);
        assert_token(&tokens[18], 18, "tại", 114, 118);
        assert_token(&tokens[19], 19, "đây", 119, 122);
        assert_token(&tokens[20], 20, ".", 122, 124);
    }

    fn token_stream_helper(text: &str) -> Vec<Token> {
        let mut a = TextAnalyzer::from(VnCore::default());
        let mut token_stream = a.token_stream(text);
        let mut tokens: Vec<Token> = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);
        tokens
    }
}
