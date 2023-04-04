use std::fmt::Debug;

use super::token::{Token, TokenWord};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct TokenMatrix {
    matrix: Vec<Token>,
    capacity: usize,
}

impl TokenMatrix {
    /// Creates a matrix of Tokens of size `tkns` x `tkns`.
    pub fn new(tkns: usize) -> TokenMatrix {
        let matrix_flat: Vec<Token> = Vec::with_capacity(tkns * tkns);
        TokenMatrix {
            matrix: matrix_flat,
            capacity: tkns,
        }
    }

    /// Insert a Token into the matrix at the specified row and column.
    /// Panics if the row or column index is out of bounds of the matrix.
    #[inline]
    pub fn insert(&mut self, row: usize, col: usize, tkn: Token) {
        let flat_i = self.to_flat_index(row, col).unwrap();
        self.matrix.insert(flat_i, tkn);
    }

    /// Push a token word into the matrix.
    #[inline]
    pub fn push(&mut self, word: TokenWord) -> Result<(), String> {
        let word_len = word.0.len();
        let matrix_len = self.capacity();
        if word_len > matrix_len {
            let error = format!(
                "The word has size {word_len} and the matrix only allows words of size {matrix_len}! Word: {:?}",
                word,
            );
            return Err(error);
        }

        self.try_extend(word.into_iter())
    }

    /// Pop a row from the matrix.
    /// Will be unpredictable if you have used insert to insert into an unused row.
    #[inline]
    pub fn pop(&mut self) -> Option<Vec<Token>> {
        let filled_rows = self.len();
        if filled_rows == 0 {
            return None;
        }
        let popped_row = self.get_row(filled_rows - 1).to_vec();
        let num_to_pop = popped_row.len();
        for _ in 0..num_to_pop {
            self.matrix.pop();
        }

        Some(popped_row)
    }

    /// Returns the Token in the specified row and column.
    /// Panics if the row or column index is out of bounds of the matrix.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<Token> {
        let flat_i = self.to_flat_index(row, col).unwrap();
        self.matrix.get(flat_i).copied()
    }

    /// Get a slice of the specified row of the matrix given the row index.
    /// Panics if the row index is out of bounds of the matrix.
    #[inline]
    pub fn get_row(&self, row: usize) -> &[Token] {
        let size = self.capacity();
        let row_begin = self.to_flat_index(row, 0).unwrap();
        let row_end = row_begin + size;
        self.matrix
            .get(row_begin..row_end)
            .expect("Should not be out of bounds.")
    }

    /// Get the length/size of the matrix.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of filled rows in the matrix.
    #[inline]
    pub fn len(&self) -> usize {
        self.matrix.len() / self.capacity()
    }

    /// Checks if the matrix is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Given a row and column index and the size of a square matrix, return the index of the element in the flattened matrix.
    /// Will panic if the row or column index is out of bounds of the matrix.
    #[inline]
    fn to_flat_index(&self, row: usize, col: usize) -> Result<usize, &'static str> {
        let size = self.capacity();
        match (row >= size, col >= size) {
            (true, true) => Err("Row and column out of bounds."),
            (true, false) => Err("Row out of bounds."),
            (false, true) => Err("Column out of bounds."),
            (false, false) => Ok(row * size + col),
        }
    }

    /// Checks if the token matrix is symmetric.
    #[inline]
    pub fn is_symmetric(&self) -> bool {
        let size = self.capacity();
        for row_i in 0..size {
            for col_i in 0..row_i {
                if self.get(row_i, col_i) != self.get(col_i, row_i) {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    fn try_extend<T>(&mut self, word: T) -> Result<(), String>
    where
        T: IntoIterator<Item = Token> + ExactSizeIterator + Debug,
    {
        let word_len = word.len();
        let matrix_size = self.capacity();
        if word_len > matrix_size {
            let error = format!(
                "Size of the word: {word_len}. Size of the matrix: {matrix_size}. Word: \"{word:?}\" too big!");
            return Err(error);
        }

        self.matrix.extend(word);
        Ok(())
    }

    /// Returns an iterator over the rows of the matrix.
    #[inline]
    pub fn rows(&self) -> impl Iterator<Item = &[Token]> {
        self.matrix.chunks(self.capacity())
    }
}
