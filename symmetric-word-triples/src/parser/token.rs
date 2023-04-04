use dashmap::DashMap;
use radix_trie::TrieKey;

use super::{matrix::TokenMatrix, wordfilter::Hr};

pub type TknSize = u16;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct TokenWord(pub Vec<Token>);

impl TokenWord {
    pub fn new() -> TokenWord {
        TokenWord(vec![])
    }

    pub fn with_capacity(capacity: usize) -> TokenWord {
        TokenWord(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, tkn: Token) {
        self.0.push(tkn);
    }
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token(pub TknSize);

#[derive(Clone, Debug)]
pub struct Tokens {
    encode: DashMap<String, Token, Hr>,
    decode: DashMap<Token, String, Hr>,
    size: TknSize,
}

impl Tokens {
    /// Create a new `Tokens` instance. Empty with no tokens.
    pub fn new() -> Tokens {
        Tokens {
            encode: DashMap::default(),
            decode: DashMap::default(),
            size: 0,
        }
    }

    /// Insert a String into the `Tokens` instance.
    /// Generates a new `Token` for the String and returns it.
    /// Will return the existing `Token` if the String is already in the `Tokens` instance.
    #[inline]
    pub fn insert(&mut self, s: String) -> Token {
        if self.encode.contains_key(&s) {
            return *self.encode.get(&s).unwrap().value();
        }

        let new_tkn = Token(self.len());
        self.encode.insert(s.clone(), new_tkn);
        self.decode.insert(new_tkn, s);
        self.size += 1;
        new_tkn
    }

    /// Returns the number of tokens in the `Tokens` instance.
    #[inline]
    pub fn len(&self) -> TknSize {
        self.size
    }

    /// Convert a token matrix into a string.
    #[inline]
    pub fn stringify_token_matrix(&self, tkn_matrix: TokenMatrix, chunk_size: usize) -> String {
        let capacity = tkn_matrix.capacity().pow(2) * chunk_size + tkn_matrix.capacity() - 1;
        let mut output = String::with_capacity(capacity); // TODO: Check if this works like this.

        let mut word_rows_it = tkn_matrix.rows();
        let first_word = word_rows_it.next().unwrap();
        output.push_str(&self.stringify_token_word(first_word));
        for word in word_rows_it {
            output.push(' ');
            output.push_str(&self.stringify_token_word(word));
        }
        output
    }

    /// Convert a token word into a string. Panics if one of the tokens in the token word is not valid.
    #[inline]
    pub fn stringify_token_word(&self, tkn_word: &[Token]) -> String {
        let mut stringified = String::new(); // TODO: Make this with_capacity.
        for tkn in tkn_word {
            let word_chunk = self.stringify_token(*tkn).unwrap();
            stringified.push_str(&word_chunk);
        }
        stringified
    }

    /// Convert a token into a string.
    #[inline]
    pub fn stringify_token(&self, tkn: Token) -> Option<String> {
        self.decode.get(&tkn).map(|s| s.value().clone())
    }

    /// Tokenize a str into a token.
    #[inline]
    pub fn tokenize_str(&self, s: &str, chunk_size: usize) -> Option<TokenWord> {
        let mut tkn_word = TokenWord::with_capacity(s.len() / chunk_size);
        for chunk in s.chars().collect::<Vec<char>>().chunks(chunk_size) {
            let chunk_str = chunk.iter().collect::<String>();
            let tkn = *self.encode.get(&chunk_str).unwrap().value();
            tkn_word.push(tkn);
        }
        Some(tkn_word)
    }
}

impl TrieKey for Token {
    #[inline]
    fn encode_bytes(&self) -> Vec<u8> {
        self.0.encode_bytes()
    }
}

impl TrieKey for TokenWord {
    #[inline]
    fn encode_bytes(&self) -> Vec<u8> {
        self.clone()
            .into_iter()
            .flat_map(|tkn| tkn.encode_bytes())
            .collect()
    }
}

impl Extend<String> for Tokens {
    #[inline]
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for s in iter {
            self.insert(s);
        }
    }
}

impl IntoIterator for TokenWord {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Token> for TokenWord {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        let mut word = TokenWord::new();
        word.extend(iter);
        word
    }
}

impl Extend<Token> for TokenWord {
    #[inline]
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        for tkn in iter {
            self.0.push(tkn);
        }
    }
}

impl<'a> From<&'a TokenWord> for &'a [Token] {
    #[inline]
    fn from(val: &'a TokenWord) -> Self {
        &val.0
    }
}

impl From<Token> for TokenWord {
    #[inline]
    fn from(val: Token) -> Self {
        TokenWord(vec![val])
    }
}
