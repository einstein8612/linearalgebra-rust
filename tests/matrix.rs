#[cfg(test)]
mod matrix_tests {
    extern crate linearalgebra;

    use linearalgebra::matrix::*;
    use linearalgebra::vector::*;

    #[test]
    fn get_col_success_test() {
        let matrix = Matrix::new(3, 3, vec![
            1, 1, 2,
            3, 4, 5,
            6, 1, 2
        ]).unwrap();

        assert_eq!(matrix.get_col(0).unwrap(), Vector::new(vec![1, 3, 6]))
    }

    #[test]
    fn get_col_fail_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

        assert!(matrix.get_col(6).is_err());
    }

    #[test]
    fn get_cols_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

        assert_eq!(
            matrix.get_cols(),
            vec![
                Vector::new(vec![1, 3, 6]),
                Vector::new(vec![1, 4, 1]),
                Vector::new(vec![2, 5, 2]),
            ]
        )
    }

    #[test]
    fn display_3x3_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

        assert_eq!(
            "┌       ┐\n│ 1 1 2 │\n│ 3 4 5 │\n│ 6 1 2 │\n└       ┘",
            matrix.to_string()
        )
    }

    #[test]
    fn display_1x1_test() {
        let matrix = Matrix::new(1, 1, vec![1]).unwrap();

        assert_eq!("┌   ┐\n│ 1 │\n└   ┘", matrix.to_string())
    }

    #[test]
    fn display_1x3_test() {
        let matrix = Matrix::new(3, 1, vec![1, 2, 3]).unwrap();

        assert_eq!("┌       ┐\n│ 1 2 3 │\n└       ┘", matrix.to_string())
    }

    #[test]
    fn display_3x1_test() {
        let matrix = Matrix::new(1, 3, vec![1, 2, 3]).unwrap();

        assert_eq!("┌   ┐\n│ 1 │\n│ 2 │\n│ 3 │\n└   ┘", matrix.to_string())
    }

    #[test]
    fn display_0_test() {
        let matrix: Matrix<i32> = Matrix::new(0, 0, vec![]).unwrap();

        assert_eq!("┌ ┐\n└ ┘", matrix.to_string())
    }
}
