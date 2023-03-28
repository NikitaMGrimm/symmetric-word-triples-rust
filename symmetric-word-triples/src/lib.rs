use crate::parser::WordSet;
use fst::Set;
use matrix::matrix_is_symmetric;
use parser::{stringify_chunky_word_list, ChunkyWord, WordDict, WordFilter};
use std::io::Write;

pub mod matrix;
pub mod parser;

type WordTupleDict = Vec<String>;

/// Takes a word dictionary and returns a vector of all the words that form a symmetric matrix
/// with specified grid size.
pub fn symmetric_words(
    word_dictionary: &mut WordDict,
    grid: usize,
    chunk_size: usize,
) -> fst::Result<WordTupleDict> {
    if grid == 0 {
        return Ok(vec![]);
    }
    parser::len_filter(word_dictionary, grid * chunk_size);
    let word_set = Set::from_iter(word_dictionary.clone())?;
    let mut solution_set = vec![];
    let mut solution: Vec<ChunkyWord> = vec![];
    // Define a recursive helper function that performs the backtracking.
    fn backtrack(
        word_set: &WordSet,
        grid: usize,
        chunk_size: usize,
        solution: &mut Vec<ChunkyWord>,
        solution_set: &mut WordDict,
    ) {
        if solution.len() == grid {
            if matrix_is_symmetric(solution) {
                let solution_tuple = stringify_chunky_word_list(solution);
                solution_set.push(solution_tuple);
            }
            return;
        }
        let next_prefix = parser::next_prefix(solution);
        let words = word_set
            .prefix_filter_chunkify(&next_prefix, chunk_size)
            .unwrap();
        for word in words {
            solution.push(word.clone());
            backtrack(word_set, grid, chunk_size, solution, solution_set);
            solution.pop();
        }
    }
    // Start the backtracking with an empty solution.
    backtrack(
        &word_set,
        grid,
        chunk_size,
        &mut solution,
        &mut solution_set,
    );
    println!("    {} solutions found.", solution_set.len());
    Ok(solution_set)
}

/// Calls the symmetric_words function with a range of grid sizes and chunk sizes.
/// Takes a input directory and output directory. Will loop over each file in the input directory
/// and write the results to the output directory.
///
/// Each file gets a folder in the output directory
/// with the same name as the input file. The folder is filled with output files with grid and chunk
/// size in the name.
pub fn dir_symmetric_words_range(
    input_dir: &str,
    output_dir: &str,
    grid_range: (usize, usize),
    chunk_size_range: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the input and output directories exist.
    if !std::path::Path::new(input_dir).exists() {
        panic!("Input directory does not exist.");
    }
    if !std::path::Path::new(output_dir).exists() {
        panic!("Output directory does not exist.");
    }

    let grid_chunk_iter = (grid_range.0..=grid_range.1)
        .flat_map(|g| (chunk_size_range.0..=chunk_size_range.1).map(move |c| (g, c)));

    for (path, (grid_size, chunk_size)) in std::fs::read_dir(input_dir)?
        .filter_map(|p| p.ok())
        .map(|p| p.path())
        .flat_map(|path| grid_chunk_iter.clone().map(move |gc| (path.clone(), gc)))
    {
        let output_path = std::path::Path::new(output_dir);
        let file_name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().replace(" ", "_"))
            .unwrap_or_default();
        let output_file_path = output_path.join(&file_name);
        std::fs::create_dir(&output_file_path).ok();

        let mut word_dictionary = vec![];
        parser::file_vec(path.to_str().unwrap(), &mut word_dictionary)?;

        println!(
            "File name: {file_name} Grid: {}, Chunk size: {}",
            grid_size, chunk_size,
        );
        let result_tuple: Vec<String> =
            symmetric_words(&mut word_dictionary.clone(), grid_size, chunk_size)?;
        if result_tuple.is_empty() {
            continue;
        }
        let file_name = format!("{file_name}_grid{}_chunk{}.txt", grid_size, chunk_size,);
        let output_file_path = output_file_path.join(&file_name);
        if let Ok(file) = std::fs::File::create(&output_file_path) {
            let mut file = std::io::BufWriter::new(file);
            for word in result_tuple {
                writeln!(file, "{}", word)?;
            }
        }
    }
    Ok(())
}

/// Multithreaded version of dir_symmetric_words_range.
/// Takes a input directory and output directory. Will loop over each file in the input directory
/// and write the results to the output directory.
pub fn dir_symmetric_words_range_mt(
    input_dir: &str,
    output_dir: &str,
    grid_range: (usize, usize),
    chunk_size_range: (usize, usize),
    num_threads: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
