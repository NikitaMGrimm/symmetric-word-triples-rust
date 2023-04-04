use std::io::Write;
use std::sync::Mutex;
use std::{path::Path, sync::Arc};

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc; // Improves performance by 18%

pub mod parser;
use parser::token::TokenWord;
use parser::wordfilter::WordTupleDict;
use rayon::prelude::*;

use crate::parser::wordfilter::{PrefixMap, WordFilter};

pub fn auto_single_sym_word_sol(
    dictionary_file: &Path,
    word: &str,
    grid_size: usize,
    chunk_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let use_table = grid_size > 4;

    let mut word_dictionary = vec![];
    parser::file_vec(dictionary_file, &mut word_dictionary)?;
    parser::len_filter(&mut word_dictionary, grid_size * chunk_size);
    let prefix_map = PrefixMap::new(&word_dictionary, grid_size, chunk_size, use_table);

    println!("Word: {:?}", word);
    let word = prefix_map.tokenize_word(word);
    println!("TknWord: {:?}\n", word);

    let solution_set_word = prefix_map.symmetric_words_single(word)?;

    let solution_set = solution_set_word
        .par_iter()
        .map(|word| prefix_map.stringify_token_matrix((**word).clone()))
        .collect::<Vec<_>>();

    println!("\nSolutions: ");
    // Print the solutions.
    for solution in solution_set {
        println!("{}", solution);
    }

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
        let dir_name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().replace(' ', "_"))
            .unwrap_or_default();
        let output_dir_path = output_dir.join(&dir_name);

        let file = format!("{dir_name}_grid{}_chunk{}.txt", grid_size, chunk_size,);
        let output_file_path = output_dir_path.join(&file);

        // if output_file_path.exists() && output_file_path.metadata().unwrap().len() > 0 {
        //     println!("File \"{file}\" exists already");
        //     continue;
        // }

        std::fs::create_dir(&output_dir_path).ok();

        println!(
            "File name: {file} Grid: {}, Chunk size: {}",
            grid_size, chunk_size,
        );

        let mut result_tuple: Vec<String> =
            symmetric_words_in_file_mt(&path, grid_size, chunk_size)?;

        if result_tuple.is_empty() {
            continue;
        } else {
            result_tuple.sort_unstable();
        }

        if let Ok(file) = std::fs::File::create(&output_file_path) {
            let mut file = std::io::BufWriter::new(file);
            for word in result_tuple {
                writeln!(file, "{}", word)?;
            }
        }
    }
    Ok(())
}

#[inline]
pub fn symmetric_words_in_file_mt(
    file_path: &Path,
    grid_size: usize,
    chunk_size: usize,
) -> fst::Result<WordTupleDict> {
    if grid_size == 0 {
        return Ok(vec![]);
    }
    let use_table = grid_size > 2;

    // Make a dictionary out of the file.
    let mut word_dictionary = vec![];
    parser::file_vec(file_path, &mut word_dictionary)?;
    parser::len_filter(&mut word_dictionary, grid_size * chunk_size);

    let prefix_map = Arc::new(PrefixMap::new(
        &word_dictionary,
        grid_size,
        chunk_size,
        use_table,
    ));

    let word_dictionary = prefix_map.get_prefix_words(&TokenWord::new());

    let size = word_dictionary.len();
    let cur = Arc::new(Mutex::new(0));
    let solution_count = Arc::new(Mutex::new(0));
    let update_freq = (size / 345).max(1);
    let solution_set_file = word_dictionary
        .par_iter()
        .flat_map_iter(|word| {
            let solutions = prefix_map.symmetric_words_single((**word).clone()).unwrap();

            let mut cur = cur.lock().unwrap();
            let mut solution_count = solution_count.lock().unwrap();
            *solution_count += solutions.len();
            *cur += 1;
            if *cur % update_freq == 0 {
                print_status(*cur, size, *solution_count, grid_size, chunk_size);
            }
            solutions
        })
        .collect::<Vec<_>>();
    let cur = cur.lock().unwrap();
    let solution_count = solution_count.lock().unwrap();
    print_status(*cur, size, *solution_count, grid_size, chunk_size);

    let solution_set_file = solution_set_file
        .par_iter()
        .map(|word| prefix_map.stringify_token_matrix((**word).clone()))
        .collect::<Vec<_>>();
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
