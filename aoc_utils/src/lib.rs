pub mod matrix {
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
}

pub mod polygon {
    pub type Polygon = Vec<Point>;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Point {
        pub row: i64,
        pub col: i64,
    }

    impl Point {
        pub fn new(row: i64, col: i64) -> Self {
            Self { row, col }
        }
        pub fn add(&self, other: &Self) -> Self {
            Self {
                row: self.row + other.row,
                col: self.col + other.col,
            }
        }
    }

    pub fn circumference(polygon: &Polygon) -> f64 {
        let mut circ = polygon
            .windows(2)
            .map(|coords| (coords[0].row, coords[1].row, coords[0].col, coords[1].col))
            .map(|(x1, x2, y1, y2)| (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt())
            .sum();
        let last = polygon.last().unwrap();
        let first = polygon.first().unwrap();
        circ += (((first.row - last.row) * (first.row - last.row)
            + (first.col - last.col) * (first.col - last.col)) as f64)
            .sqrt();
        circ
    }

    pub fn shoelace(polygon: &Polygon) -> f64 {
        let mut polygon_copy = polygon.clone();
        let start_coords = polygon[0];
        polygon_copy.push(start_coords);
        (polygon_copy
            .windows(2)
            .map(|coords| (coords[0].row, coords[0].col, coords[1].row, coords[1].col))
            .map(|(y1, x1, y2, x2)| x1 * y2 - y1 * x2)
            .sum::<i64>() as f64
            * 0.5)
            .abs()
            .ceil()
    }

    pub fn picks_theorem_num_internal_points(area: f64, num_boundary_points: i64) -> i64 {
        println!("A = {area}, b = {num_boundary_points}");
        (area + 1. - num_boundary_points as f64 * 0.5) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::matrix::*;
    use super::polygon::*;

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
    #[test]
    fn test_shoelace() {
        let polygon: Vec<Point> = vec![
            Point { row: 1, col: 2 },
            Point { row: 3, col: 1 },
            Point { row: 2, col: 4 },
            Point { row: 4, col: 6 },
            Point { row: 0, col: 5 },
        ];
        assert_eq!(shoelace(&polygon), 8.);
    }

    #[test]
    fn test_picks() {
        assert_eq!(picks_theorem_num_internal_points(10., 8), 7)
    }
}
