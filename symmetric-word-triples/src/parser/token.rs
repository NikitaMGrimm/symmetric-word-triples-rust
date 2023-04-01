use dashmap::DashMap;
use radix_trie::TrieKey;

use super::wordfilter::Hr;

pub type TknSize = u16;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token(pub TknSize);

#[derive(Clone, Debug)]
pub struct Tokens {
    encode: DashMap<String, Token, Hr>,
    decode: DashMap<Token, String, Hr>,
    size: TknSize,
}

impl Tokens {
    /// Create a new `Tkns` instance. Empty with no tokens.
    pub fn new() -> Tokens {
        Tokens {
            encode: DashMap::default(),
            decode: DashMap::default(),
            size: 0,
        }
    }

    /// Insert a String into the `Tkns` instance.
    /// Generates a new `Tkn` for the String and returns it.
    pub fn insert(&self, s: String) {
        let new_tkn = Token(self.len());
        self.encode.insert(s.clone(), new_tkn);
        self.decode.insert(new_tkn, s);
    }

    /// Returns the number of tokens in the `Tkns` instance.
    pub fn len(&self) -> TknSize {
        self.size
    }
}

impl TrieKey for Token {
    #[inline]
    fn encode_bytes(&self) -> Vec<u8> {
        self.0.encode_bytes()
    }
}
