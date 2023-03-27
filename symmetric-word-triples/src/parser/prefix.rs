use fst::automaton::{Automaton, Str};
use fst::IntoStreamer;
pub type ChunkyWord = Vec<String>;
pub type WordDict = Vec<String>;
pub type ChunkyWordDict = Vec<ChunkyWord>;
pub type WordSet = fst::Set<Vec<u8>>;

pub trait WordFilter {
    fn prefix_filter(&self, prefix: &str) -> fst::Result<WordDict>;
    fn prefix_filter_chunkify(&self, prefix: &str, grid: usize) -> fst::Result<ChunkyWordDict>;
}

impl WordFilter for WordSet {
    /// Filter the words in the set that begin with the prefix.
    fn prefix_filter(&self, prefix: &str) -> fst::Result<WordDict> {
        // Make a prefix matcher.
        let prefix_match = Str::new(prefix).starts_with();
        // Filter all the words that begin with the prefix.
        let stream = self.search(prefix_match).into_stream();
        // Convert it into a string.
        stream.into_strs()
    }

    /// Filter the words in the set that begin with the prefix and return a set of the chunkified words.
    fn prefix_filter_chunkify(&self, prefix: &str, grid: usize) -> fst::Result<ChunkyWordDict> {
        // Filter the words that begin with the prefix.
        let words = self.prefix_filter(prefix)?;
        // Chunkify the words.
        Ok(super::chunkify_dict_set(&words, grid))
    }
}

/// Takes in the chunkified words up until the n'th row of the grid and it returns
/// the calculated prefix that is needed for the word in the n+1'th row.
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
