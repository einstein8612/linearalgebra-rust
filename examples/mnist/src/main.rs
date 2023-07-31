extern crate linearalgebra;
extern crate rand;

use std::time::SystemTime;
use std::{fs::File, io::stdin};

use linearalgebra::vector::Axis;
use linearalgebra::{matrix::Matrix, vector::Vector};
use rand::Rng;

const ITERATIONS: i32 = 100;
const LEARNING_RATE: f64 = 0.25;

fn main() {
    let (x_train, y_train, x_test, y_test) = read_data().unwrap();
    println!("Loaded data");
    println!("Training: {} | Testing: {}", y_train.len(), y_test.len());

    let (mut weights_1, mut base_1, mut weights_2, mut base_2) = init_params();
    println!("Initiated weights/bases");

    let now = SystemTime::now();
    for i in 1..=ITERATIONS {
        println!("Running iteration #{}", i);
        let (z_1, a_1, z_2, a_2) = forward_prop(&weights_1, &base_1, &weights_2, &base_2, &x_train);
        let (dw_1, db_1, dw_2, db_2) =
            back_prop(&z_1, &a_1, &z_2, &a_2, &weights_2, &x_train, &y_train);

        weights_1 = weights_1.sub(&dw_1.scale(LEARNING_RATE)).unwrap();
        base_1 = base_1.sub(&db_1.scale(LEARNING_RATE)).unwrap();
        weights_2 = weights_2.sub(&dw_2.scale(LEARNING_RATE)).unwrap();
        base_2 = base_2.sub(&db_2.scale(LEARNING_RATE)).unwrap();
    }
    println!("Took {:?} to train", now.elapsed().unwrap());

    let stdin = stdin();
    let mut buffer = String::new();
    let mut rng = rand::thread_rng();
    loop {
        let i = rng.gen_range(0..100);
        let data = x_test.get_col(i).unwrap();
        let (_, _, _, a_2) = forward_prop(
            &weights_1,
            &base_1,
            &weights_2,
            &base_2,
            &Matrix::new(1, 784, data.as_vec().to_vec()).unwrap(),
        );
        let mut max_chance = *a_2
            .as_vec()
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let number = a_2.as_vec().iter().position(|&x| x == max_chance).unwrap();
        max_chance = max_chance * 100f64;
        println!("Guessed: {number} with {max_chance:.2}%");
        println!("Answer: {}", y_test.as_vec()[i]);
        let _ = stdin.read_line(&mut buffer).unwrap();
    }
}

fn read_data() -> Result<(Matrix<f64>, Vector<i32>, Matrix<f64>, Vector<i32>), &'static str> {
    let file = File::open("./mnist_train.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut y = Vec::new();
    let mut x = Vec::new();
    for result in rdr.records() {
        if result.is_err() {
            return Err("error reading data");
        }

        let record = result.unwrap();
        let mut row: Vec<f64> = record
            .iter()
            .skip(1)
            .map(|pixel| pixel.parse::<f64>().unwrap() / 255f64)
            .collect();
        x.append(&mut row);
        y.push(record.get(0).unwrap().parse::<i32>().unwrap());
    }

    return Ok((
        Matrix::new(784, 59900, x[..59900 * 784].to_vec())
            .unwrap()
            .transpose(),
        Vector::new(y[..59900].to_vec()),
        Matrix::new(784, 100, x[59900 * 784..].to_vec())
            .unwrap()
            .transpose(),
        Vector::new(y[59900..].to_vec()),
    ));
}

fn init_params() -> (Matrix<f64>, Vector<f64>, Matrix<f64>, Vector<f64>) {
    let mut rng = rand::thread_rng();

    let weights_1 = Matrix::new_of_supplier(784, 10, || rng.gen_range(-0.5..0.5));
    let base_1 = Vector::new_of_supplier(10, || rng.gen_range(-0.5..0.5));
    let weights_2 = Matrix::new_of_supplier(10, 10, || rng.gen_range(-0.5..0.5));
    let base_2 = Vector::new_of_supplier(10, || rng.gen_range(-0.5..0.5));

    (weights_1, base_1, weights_2, base_2)
}

fn forward_prop(
    w_1: &Matrix<f64>,
    b_1: &Vector<f64>,
    w_2: &Matrix<f64>,
    b_2: &Vector<f64>,
    x: &Matrix<f64>,
) -> (Matrix<f64>, Matrix<f64>, Matrix<f64>, Matrix<f64>) {
    let z_1 = w_1
        .simd_product_matrix(&x).unwrap()
        .add(&b_1.expand(x.width(), Axis::Column)).unwrap();

    let mut a_1 = z_1.clone();
    a_1.apply(|&x| if x > 0f64 { x } else { 0f64 }); // ReLU

    let z_2 = w_2
        .simd_product_matrix(&a_1).unwrap()
        .add(&b_2.expand(a_1.width(), Axis::Column)).unwrap();

    let mut a_2 = z_2.clone();
    let max = a_2.max();
    a_2.apply(|&x| 2.718281f64.powf(x - max)); // Softmax
    let a_2_sum = a_2.sum_columns();
    let a_2_sum_ref = a_2_sum.as_vec();

    a_2 = Matrix::new(
        a_2.width(),
        a_2.height(),
        a_2.as_vec()
            .chunks(a_2_sum.width())
            .map(|c| {
                c.iter()
                    .zip(a_2_sum_ref)
                    .map(|(a, b)| a / b)
                    .collect::<Vec<f64>>()
            })
            .flatten()
            .collect(),
    )
    .unwrap();

    (z_1, a_1, z_2, a_2)
}

fn one_hot(y: &Vector<i32>) -> Matrix<f64> {
    let mut zeroes = Matrix::new_of_element(10, y.len(), 0f64).unwrap();
    for (i, label) in y.as_vec().iter().enumerate() {
        zeroes[(i, *label as usize)] = 1f64;
    }
    zeroes.transpose()
}

fn deriv_re_lu(matrix: &Matrix<f64>) -> Matrix<f64> {
    let mut der = matrix.clone();
    der.apply(|&x| if x > 0f64 { 1f64 } else { 0f64 });
    der
}

fn back_prop(
    z_1: &Matrix<f64>,
    a_1: &Matrix<f64>,
    _z_2: &Matrix<f64>,
    a_2: &Matrix<f64>,
    w_2: &Matrix<f64>,
    x: &Matrix<f64>,
    y: &Vector<i32>,
) -> (Matrix<f64>, Vector<f64>, Matrix<f64>, Vector<f64>) {
    let m = y.len();
    let one_hot_y = one_hot(y);

    let dz_2 = a_2.sub(&one_hot_y).unwrap();
    let mut dw_2 = dz_2.simd_product_matrix(&a_1.transpose()).unwrap();
    dw_2.apply(|&x| x / (m as f64));
    let mut db_2 = dz_2.sum_rows();
    db_2.apply(|&x| x / (m as f64));

    let deriv_z_1 = deriv_re_lu(z_1);
    let mut dz_1 = w_2.transpose().simd_product_matrix(&dz_2).unwrap();
    dz_1 = Matrix::new(
        dz_1.width(),
        dz_1.height(),
        dz_1.as_vec()
            .iter()
            .zip(deriv_z_1.as_vec())
            .map(|(&i1, &i2)| i1 * i2)
            .collect(),
    )
    .unwrap();

    let mut dw_1 = dz_1.simd_product_matrix(&x.transpose()).unwrap();
    dw_1.apply(|&x| x / (m as f64));
    let mut db_1 = dz_1.sum_rows();
    db_1.apply(|&x| x / (m as f64));

    (dw_1, db_1, dw_2, db_2)
}
