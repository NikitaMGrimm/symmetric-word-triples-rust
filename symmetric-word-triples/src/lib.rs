use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc; // Improves performance by 18%

use parser::{WordFilter, WordTupleDict};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::parser::PrefixMap;

pub mod matrix;
pub mod parser;

pub fn auto_single_sym_word_sol(
    input_dictionary: &Path,
    word: &str,
    grid_size: usize,
    chunk_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut word_dictionary = vec![];
    parser::file_vec(input_dictionary, &mut word_dictionary)?;
    parser::len_filter(&mut word_dictionary, grid_size * chunk_size);
    let prefix_map = PrefixMap::new(word_dictionary, grid_size, chunk_size);
    let solution_set_word = prefix_map.symmetric_words_single(word)?;
    println!("{:?}", solution_set_word);

    Ok(())
}

pub fn auto_dir_sym_word_sol(
    input_dir: &Path,
    output_dir: &Path,
    grid_range: (usize, usize),
    chunk_size_range: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    dir_symmetric_words_range(input_dir, output_dir, grid_range, chunk_size_range)?;

    Ok(())
}

pub fn dir_symmetric_words_range(
    input_dir: &Path,
    output_dir: &Path,
    grid_range: (usize, usize),
    chunk_size_range: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the input and output directories exist.
    if !input_dir.exists() {
        panic!("Input directory does not exist.");
    }
    if !output_dir.exists() {
        panic!("Output directory does not exist.");
    }
    let grid_chunk_iter = (grid_range.0..=grid_range.1)
        .flat_map(|g| (chunk_size_range.0..=chunk_size_range.1).map(move |c| (g, c)));
    for (path, (grid_size, chunk_size)) in std::fs::read_dir(input_dir)?
        .filter_map(|p| p.ok())
        .map(|p| p.path())
        .flat_map(|path| grid_chunk_iter.clone().map(move |gc| (path.clone(), gc)))
    {
        let file_name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().replace(' ', "_"))
            .unwrap_or_default();
        let output_file_path = output_dir.join(&file_name);
        std::fs::create_dir(&output_file_path).ok();

        // missing
        println!(
            "File name: {file_name} Grid: {}, Chunk size: {}",
            grid_size, chunk_size,
        );

        let mut result_tuple: Vec<String> =
            symmetric_words_in_file_mt(&path, grid_size, chunk_size)?;
        if result_tuple.is_empty() {
            continue;
        } else {
            result_tuple.sort_unstable();
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

pub fn symmetric_words_in_file_mt(
    file_path: &Path,
    grid_size: usize,
    chunk_size: usize,
) -> fst::Result<WordTupleDict> {
    if grid_size == 0 {
        return Ok(vec![]);
    }

    // Make a dictionary out of the file.
    let mut word_dictionary = vec![];
    parser::file_vec(file_path, &mut word_dictionary)?;
    parser::len_filter(&mut word_dictionary, grid_size * chunk_size);

    let prefix_map = Arc::new(PrefixMap::new(
        word_dictionary.clone(),
        grid_size,
        chunk_size,
    ));
    let size = word_dictionary.len();
    let cur = Arc::new(Mutex::new(0));
    let solution_count = Arc::new(Mutex::new(0));
    let update_freq = size / 345;
    let solution_set_file: Vec<_> = word_dictionary
        .par_iter()
        .flat_map_iter(|words| {
            let solutions = prefix_map.symmetric_words_single(words).unwrap();
            let mut cur = cur.lock().unwrap();
            let mut solution_count = solution_count.lock().unwrap();
            *solution_count += solutions.len();
            *cur += 1;
            if *cur % update_freq == 0 {
                print_status(*cur, size, *solution_count, grid_size, chunk_size);
            }
            solutions
        })
        .collect();
    let cur = cur.lock().unwrap();
    let solution_count = solution_count.lock().unwrap();
    print_status(*cur, size, *solution_count, grid_size, chunk_size);
    Ok(solution_set_file)
}

fn print_status(
    cur: usize,
    size: usize,
    solution_count: usize,
    grid_size: usize,
    chunk_size: usize,
) {
    println!(
        "\x1b[1A\x1b[2K    Finished {:.2}% of file. {} solutions found with grid size {} and chunk size {}",
        (cur as f64 / size as f64) * 100.0,
        solution_count,
        grid_size,
        chunk_size,
    );
}
