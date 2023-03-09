#![feature(test)]

extern crate rand;
extern crate test;

use std::time::Instant;

use linearalgebra::matrix::Matrix;
use rand::Rng;
use test::Bencher;

use numpy::PyArray2;
use pyo3::{types::IntoPyDict, Python};

fn setup(len: usize) -> (Vec<f64>, Vec<f64>) {
    let mut rng = rand::thread_rng();

    let m1: Vec<f64> = (0..len).map(|_| rng.gen_range(0..10) as f64).collect();
    let m2: Vec<f64> = (0..len).map(|_| rng.gen_range(0..10) as f64).collect();

    (m1, m2)
}

#[bench]
fn matrix_simple_multiplication_bench(b: &mut Bencher) {
    let (m1, m2) = setup(3000 * 32);

    let matrix1 = Matrix::new(3000, 32, m1).unwrap();
    let matrix2 = Matrix::new(32, 3000, m2).unwrap();

    b.iter(|| {
        matrix1.simple_product_matrix(&matrix2).unwrap();
    })
}

#[bench]
fn matrix_trivial_multiplication_bench(b: &mut Bencher) {
    let (m1, m2) = setup(3000 * 32);

    let matrix1 = Matrix::new(3000, 32, m1).unwrap();
    let matrix2 = Matrix::new(32, 3000, m2).unwrap();

    b.iter(|| {
        matrix1.trivial_product_matrix(&matrix2).unwrap();
    })
}

#[bench]
fn big_matrix_multiplication_numpy_test(b: &mut Bencher) {
    let (m1, m2) = setup(3000 * 32);

    b.iter(|| {
        Python::with_gil(|py| {
            let np = py.import("numpy").unwrap();
            let locals = [("np", np)].into_py_dict(py);
            locals.set_item("m1", &m1).unwrap();
            locals.set_item("m2", &m2).unwrap();

            let result: &PyArray2<f64> = py
                .eval(
                    "np.matmul(np.reshape(m1, (32,3000)), np.reshape(m2, (3000,32)))",
                    Some(locals),
                    None,
                )
                .unwrap()
                .extract()
                .unwrap();

            Matrix::new(32, 32, result.to_vec().unwrap()).unwrap();
        })
    })
}

#[test]
fn big_matrix_multiplication_test() {
    let pre_random = Instant::now();
    let (m1, m2) = setup(3000 * 32);
    println!("{}", pre_random.elapsed().as_millis());

    let multiplication = Instant::now();
    let matrix1 = Matrix::new(3000, 32, m1).unwrap();
    let matrix2 = Matrix::new(32, 3000, m2).unwrap();

    matrix1.product_matrix(&matrix2).unwrap();

    println!("{}", multiplication.elapsed().as_millis())
}
