use std::path::Path;
use std::thread::available_parallelism;

use symmetric_word_triples::{dir_symmetric_words_range, auto_dir_sym_word_sol, auto_single_sym_word_sol};

mod matrix;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text_dir = Path::new("./data");
    let grid_range = (3, 3);
    let chunk_size_range = (3, 3);
    let input_dir = text_dir.join("input");
    let output_dir = text_dir.join("output");

    //auto_dir_sym_word_sol(&input_dir, &output_dir, grid_range, chunk_size_range, true)?;

    auto_single_sym_word_sol(&input_dir, "gladiator", 3, 3)?;

    Ok(())
}
