use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub mod wordfilter;
pub use wordfilter::*;

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

pub fn chunkify_dict_set(
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

pub fn len_filter(word_dictionary: &mut WordDict, grid: usize) {
    word_dictionary.retain(|word| word.len() == grid);
    word_dictionary.sort_unstable();
}

pub fn stringify_chunky_word(chunky_words: &ChunkyWord) -> String {
    chunky_words.join("")
}

pub fn stringify_chunky_word_list(chunky_words: &ChunkyWordDict) -> String {
    // Chunky word count * chunk count * character count + spaces (chunky word count - 1)
    let capacity = chunky_words.len() * (chunky_words[0].len() * chunky_words[0][0].len() + 1) - 1;
    let mut output = String::with_capacity(capacity); // assuming an average chunky word length of 10 characters
    output.push_str(&stringify_chunky_word(&chunky_words[0]));
    for chunky_word in &chunky_words[1..] {
        output.push(' ');
        output.push_str(&stringify_chunky_word(chunky_word));
    }
    output
}

pub fn next_prefix(chunky_word_vec: &[ChunkyWord]) -> String {
    let mut prefix = String::new();
    let words = chunky_word_vec.len();
    if let Some(word) = chunky_word_vec.get(0) {
        if word.len() <= words {
            return prefix;
        }
    }

    for word in chunky_word_vec.iter().take(words) {
        prefix.push_str(word.get(words).unwrap_or_else(|| panic!("
\nCan't calculate the next prefix. Out of bounds.\nWord: {word:?},\nwords: {words},\nchunky: {chunky_word_vec:?}")));
    }
    // println!("{:?}", prefix);
    prefix
}
