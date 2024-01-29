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
}
