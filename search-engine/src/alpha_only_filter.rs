//! # Example
//! ```rust
//! use search_engine::alpha_only_filter::AlphaOnlyFilter;
//! use tantivy::tokenizer::*;
//! let mut tokenizer = TextAnalyzer::builder(RawTokenizer::default())
//!   .filter(AlphaOnlyFilter)
//!   .build();
//!
//! let mut stream = tokenizer.token_stream("hello there");
//! // is none because the raw filter emits one token that
//! // contains a space
//! assert!(stream.next().is_none());
//!
//! let mut tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
//!   .filter(AlphaOnlyFilter)
//!   .build();
//!
//! let mut stream = tokenizer.token_stream("hello there 💣");
//! assert!(stream.next().is_some());
//! assert!(stream.next().is_some());
//! // the "emoji" is dropped because its not an alphanum
//! assert!(stream.next().is_none());
//! ```
use tantivy::tokenizer::{Token, TokenFilter, TokenStream, Tokenizer};

/// `TokenFilter` that removes all tokens that contain non
/// ascii alphanumeric characters.
#[derive(Clone)]
pub struct AlphaOnlyFilter;

pub struct AlphaOnlyFilterStream<T> {
    tail: T,
}
const SUPPORTED_CHARACTERS: [char; 93] = [
    'a', 'á', 'à', 'ả', 'ã', 'ạ', 'ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ', 'â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ', 'b',
    'c', 'd', 'đ', 'e', 'é', 'è', 'ẻ', 'ẽ', 'ẹ', 'ê', 'ế', 'ề', 'ể', 'ễ', 'ệ', 'f', 'g', 'h', 'i',
    'í', 'ì', 'ỉ', 'ĩ', 'ị', 'j', 'k', 'l', 'm', 'n', 'o', 'ó', 'ò', 'ỏ', 'õ', 'ọ', 'ô', 'ố', 'ồ',
    'ổ', 'ỗ', 'ộ', 'ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ', 'p', 'q', 'r', 's', 't', 'u', 'ú', 'ù', 'ủ', 'ũ',
    'ụ', 'ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự', 'v', 'w', 'x', 'y', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ', 'z',
];

impl<T> AlphaOnlyFilterStream<T> {
    fn predicate(&self, token: &Token) -> bool {
        token
            .text
            .chars()
            .all(|c| SUPPORTED_CHARACTERS.contains(&c))
    }
}

impl TokenFilter for AlphaOnlyFilter {
    type Tokenizer<T: Tokenizer> = AlphaOnlyFilterWrapper<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> AlphaOnlyFilterWrapper<T> {
        AlphaOnlyFilterWrapper(tokenizer)
    }
}

#[derive(Clone)]
pub struct AlphaOnlyFilterWrapper<T>(T);

impl<T: Tokenizer> Tokenizer for AlphaOnlyFilterWrapper<T> {
    type TokenStream<'a> = AlphaOnlyFilterStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        AlphaOnlyFilterStream {
            tail: self.0.token_stream(text),
        }
    }
}

impl<T: TokenStream> TokenStream for AlphaOnlyFilterStream<T> {
    fn advance(&mut self) -> bool {
        while self.tail.advance() {
            if self.predicate(self.tail.token()) {
                return true;
            }
        }

        false
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::alpha_only_filter::AlphaOnlyFilter;
    use crate::assert_token;
    use tantivy::tokenizer::{SimpleTokenizer, TextAnalyzer, Token};
    #[test]
    fn test_alphanum_only() {
        let tokens = token_stream_helper("i am a cat. 我輩は猫である。(1906)");
        assert_eq!(tokens.len(), 4);
        assert_token(&tokens[0], 0, "i", 0, 1);
        assert_token(&tokens[1], 1, "am", 2, 4);
        assert_token(&tokens[2], 2, "a", 5, 6);
        assert_token(&tokens[3], 3, "cat", 7, 10);
    }

    fn token_stream_helper(text: &str) -> Vec<Token> {
        let mut a = TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(AlphaOnlyFilter)
            .build();
        let mut token_stream = a.token_stream(text);
        let mut tokens: Vec<Token> = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);
        tokens
    }
}
