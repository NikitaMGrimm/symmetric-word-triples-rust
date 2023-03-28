use fst::automaton::{Automaton, Str};
use fst::IntoStreamer;

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
    fn prefix_filter_chunkify(&self, prefix: &str, grid: usize) -> fst::Result<ChunkyWordDict>;
    fn symmetric_words_single(
        &self,
        dictionary_piece: &str,
        grid: usize,
        chunk_size: usize,
    ) -> fst::Result<WordTupleDict>;
}

impl WordFilter for WordSet {
    /// Filter the words in the set that begin with the prefix.
    fn prefix_filter(&self, prefix: &str) -> fst::Result<WordDict> {
        // Make a prefix matcher.
        let prefix_match = Str::new(prefix).starts_with();
        // Filter all the words that begin with the prefix.
        let stream = self.search(prefix_match).into_stream();
        // Convert it into a string.
        stream.into_strs()
    }

    /// Filter the words in the set that begin with the prefix and return a set of the chunkified words.
    fn prefix_filter_chunkify(&self, prefix: &str, grid: usize) -> fst::Result<ChunkyWordDict> {
        // Filter the words that begin with the prefix.
        let words = self.prefix_filter(prefix)?;
        // Chunkify the words.
        Ok(super::chunkify_dict_set(&words, grid))
    }

    /// Takes the first word of a matrix and it return all possible solutions with
    /// that word in the first row.
    fn symmetric_words_single(
        &self,
        first_word: &str,
        grid: usize,
        chunk_size: usize,
    ) -> fst::Result<WordTupleDict> {
        if grid == 0 {
            return Ok(vec![]);
        }
        let mut solution_set = vec![];
        let mut chunky_solution_matrix: Vec<ChunkyWord> = vec![parser::chunkify(first_word, chunk_size)];
        // Define a recursive helper function that performs the backtracking.
        fn backtrack(
            word_set: &WordSet,
            grid_size: usize,
            chunk_size: usize,
            chunky_solution_matrix: &mut Vec<ChunkyWord>,
            solution_set: &mut WordDict,
        ) {
            if chunky_solution_matrix.is_empty() {
                return;
            }
            if chunky_solution_matrix.len() == grid_size {
                if matrix_is_symmetric(chunky_solution_matrix) {
                    let solution = stringify_chunky_word_list(chunky_solution_matrix);
                    solution_set.push(solution);
                }
                return;
            }
            let next_prefix = parser::next_prefix(chunky_solution_matrix);
            let chunkified_words = word_set
                .prefix_filter_chunkify(&next_prefix, chunk_size)
                .unwrap();
            for chunky_word in chunkified_words {
                chunky_solution_matrix.push(chunky_word.clone());
                backtrack(
                    word_set,
                    grid_size,
                    chunk_size,
                    chunky_solution_matrix,
                    solution_set,
                );
                chunky_solution_matrix.pop();
            }
        }
        // Start the backtracking with an empty solution.
        backtrack(
            self,
            grid,
            chunk_size,
            &mut chunky_solution_matrix,
            &mut solution_set,
        );
        Ok(solution_set)
    }
}

