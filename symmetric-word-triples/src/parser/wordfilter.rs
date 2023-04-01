use std::hash::{BuildHasher, BuildHasherDefault};
use std::sync::Arc;

use ahash::{AHasher, RandomState};

use dashmap::DashMap;
use fst::automaton::{Automaton, Str};
use fst::{IntoStreamer, Set};

use crate::matrix::matrix_is_symmetric;
use crate::parser;
use crate::parser::stringify_chunky_word_list;
pub type ChunkyWord = Vec<String>;
pub type WordDict = Vec<String>;
pub type ChunkyWordDict = Vec<ChunkyWord>;
pub type WordSet = fst::Set<Vec<u8>>;
pub type WordTupleDict = Vec<String>;

pub trait WordFilter {
    fn prefix_filter(&self, prefix: &str) -> fst::Result<WordDict>;
    fn prefix_filter_chunkify(&self, prefix: &str) -> fst::Result<ChunkyWordDict>;
    fn symmetric_words_single(&self, dictionary_word: &str) -> fst::Result<WordTupleDict>;
    fn symmetric_words_multiple(&self, words: &[String]) -> WordTupleDict;
}

pub struct PrefixMap {
    pub word_set: WordSet,
    grid_size: usize,
    chunk_size: usize,
    table: DashMap<String, Vec<Arc<ChunkyWord>>, BuildHasherDefault<AHasher>>,
}

impl PrefixMap {
    pub fn new(word_dict: WordDict, grid_size: usize, chunk_size: usize) -> PrefixMap {
        let word_set = Set::from_iter(word_dict).expect("Word set should have been made.");
        // Make the dashmap with the aHash hasher.
        let dashmap: DashMap<_, _, BuildHasherDefault<AHasher>> = DashMap::default();
        PrefixMap {
            word_set,
            grid_size,
            chunk_size,
            table: dashmap,
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<Arc<ChunkyWord>>> {
        self.table.get(key).map(|v| v.value().clone())
    }

    pub fn insert_prefix_and_get(&self, key: &str) -> Option<Vec<Arc<ChunkyWord>>> {
        // First check if the key is already in the table.
        if let Some(chunky_words) = self.get(key) {
            return Some(chunky_words);
        }

        let chunky_words = self.prefix_filter_chunkify(key).unwrap();
        let chunky_words = chunky_words
            .into_iter()
            .map(|chunky_word| Arc::new(chunky_word))
            .collect::<Vec<_>>();

        self.table.insert(key.to_string(), chunky_words.clone());
        // for debuggin we check the size of the table.
        // println!("table size: {}", self.table.len());
        Some(chunky_words)
    }
}

impl WordFilter for PrefixMap {
    /// Filter the words in the set that begin with the prefix.
    fn prefix_filter(&self, prefix: &str) -> fst::Result<WordDict> {
        // Make a prefix matcher.
        let prefix_match = Str::new(prefix).starts_with();
        // Filter all the words that begin with the prefix.
        let stream = self.word_set.search(prefix_match).into_stream();
        // Convert it into a string.
        stream.into_strs()
    }

    /// Filter the words in the set that begin with the prefix and return a set of the chunkified words.
    fn prefix_filter_chunkify(&self, prefix: &str) -> fst::Result<ChunkyWordDict> {
        // Filter the words that begin with the prefix.
        let words = self.prefix_filter(prefix)?;
        // Chunkify the words.
        let mut chunkified_dict = super::chunkify_dict_set(&words, self.grid_size, self.chunk_size);
        chunkified_dict.retain(|chunky| chunky.len() == self.grid_size);
        // println!("chunkified_dict: {:?}", chunkified_dict);
        Ok(chunkified_dict)
    }

    /// Takes the first word of a matrix and it return all possible solutions with
    /// that word in the first row.
    fn symmetric_words_single(&self, first_word: &str) -> fst::Result<WordTupleDict> {
        if self.grid_size == 0 {
            return Ok(vec![]);
        }
        let mut solution_set = vec![];
        let mut chunky_solution_matrix: Vec<ChunkyWord> =
            vec![parser::chunkify(first_word, self.chunk_size)];
        // Define a recursive helper function that performs the backtracking.
        fn backtrack(
            prefix_map: &PrefixMap,
            chunky_solution_matrix: &mut Vec<ChunkyWord>,
            solution_set: &mut WordDict,
        ) {
            if chunky_solution_matrix.is_empty() {
                return;
            }
            if chunky_solution_matrix.len() == prefix_map.grid_size {
                if matrix_is_symmetric(chunky_solution_matrix) {
                    let solution = stringify_chunky_word_list(chunky_solution_matrix);
                    solution_set.push(solution);
                }
                return;
            }
            let next_prefix = parser::next_prefix(chunky_solution_matrix);
            // Use the prefix map to get the chunkified words.
            let chunkified_words = prefix_map.insert_prefix_and_get(&next_prefix).unwrap();

            for chunky_word in chunkified_words {
                chunky_solution_matrix.push(chunky_word.to_vec());
                backtrack(prefix_map, chunky_solution_matrix, solution_set);
                chunky_solution_matrix.pop();
            }
        }
        // Start the backtracking with an empty solution.
        backtrack(self, &mut chunky_solution_matrix, &mut solution_set);
        Ok(solution_set)
    }

    fn symmetric_words_multiple(&self, words: &[String]) -> WordTupleDict {
        let mut solution_set = vec![];
        for first_word in words {
            let solutions = self
                .symmetric_words_single(first_word)
                .expect("Should have added a new solution for a word.");
            solution_set.extend(solutions);
        }
        solution_set
    }
}
