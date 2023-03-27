use fst::Set;
use matrix::matrix_is_symmetric;
use parser::{stringify_chunky_word_list, ChunkyWord, WordDict, WordFilter};

use crate::parser::WordSet;

mod matrix;
mod parser;

type WordTupleDict = Vec<String>;

/// Takes a word dictionary and returns a vector of all the words that form a symmetric matrix
/// with specified grid size.
pub fn symmetric_word_tuples(
    word_dictionary: &mut WordDict,
    grid: usize,
) -> fst::Result<WordTupleDict> {
    if grid == 0 {
        return Ok(vec![]);
    }
    parser::len_filter(word_dictionary, grid * grid);
    // Make the words into a set that is regex searchable.
    let word_set = Set::from_iter(word_dictionary.clone())?;
    let mut solution_set = vec![];
    let mut solution: Vec<ChunkyWord> = vec![];
    // Define a recursive helper function that performs the backtracking.
    fn backtrack(
        word_set: &WordSet,
        grid: usize,
        solution: &mut Vec<ChunkyWord>,
        solution_set: &mut Vec<String>,
    ) {
        // If we have a complete solution, check if it is symmetric and add it to the solution set.
        if solution.len() == grid {
            if matrix_is_symmetric(solution) {
                let solution_tuple = stringify_chunky_word_list(solution);
                println!("Found solution: {}", solution_tuple);
                solution_set.push(solution_tuple);
            }
            return;
        }
        // Get the prefix for the next word.
        let next_prefix = parser::next_prefix(solution);
        // Get all words that begin with this prefix.
        let words = word_set.prefix_filter_chunkify(&next_prefix, grid).unwrap();
        // Iterate over all words that begin with the next prefix.
        for word in words {
            // Check if the word is not already in the solution.
            // Add the word to the solution.
            solution.push(word.clone());
            // Recursively backtrack.
            backtrack(word_set, grid, solution, solution_set);
            // Remove the word from the solution.
            solution.pop();
        }
    }
    // Start the backtracking with an empty solution.
    backtrack(&word_set, grid, &mut solution, &mut solution_set);
    Ok(solution_set)
}
