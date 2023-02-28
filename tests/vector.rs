
#[cfg(test)]
mod tests {
    extern crate linearalgebra;

    use linearalgebra::vector::*;

    #[test]
    fn length_test() {
        let v = Vector::new(vec![0,0,0]);
        assert_eq!(v.len(), 3)
    }

    #[test]
    fn scalar_test() {
        let v = Vector::new(vec![3,4,5]);

        assert_eq!(v.scale(5).as_vec(), &vec![15,20,25]); // Check scalar
        assert_eq!(v.as_vec(), &vec![3,4,5]); // Preserve old
    }

    #[test]
    fn addition_mismatched_length_test() {
        let v = Vector::new(vec![3,4,5]);
        let v2 = Vector::new(vec![1,3,9,5,1,12,3,12,12,33,12,3,12]);

        assert_eq!(v.add(&v2).is_err(), true);
    }

    #[test]
    fn addition_success_test() {
        let v = Vector::new(vec![3,4,5]);
        let v2 = Vector::new(vec![1,3,9]);

        assert_eq!(v.add(&v2).unwrap().as_vec(), &vec![4,7,14]);
        assert_eq!(v.as_vec(), &vec![3,4,5]); // Preserve old
        assert_eq!(v2.as_vec(), &vec![1,3,9]); // Preserve old
    }

    #[test]
    fn subtraction_mismatched_length_test() {
        let v = Vector::new(vec![3,4,5]);
        let v2 = Vector::new(vec![1,3,9,5,1,12,3,12,12,33,12,3,12]);

        assert_eq!(v.sub(&v2).is_err(), true);
    }

    #[test]
    fn subtraction_success_test() {
        let v = Vector::new(vec![3,4,5]);
        let v2 = Vector::new(vec![1,3,9]);

        assert_eq!(v.sub(&v2).unwrap().as_vec(), &vec![2,1,-4]);
        assert_eq!(v.as_vec(), &vec![3,4,5]); // Preserve old
        assert_eq!(v2.as_vec(), &vec![1,3,9]); // Preserve old
    }
}
