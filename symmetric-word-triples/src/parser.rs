use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub mod prefix;
pub use prefix::*;

/// Reads the file and returns a vector of strings where each string is a line in the file.
pub fn file_vec(file_path: &str, s: &mut WordDict) -> std::io::Result<()> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        s.push(line?);
    }
    Ok(())
}

/// Split the words in a word dictionary into chunks of size `grid`.
pub fn chunkify_dict_set(word_dictionary: &WordDict, grid: usize) -> ChunkyWordDict {
    word_dictionary
        .iter()
        .map(|word| chunkify(word, grid))
        .collect()
}

/// Split the words in a word dictionary into chunks of size `grid`.
pub fn chunkify_dict_vec(word_dictionary: &WordDict, grid: usize) -> Vec<ChunkyWord> {
    word_dictionary
        .iter()
        .map(|word| chunkify(word, grid))
        .collect()
}

/// Split a word into chunks of size `grid`.
pub fn chunkify(word: &str, grid: usize) -> ChunkyWord {
    word.chars()
        .collect::<Vec<char>>()
        .chunks(grid)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

/// Filters out all words that don't match the length and sort the words.
pub fn len_filter(word_dictionary: &mut WordDict, grid: usize) {
    word_dictionary.retain(|word| word.len() == grid);
    word_dictionary.sort();
}

/// Chunky word is converted into a String.
pub fn stringify_chunky_word(chunky_words: &ChunkyWord) -> String {
    chunky_words.join("")
}

/// List of chunky words are converted into one string.
pub fn stringify_chunky_word_list(chunky_words: &ChunkyWordDict) -> String {
    chunky_words
        .iter()
        .map(stringify_chunky_word)
        .collect::<Vec<String>>()
        .join(" ")
}
