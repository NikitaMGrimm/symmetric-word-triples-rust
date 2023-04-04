use std::path::Path;
use symmetric_word_triples::{auto_single_sym_word_sol, dir_symmetric_words_range};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text_dir = Path::new("./data");
    let grid_range = (3, 3);
    let chunk_size_range = (3, 3);
    let input_dir = text_dir.join("input");
    let output_dir = text_dir.join("output");

    dir_symmetric_words_range(&input_dir, &output_dir, grid_range, chunk_size_range)?;
    auto_single_sym_word_sol(&input_dir.join("words_alpha.txt"), "gladiator", 3, 3)?;

    Ok(())
}
