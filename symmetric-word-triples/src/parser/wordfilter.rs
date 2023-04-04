use std::hash::BuildHasherDefault;

use std::sync::Arc;

use ahash::AHasher;

use dashmap::DashMap;

use radix_trie::{Trie, TrieCommon};

use crate::parser::matrix::TokenMatrix;
use crate::parser::{self};

use super::token::{TokenWord, Tokens};

pub type ChunkyWord = Vec<String>;
pub type WordDict = Vec<String>;
pub type ChunkyWordDict = Vec<ChunkyWord>;
pub type WordSet = fst::Set<Vec<u8>>;
pub type WordTupleDict = Vec<String>;

pub type Hr = BuildHasherDefault<AHasher>;

pub trait WordFilter {
    fn symmetric_words_single(
        &self,
        dictionary_word: TokenWord,
    ) -> fst::Result<Vec<Arc<TokenMatrix>>>;
}

pub struct PrefixMap {
    tokens: Tokens,
    trie: Trie<TokenWord, ()>,
    grid_size: usize,
    chunk_size: usize,
    table: DashMap<TokenWord, Vec<Arc<TokenWord>>, Hr>,
    use_table: bool,
}

impl PrefixMap {
    /// Creates a new prefix map with the word dictionary and the grid_ and chunk_size.
    pub fn new(dict: &WordDict, grid_size: usize, chunk_size: usize, use_table: bool) -> PrefixMap {
        let mut tokens = Tokens::new();
        let chunky_dict = parser::chunkify_dict(dict, grid_size, chunk_size);
        let token_dict = chunky_dict
            .iter()
            .map(|chunky| {
                chunky
                    .iter()
                    .map(|chunk| tokens.insert(chunk.clone()))
                    .collect::<TokenWord>()
            })
            .collect::<Vec<_>>();

        let mut trie = Trie::new();
        for tkn_word in token_dict {
            trie.insert(tkn_word, ());
        }
        // println!("{:?}", trie);

        let table: DashMap<_, _, Hr> = DashMap::default();
        PrefixMap {
            tokens,
            trie,
            grid_size,
            chunk_size,
            table,
            use_table,
        }
    }

    #[inline]
    pub fn get(&self, key: &TokenWord) -> Option<Vec<Arc<TokenWord>>> {
        self.table.get(key).map(|v| v.value().clone())
    }

    /// Returns an iterator over all words with the given prefix.
    #[inline]
    pub fn get_prefix_words(&self, prefix: &TokenWord) -> Vec<Arc<TokenWord>> {
        let _use_table = self.use_table;

        let prefixes = self
            .trie
            .get_raw_descendant(prefix)
            .into_iter()
            .flat_map(|subtrie| subtrie.keys())
            .map(|x| Arc::new(x.clone()))
            .collect::<Vec<_>>();

        prefixes
    }

    /// Returns an iterator over all words with the given prefix.
    #[inline]
    pub fn get_prefix_words_table(&self, prefix: &TokenWord) -> Vec<Arc<TokenWord>> {
        // First check if the key is already in the table.

        if let Some(chunky_words) = self.get(prefix) {
            return chunky_words;
        }

        let prefixes = self
            .trie
            .get_raw_descendant(prefix)
            .into_iter()
            .flat_map(|subtrie| subtrie.keys())
            .map(|x| Arc::new(x.clone()))
            .collect::<Vec<_>>();

        self.table.insert(prefix.clone(), prefixes.clone());

        prefixes
    }

    #[inline]
    pub fn stringify_token_matrix(&self, tkn_matrix: TokenMatrix) -> String {
        self.tokens
            .stringify_token_matrix(tkn_matrix, self.chunk_size)
    }

    #[inline]
    pub fn tokenize_word(&self, word: &str) -> TokenWord {
        self.tokens.tokenize_str(word, self.chunk_size).unwrap()
    }
}

impl WordFilter for PrefixMap {
    /// Takes the first word of a matrix and it return all possible solutions with
    /// that word in the first row.
    #[inline]
    fn symmetric_words_single(&self, word: TokenWord) -> fst::Result<Vec<Arc<TokenMatrix>>> {
        if self.grid_size == 0 {
            return Ok(vec![]);
        }
        let mut solution_set = vec![];
        let mut solution_matrix: TokenMatrix = TokenMatrix::new(self.grid_size);
        solution_matrix.push(word).unwrap();

        fn backtrack(
            prefix_map: &PrefixMap,
            solution_matrix: &mut TokenMatrix,
            solution_set: &mut Vec<Arc<TokenMatrix>>,
        ) {
            if solution_matrix.len() == 0 {
                return;
            }
            if solution_matrix.is_full() {
                if solution_matrix.is_symmetric() {
                    solution_set.push(Arc::new(solution_matrix.clone()));
                }
                return;
            }
            let next_prefix = parser::next_prefix(solution_matrix);

            let prefixed_words = if prefix_map.use_table {
                prefix_map.get_prefix_words_table(&next_prefix)
            } else {
                prefix_map.get_prefix_words(&next_prefix)
            };

            for word in prefixed_words {
                solution_matrix.push((*word).clone()).unwrap();
                backtrack(prefix_map, solution_matrix, solution_set);
                solution_matrix.pop();
            }
        }

        backtrack(self, &mut solution_matrix, &mut solution_set);
        Ok(solution_set)
    }
}
