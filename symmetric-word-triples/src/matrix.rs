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
