# Symmetric Word Triples

This program finds all word triples with a specific property, as shown in the example below:

```
complaint placement intention

com pla int 
pla cem ent
int ent ion
```

The words in each triple can be read either from left to right or from top to bottom.

## Usage

-Add a list of words to a file called `word_dictionary.txt`. Each word should be on a new line.
-Run the program using the cargo run command in your terminal.
-The program will output all word triples with the specified property to a new file called `words_filtered.txt`.

## Example

Suppose `word_dictionary.txt` contains the following words:

```
complaint
placement
intention
apple
orange
banana
```

Running the program will output the following triples to words_filtered.txt:

`complaint placement intention`

Note

This program is written in Rust.