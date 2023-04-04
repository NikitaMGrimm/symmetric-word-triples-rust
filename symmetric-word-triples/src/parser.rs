pub mod matrix;
pub mod token;
pub mod wordfilter;

use self::{
    matrix::TokenMatrix,
    token::TokenWord,
    wordfilter::{ChunkyWord, ChunkyWordDict, WordDict},
};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[inline]
pub fn file_vec(file_path: &Path, s: &mut WordDict) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file),
    );
    for line in reader.lines() {
        s.push(line?);
    }
    Ok(())
}

#[inline]
pub fn chunkify_dict(
    word_dictionary: &WordDict,
    grid_size: usize,
    chunk_size: usize,
) -> ChunkyWordDict {
    word_dictionary
        .iter()
        .map(|word| chunkify(word, chunk_size))
        .filter(|word| word.len() == grid_size)
        .collect()
}

#[inline]
pub fn chunkify(word: &str, chunk_size: usize) -> ChunkyWord {
    let mut chunked_word = ChunkyWord::new();
    let mut current_chunk = String::with_capacity(chunk_size);

    for c in word.chars() {
        if current_chunk.len() == chunk_size {
            chunked_word.push(current_chunk);
            current_chunk = String::with_capacity(chunk_size);
        }
        current_chunk.push(c);
    }

    if !current_chunk.is_empty() {
        chunked_word.push(current_chunk);
    }

    chunked_word
}

#[inline]
pub fn len_filter(word_dictionary: &mut WordDict, grid: usize) {
    word_dictionary.retain(|word| word.len() == grid);
    word_dictionary.sort_unstable();
}

#[inline]
pub fn next_prefix(solution_matrix: &TokenMatrix) -> TokenWord {
    let mut prefix = TokenWord::new();
    let words = solution_matrix.len();
    let first_row = solution_matrix.get_row(0);
    if first_row.len() <= words {
        return prefix;
    }
    for word in solution_matrix.rows() {
        let prefix_piece = word.get(words).unwrap_or_else(|| {
            panic!(
                "
        \nCan't calculate the next prefix. Out of bounds.\nWord: {word:?},\nwords: {words}"
            )
        });

        prefix.push(*prefix_piece);
    }
    // println!("{:?}", prefix);
    prefix
}
