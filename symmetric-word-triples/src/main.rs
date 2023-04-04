use std::path::Path;

use symmetric_word_triples::{dir_symmetric_words_range, auto_single_sym_word_sol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text_dir = Path::new("./data");
    let grid_range = (3, 3);
    let chunk_size_range = (3, 3);
    let input_dir = text_dir.join("input");
    let output_dir = text_dir.join("output");

    dir_symmetric_words_range(&input_dir, &output_dir, grid_range, chunk_size_range)?;
    auto_single_sym_word_sol(&input_dir.join("words_alpha.txt"), "gladiator", 3, 3)?;

    // TODO: Use hashmaps somehow?? 
        // Maybe every time we get the next_prefix, we add it into a hashmap (key: next_prefix, value: Option<set of words with prefix>)
        // Because we might get the same necessary prefix multiple times, we can just retrieve the set if it already exists!
        // To reduce the hashtables size: Make another hashset with all prefixes that have no words.
        // Every time before we insert into the hashmap, we check if it has any words, if not -> hashset
            // If we have a prefix, we first check if it is in the hashset, if it is, skip it, (no solutions)
            // if it isnt in the hashset, check if its in the hashmap, if it is, iterate over
    // TODO: Priority of making the cache is higher than reading the cache
    // TODO: Make a prerun where you cache the prefixes of the first word before finding solutions.
    // TODO: Diagonal optimization? (If you have a solution, you can vary the diagonal chunks to get more solutions)

    // TODO: dashset for solution_set_file (or similar)
        // Instantly add into hashmap if you have a solution instead of collecting and appending.
        // Also: Somehow dump the results into a file during the computation (instead of waiting for the whole thing to finish) (high ram usage)

    // TODO: In the hot part, iterate over a finished chunky word dict instead of calling chunkify for each word.
        // Pretty much just pre-chunkify everything for each file beforehand and only accept chunky words everywhere.

    // get should return customoption<option, maybe new enum for option that can be unwrapped to differentiate between not found and empty set
    // make new matrix struct for chunky to avoid indirection, make methods to work on flat vector with offsets
    Ok(())
}
