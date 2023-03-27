use std::io::Write;
use std::{fs::File, vec};
use symmetric_word_triples::symmetric_word_tuples;

mod matrix;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = 3;
    let dictionary_path = "word_dictionary.txt";
    let mut word_dictionary = vec![];
    parser::file_vec(dictionary_path, &mut word_dictionary)?;
    let result_tuple: Vec<String> = symmetric_word_tuples(&mut word_dictionary, grid)?;

    // Result_tuple into file
    let mut file = File::create("result.txt")?;
    for word in result_tuple {
        writeln!(file, "{}", word)?;
    }
    // TODO: Multithreading!

    Ok(())
}
