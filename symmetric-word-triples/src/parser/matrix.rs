use minivec::MiniVec;

use super::token::Token;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]

pub struct TknMatrix {
    matrix: MiniVec<Token>,
    size: usize,
}

impl TknMatrix {
    /// Creates a matrix of Tokens of size `tkns` x `tkns`.
    pub fn new(tkns: usize) -> TknMatrix {
        let matrix_flat: MiniVec<Token> = MiniVec::with_capacity(tkns * tkns);
        TknMatrix {
            matrix: matrix_flat,
            size: tkns,
        }
    }

    // Insert a Token into the matrix at the specified row and column.
    // Panics if the row or column index is out of bounds of the matrix.
    #[inline]
    pub fn insert(&mut self, row: usize, col: usize, tkn: Token) {
        let flat_i = self.to_flat_index(row, col).unwrap();
        self.matrix.insert(flat_i, tkn);
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
        let size = self.len();
        let row_begin = self.to_flat_index(row, 0).unwrap();
        let row_end = row_begin + size;
        self.matrix
            .get(row_begin..row_end)
            .expect("Should not be out of bounds.")
    }

    /// Get the length/size of the matrix.
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Given a row and column index and the size of a square matrix, return the index of the element in the flattened matrix.
    /// Will panic if the row or column index is out of bounds of the matrix.
    #[inline]
    fn to_flat_index(&self, row: usize, col: usize) -> Result<usize, &'static str> {
        let size = self.len();
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
        let size = self.len();
        for row_i in 0..size {
            for col_i in 0..row_i {
                if self.get(row_i, col_i) != self.get(col_i, row_i) {
                    return false;
                }
            }
        }
        true
    }
}

#[inline]
pub fn matrix_is_symmetric<T: PartialEq>(matrix: &[Vec<T>]) -> bool {
    let size = matrix.len();
    if let Some(first_row) = matrix.get(0) {
        if size != first_row.len() {
            return false;
        }
    } else {
        return false;
    }

    for i in 0..size {
        for j in 0..i {
            if matrix[i][j] != matrix[j][i] {
                return false;
            }
        }
    }
    true
}
