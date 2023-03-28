use std::{
    fs::File,
    io::{BufRead, BufReader}, path::Path,
};

pub mod wordfilter;
pub use wordfilter::*;

pub fn file_vec(file_path: &Path, s: &mut WordDict) -> std::io::Result<()> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        s.push(line?);
    }
    Ok(())
}

pub fn chunkify_dict_set(word_dictionary: &WordDict, grid: usize) -> ChunkyWordDict {
    word_dictionary
        .iter()
        .map(|word| chunkify(word, grid))
        .collect()
}

pub fn chunkify(word: &str, grid: usize) -> ChunkyWord {
    word.chars()
        .collect::<Vec<char>>()
        .chunks(grid)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

pub fn len_filter(word_dictionary: &mut WordDict, grid: usize) {
    word_dictionary.retain(|word| word.len() == grid);
    word_dictionary.sort();
}

pub fn stringify_chunky_word(chunky_words: &ChunkyWord) -> String {
    chunky_words.join("")
}

pub fn stringify_chunky_word_list(chunky_words: &ChunkyWordDict) -> String {
    chunky_words
        .iter()
        .map(stringify_chunky_word)
        .collect::<Vec<String>>()
        .join(" ")
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
        prefix.push_str(&word[words]);
    }
    prefix
}
