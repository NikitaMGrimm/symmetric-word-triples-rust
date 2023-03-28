use fst::Set;

use parser::{WordFilter, WordTupleDict};
use std::{io::Write, path::Path, sync::Arc, thread::available_parallelism};
use threadpool::ThreadPool;

pub mod matrix;
pub mod parser;
pub mod threadpool;

pub fn auto_single_sym_word_sol(
    input_dictionary: &Path,
    word: &str,
    grid_size: usize,
    chunk_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut word_dictionary = vec![];
    parser::file_vec(&input_dictionary, &mut word_dictionary)?;
    parser::len_filter(&mut word_dictionary, grid_size * chunk_size);
    let word_set = Set::from_iter(word_dictionary.clone())?;
    let solution_set_word = word_set.symmetric_words_single(word, grid_size, chunk_size)?;
    println!("{:?}", solution_set_word);

    Ok(())
}

pub fn auto_dir_sym_word_sol(
    input_dir: &Path,
    output_dir: &Path,
    grid_range: (usize, usize),
    chunk_size_range: (usize, usize),
    multithreading: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut threads_available = 1;
    if multithreading {
        if let Ok(available) = available_parallelism() {
            threads_available = available.get();
        }
    }
    let threadpool = ThreadPool::new(threads_available);

    dir_symmetric_words_range(
        &input_dir,
        &output_dir,
        grid_range,
        chunk_size_range,
        &threadpool,
    )?;

    Ok(())
}

pub fn dir_symmetric_words_range(
    input_dir: &Path,
    output_dir: &Path,
    grid_range: (usize, usize),
    chunk_size_range: (usize, usize),
    threadpool: &ThreadPool,
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
            symmetric_words_in_file_mt(&path, grid_size, chunk_size, &threadpool)?;
        if result_tuple.is_empty() {
            continue;
        } else {
            result_tuple.sort();
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
    grid: usize,
    chunk_size: usize,
    threadpool: &ThreadPool,
) -> fst::Result<WordTupleDict> {
    if grid == 0 {
        return Ok(vec![]);
    }
    // Make a dictionary out of the file.
    let mut word_dictionary = vec![];
    parser::file_vec(file_path, &mut word_dictionary)?;
    parser::len_filter(&mut word_dictionary, grid * chunk_size);
    let word_set = Arc::new(Set::from_iter(word_dictionary.clone())?);

    let mut solution_set_file = vec![];
    let size = word_dictionary.len();
    let (tx, rx) = std::sync::mpsc::channel();
    for word in word_dictionary {
        let tx = tx.clone();
        let word_set = Arc::clone(&word_set);
        threadpool.execute(move || {
            let solution_set_word = word_set.symmetric_words_single(&word, grid, chunk_size);
            tx.send(solution_set_word).unwrap();
        })
    }
    drop(tx);
    let mut cur = 0;
    for receiver in rx {
        solution_set_file.append(&mut receiver?);
        cur += 1;
        if cur % 100 == 0 {
            println!(
                "{clear}    Finished {:.2}% of file. {} solutions found with grid size {} and chunk size {}",
                (cur as f64 / size as f64) * 100.0,
                solution_set_file.len(),
                grid,
                chunk_size,
                clear = "\x1b[1A\x1b[2K",
            );
        }
    }

    println!(
        "{clear}    {} solutions found with grid size {} and chunk size {}",
        solution_set_file.len(),
        grid,
        chunk_size,
        clear = "\x1b[1A\x1b[2K",
    );
    Ok(solution_set_file)
}
