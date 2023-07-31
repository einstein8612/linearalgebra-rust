#[cfg(test)]
mod vector_tests {
    extern crate linearalgebra;

    use linearalgebra::vector::*;

    #[test]
    fn length_test() {
        let v = Vector::new(vec![0, 0, 0]);
        assert_eq!(v.len(), 3)
    }

    #[test]
    fn scalar_test() {
        let v = Vector::new(vec![3, 4, 5]);

        assert_eq!(v.scale(5).as_vec(), &vec![15, 20, 25]); // Check scalar
        assert_eq!(v.as_vec(), &vec![3, 4, 5]); // Preserve old
    }

    #[test]
    fn addition_mismatched_length_test() {
        let v = Vector::new(vec![3, 4, 5]);
        let v2 = Vector::new(vec![1, 3, 9, 5, 1, 12, 3, 12, 12, 33, 12, 3, 12]);

        assert_eq!(v.add(&v2).is_err(), true);
    }

    #[test]
    fn addition_success_test() {
        let v = Vector::new(vec![3, 4, 5]);
        let v2 = Vector::new(vec![1, 3, 9]);

        assert_eq!(v.add(&v2).unwrap().as_vec(), &vec![4, 7, 14]);
        assert_eq!(v.as_vec(), &vec![3, 4, 5]); // Preserve old
        assert_eq!(v2.as_vec(), &vec![1, 3, 9]); // Preserve old
    }

    #[test]
    fn subtraction_mismatched_length_test() {
        let v = Vector::new(vec![3, 4, 5]);
        let v2 = Vector::new(vec![1, 3, 9, 5, 1, 12, 3, 12, 12, 33, 12, 3, 12]);

        assert_eq!(v.sub(&v2).is_err(), true);
    }

    #[test]
    fn subtraction_success_test() {
        let v = Vector::new(vec![3, 4, 5]);
        let v2 = Vector::new(vec![1, 3, 9]);

        assert_eq!(v.sub(&v2).unwrap().as_vec(), &vec![2, 1, -4]);
        assert_eq!(v.as_vec(), &vec![3, 4, 5]); // Preserve old
        assert_eq!(v2.as_vec(), &vec![1, 3, 9]); // Preserve old
    }

    #[test]
    fn zeroes_test() {
        let v = Vector::<i32>::zeroes(10);

        assert_eq!(v.as_vec(), &vec![0i32; 10]);
    }

    #[test]
    fn ones_test() {
        let v = Vector::<i32>::ones(10);

        assert_eq!(v.as_vec(), &vec![1i32; 10]);
    }

    #[test]
    fn unit_vector_success_test() {
        let v = Vector::<i32>::cartesian_unit_vector(1, 5).unwrap();

        assert_eq!(v.as_vec(), &vec![1, 0, 0, 0, 0]);
    }

    #[test]
    fn unit_vector_fail_test() {
        let v = Vector::<i32>::cartesian_unit_vector(6, 5);

        assert!(v.is_err())
    }

    #[test]
    fn dot_success_test() {
        let v = Vector::new(vec![3, 4, 5]);
        let v2 = Vector::new(vec![1, 3, 9]);

        assert_eq!(v.dot(&v2).unwrap(), 60);
    }

    #[test]
    fn dot_fail_test() {
        let v = Vector::new(vec![3, 4, 5]);
        let v2 = Vector::new(vec![1, 3, 9, 1]);

        assert!(v.dot(&v2).is_err());
    }

    #[test]
    fn abs_test() {
        let v = Vector::new(vec![3.0, 4.0]);

        assert_eq!(v.abs(), 5.0);
    }

    #[test]
    fn apply_test() {
        let mut v = Vector::new(vec![3.0, 4.0]);
        v.apply(|&x| x * 2.0);

        assert_eq!(&vec![6.0, 8.0], v.as_vec());
    }

    #[test]
    fn expand_column_test() {
        let v = Vector::new(vec![3.0, 4.0]);
        let expanded = v.expand(4, Axis::Column);

        assert_eq!((2, 4), expanded.shape());
        assert_eq!(
            &vec![3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0],
            expanded.as_vec()
        );
    }

    #[test]
    fn expand_row_test() {
        let v = Vector::new(vec![3.0, 4.0]);
        let expanded = v.expand(4, Axis::Row);

        assert_eq!((4, 2), expanded.shape());
        assert_eq!(
            &vec![3.0, 4.0, 3.0, 4.0, 3.0, 4.0, 3.0, 4.0],
            expanded.as_vec()
        );
    }

}
