#[cfg(test)]
mod matrix_tests {
    extern crate linearalgebra;

    use std::ops::Index;

    use linearalgebra::matrix::*;
    use linearalgebra::vector::*;

    #[test]
    fn get_col_success_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

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
    fn get_row_success_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

        assert_eq!(matrix.get_row(0).unwrap(), Vector::new(vec![1, 1, 2]))
    }

    #[test]
    fn get_row_fail_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

        assert!(matrix.get_row(6).is_err());
    }

    #[test]
    fn get_rows_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();

        assert_eq!(
            matrix.get_rows(),
            vec![
                Vector::new(vec![1, 1, 2]),
                Vector::new(vec![3, 4, 5]),
                Vector::new(vec![6, 1, 2]),
            ]
        )
    }

    #[test]
    fn matrix_vector_product_fail_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();
        let vector = Vector::new(vec![1, 4, 1, 1]);

        assert!(matrix.product_vector(&vector).is_err())
    }

    #[test]
    fn matrix_vector_product_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();
        let vector = Vector::new(vec![1, 4, 1]);

        assert_eq!(
            matrix.product_vector(&vector).unwrap(),
            Vector::new(vec![7, 24, 12])
        )
    }

    #[test]
    fn matrix_matrix_product_test() {
        let matrix = Matrix::new(3, 3, vec![1, 1, 2, 3, 4, 5, 6, 1, 2]).unwrap();
        let matrix2 = Matrix::new(3, 3, vec![1, 9, 2, 7, 1, 5, 3, 8, 2]).unwrap();

        assert_eq!(
            matrix.product_matrix(&matrix2).unwrap().as_vec(),
            &vec![14, 26, 11, 46, 71, 36, 19, 71, 21]
        )
    }

    #[test]
    fn transpose_nonsquare_test() {
        let matrix = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(matrix.transpose().as_vec(), &vec![1, 4, 2, 5, 3, 6])
    }

    #[test]
    fn transpose_square_test() {
        let matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(
            matrix.transpose().as_vec(),
            &vec![1, 4, 7, 2, 5, 8, 3, 6, 9]
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
    fn display_unequal_length_3x2_test() {
        let matrix = Matrix::new(2, 3, vec![1, 20, 3, 1, 0, 9]).unwrap();

        assert_eq!("┌       ┐\n│ 1  20 │\n│ 3  1  │\n│ 0  9  │\n└       ┘", matrix.to_string())
    }

    #[test]
    fn display_0_test() {
        let matrix: Matrix<i32> = Matrix::new(0, 0, vec![]).unwrap();

        assert_eq!("┌ ┐\n└ ┘", matrix.to_string())
    }

    #[test]
    fn indexed_access_test() {
        let matrix: Matrix<i32> = Matrix::new(3, 3, vec![4,5,1,2,3,1,5,1,9]).unwrap();

        assert_eq!(3, matrix[(1,1)]);
        assert_eq!(3, *matrix.index((1,1)))
    }

    #[test]
    fn apply_test() {
        let mut matrix: Matrix<i32> = Matrix::new(3, 3, vec![4,5,1,2,3,1,5,1,9]).unwrap();

        assert_eq!(3, matrix[(1,1)]);
        matrix.apply(|x| *x = *x*2);
        assert_eq!(6, matrix[(1,1)]);
    }

    #[test]
    fn supplier_test() {
        let matrix: Matrix<i32> = Matrix::new_of_supplier(3, 3, || 10).unwrap();
        assert_eq!(&vec![10;9], matrix.as_vec())
    }

    #[test]
    fn add_test() {
        let matrix: Matrix<i32> = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();
        let matrix_2: Matrix<i32> = Matrix::new(2, 2, vec![3,2,1,4]).unwrap();
        assert_eq!(&vec![4,4,4,8], matrix.add(&matrix_2).unwrap().as_vec())
    }

    #[test]
    fn sub_test() {
        let matrix: Matrix<i32> = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();
        let matrix_2: Matrix<i32> = Matrix::new(2, 2, vec![3,2,1,4]).unwrap();

        assert_eq!(&vec![-2, 0, 2, 0], matrix.sub(&matrix_2).unwrap().as_vec())
    }

    #[test]
    fn sum_test() {
        let matrix: Matrix<i32> = Matrix::new(2, 2, vec![9,2,3,4]).unwrap();
        assert_eq!(18, matrix.sum())
    }

    #[test]
    fn test_test() {
        let matrix: Matrix<i32> = Matrix::new(2, 2, vec![-4, -6, -2, 6]).unwrap();
        let matrix2: Matrix<i32> = Matrix::new(2, 2, vec![0, 2, -1, -2]).unwrap();

        println!("{}", matrix2.product_matrix(&matrix).unwrap())
    }
}
