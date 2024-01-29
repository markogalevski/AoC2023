pub type Matrix = Vec<Vec<char>>;

pub fn matrix_transpose(matrix: Matrix) -> Matrix {
    let new_row: Vec<char> = vec!['x'; matrix.len()];
    let mut new_matrix: Matrix = vec![new_row; matrix[0].len()];
    matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, element)| {
            new_matrix[j][i] = *element;
        })
    });
    new_matrix
}

pub fn matrix_invert_rows(matrix: Matrix) -> Matrix {
    let mut new_matrix = matrix.clone();
    new_matrix.reverse();
    new_matrix
}

pub fn matrix_invert_cols(matrix: Matrix) -> Matrix {
    let mut new_matrix = matrix.clone();
    new_matrix.iter_mut().for_each(|row| row.reverse());
    new_matrix
}

pub fn matrix_rotate_ccw(matrix: Matrix) -> Matrix {
    matrix_invert_rows(matrix_transpose(matrix))
}

pub fn matrix_rotate_cw(matrix: Matrix) -> Matrix {
    matrix_invert_cols(matrix_transpose(matrix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_transpose() {
        let a = vec!['a', 'b', 'c', 'd'];
        let b = vec!['e', 'f', 'g', 'h'];
        let matrix: Matrix = vec![a, b];
        let matrix_t = matrix_transpose(matrix);
        let reference: Matrix = vec![
            vec!['a', 'e'],
            vec!['b', 'f'],
            vec!['c', 'g'],
            vec!['d', 'h'],
        ];

        assert_eq!(matrix_t, reference);
    }

    #[test]
    fn test_matrix_invert_rows() {
        let matrix = vec![vec!['a', 'b'], vec!['c', 'd']];
        let matrix_t = matrix_invert_rows(matrix);
        let reference: Matrix = vec![vec!['c', 'd'], vec!['a', 'b']];

        assert_eq!(matrix_t, reference);
    }

    #[test]
    fn test_matrix_invert_cols() {
        let matrix = vec![vec!['a', 'b'], vec!['c', 'd']];
        let matrix_t = matrix_invert_cols(matrix);
        let reference: Matrix = vec![vec!['b', 'a'], vec!['d', 'c']];

        assert_eq!(matrix_t, reference);
    }

    #[test]
    fn test_matrix_ccw() {
        let matrix = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let matrix_t = matrix_rotate_ccw(matrix);
        let reference: Matrix = vec![
            vec!['c', 'f', 'i'],
            vec!['b', 'e', 'h'],
            vec!['a', 'd', 'g'],
        ];

        assert_eq!(matrix_t, reference);
    }

    #[test]
    fn test_matrix_cw() {
        let matrix = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let matrix_t = matrix_rotate_cw(matrix);
        let reference: Matrix = vec![
            vec!['g', 'd', 'a'],
            vec!['h', 'e', 'b'],
            vec!['i', 'f', 'c'],
        ];

        assert_eq!(matrix_t, reference);
    }
}
