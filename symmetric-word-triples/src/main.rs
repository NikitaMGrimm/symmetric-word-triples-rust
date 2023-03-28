use std::io::Write;
use std::thread::available_parallelism;
use std::path::Path;
use std::{fs::File, vec};
use symmetric_word_triples::{symmetric_words, dir_symmetric_words_range};

mod matrix;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text_dir = Path::new("./text");
    let grid_range = (2,8);
    let chunk_size_range = (1,8);
    let input_dir = text_dir.join("input").to_string_lossy().to_string();
    let output_dir = text_dir.join("output").to_string_lossy().to_string();

    let mut threads_available = 1;
    if let Ok(available) = available_parallelism() {
        threads_available = available.get();
    }

    dir_symmetric_words_range(&input_dir, &output_dir, grid_range, chunk_size_range)?;

    Ok(())
}
