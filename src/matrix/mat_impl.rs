extern crate rayon;

use std::{
    ops::{Add, Index, IndexMut, Mul},
    simd::{Simd, SimdFloat},
};

use rayon::prelude::*;

use super::Matrix;
use crate::{numlib::Zero, vector::Vector};

const CHUNK_SIZE: usize = 8usize;

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Result<Matrix<T>, &'static str> {
        if width * height != data.len() {
            return Err("Dimensions don't match the given data");
        }

        Ok(Matrix {
            width,
            height,
            size: width * height,
            data,
        })
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn shape(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}

impl<T: Copy> Matrix<T> {
    pub fn new_of_element(
        width: usize,
        height: usize,
        element: T,
    ) -> Result<Matrix<T>, &'static str> {
        Ok(Matrix {
            width,
            height,
            size: width * height,
            data: vec![element; width * height],
        })
    }

    pub fn new_of_supplier<F: FnMut() -> T>(
        width: usize,
        height: usize,
        mut supplier: F,
    ) -> Result<Matrix<T>, &'static str> {
        Ok(Matrix {
            width,
            height,
            size: width * height,
            data: (0..width * height).map(|_| supplier()).collect(),
        })
    }

    pub fn apply<F: Fn(&T) -> T>(&mut self, f: F) {
        for i in self.data.iter_mut() {
            *i = f(i);
        }
    }

    pub fn get_col(&self, number: usize) -> Result<Vector<T>, &'static str> {
        if number >= self.width {
            return Err("This column isn't present in this matrix");
        }

        let mut vector: Vec<T> = Vec::with_capacity(self.height);
        let mut index = number;
        while index < self.size {
            vector.push(self.data[index]);
            index += self.width; // Add width to get next value
        }

        Ok(Vector::new(vector))
    }

    pub fn get_cols(&self) -> Vec<Vector<T>> {
        let mut res: Vec<Vector<T>> = Vec::with_capacity(self.width);
        for w in 0..self.width {
            res.push(self.get_col(w).unwrap());
        }

        return res;
    }

    pub fn get_row(&self, number: usize) -> Result<Vector<T>, &'static str> {
        if number >= self.height {
            return Err("This row isn't present in this matrix");
        }

        let mut vector: Vec<T> = Vec::with_capacity(self.width);
        let index = number * self.width;
        for offset in 0..self.width {
            vector.push(self.data[index + offset]);
        }

        Ok(Vector::new(vector))
    }

    pub fn get_rows(&self) -> Vec<Vector<T>> {
        let mut res: Vec<Vector<T>> = Vec::with_capacity(self.width);
        for w in 0..self.height {
            res.push(self.get_row(w).unwrap());
        }

        return res;
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut list = Vec::with_capacity(self.size);
        for col in self.get_cols() {
            list.extend(col.as_vec())
        }

        let width = self.height;
        let height = self.width;

        return Matrix::new(width, height, list).unwrap();
    }
}

impl<T: Copy + Zero + Add<T, Output = T> + Mul<T, Output = T> + std::ops::Sub<Output = T>>
    Matrix<T>
{
    pub fn product_vector(&self, vector: &Vector<T>) -> Result<Vector<T>, &'static str> {
        if self.width != vector.len() {
            return Err("Vector and matrix have mismatched sizes");
        }
        let mut res = Vec::with_capacity(self.height);
        for row in self.get_rows() {
            res.push(row.dot(vector).unwrap());
        }

        Ok(Vector::new(res))
    }

    pub fn product_matrix(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        self.trivial_product_matrix(other)
    }

    /**
     * A simple multiplication algorithm using the provided methods
     * for short and readable code
     */
    #[deprecated(
        since = "0.1.7",
        note = "simple_product_matrix is a slower method use product_matrix to use the fastest algorithm"
    )]
    pub fn simple_product_matrix(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.width != other.height {
            return Err("Matrices have mismatched sizes");
        }
        let mut res = Vec::with_capacity(self.height * other.width);
        for col in other.get_cols() {
            res.extend(self.product_vector(&col).unwrap().as_vec());
        }

        Ok(Matrix::new(self.height, other.width, res)
            .unwrap()
            .transpose())
    }

    /**
     * All logic written locally here instead of spread out. Still using the logic of
     * AB = Ab1 + Ab2 + ... + Abn
     * O(T(n)) = n^3
     */
    pub fn trivial_product_matrix(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.width != other.height {
            return Err("Matrices have mismatched sizes");
        }
        let mut res = Vec::with_capacity(self.height * other.width);
        for row in 0..self.height {
            for col in 0..other.width {
                let mut entry = T::zero();
                for index in 0..self.width {
                    entry = entry
                        + self.data[self.width * row + index]
                            * other.data[other.width * index + col];
                }
                res.push(entry);
            }
        }

        Matrix::new(other.width, self.height, res)
    }

    pub fn add(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.shape() != other.shape() {
            return Err("Matrices have mismatched sizes");
        }

        let mut res: Vec<T> = Vec::with_capacity(self.size);
        for (i, element) in self.data.iter().enumerate() {
            res.push(*element + *other.data.get(i).unwrap());
        }

        Matrix::new(self.width, self.height, res)
    }

    pub fn sub(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.shape() != other.shape() {
            return Err("Matrices have mismatched sizes");
        }

        let mut res: Vec<T> = Vec::with_capacity(self.size);
        for (i, element) in self.data.iter().enumerate() {
            res.push(*element - *other.data.get(i).unwrap());
        }

        Matrix::new(self.width, self.height, res)
    }

    pub fn sum(&self) -> T {
        let mut accumulator: T = T::zero();
        for el in self.data.iter() {
            accumulator = accumulator + *el;
        }
        accumulator
    }

    pub fn sum_columns(&self) -> Matrix<T> {
        let mut sum = Vec::with_capacity(self.width);
        for column in 0..self.width {
            let mut accumulator: T = T::zero();
            for k in 0..self.height {
                accumulator = accumulator + self.data[k * self.width + column];
            }
            sum.push(accumulator);
        }

        return Matrix::new(self.width, 1, sum).unwrap();
    }

    pub fn sum_rows(&self) -> Vector<T> {
        let mut sum = Vec::with_capacity(self.height);
        for row in 0..self.height {
            let mut accumulator: T = T::zero();
            for k in 0..self.width {
                accumulator = accumulator + self.data[row * self.width + k];
            }
            sum.push(accumulator);
        }

        return Vector::new(sum);
    }

    pub fn scale(&self, scalar: T) -> Matrix<T> {
        let mut scaled = self.clone();
        scaled.apply(|x| *x = *x*scalar);

        scaled
    }
}

// impl<T: Zero + SimdElement + SimdFloat> Matrix<T> where Simd<T, 8>: AddAssign +  Mul<Output = Simd<T,8>>{
impl Matrix<f64> {
    pub fn simd_product_matrix(
        &self,
        other: &Matrix<f64>,
    ) -> Result<Matrix<f64>, &'static str> {
        let data = &self.data;
        let mut out: Vec<f64> = vec![];

        let mut transposed_b = vec![0f64; other.width() * other.height()];
        for i in 0..other.height() {
            for j in 0..other.width() {
                transposed_b[j * other.height() + i] = other[(i, j)];
            }
        }

        let chunks = self.width() / CHUNK_SIZE;
        let left = chunks * CHUNK_SIZE;

        (0..self.height() * other.width())
            .into_par_iter()
            .map(|index| {
                let row = index / other.width();
                let column = index % other.width();

                let mut total_simd = Simd::<f64, 8>::splat(0f64);
                for k in 0..chunks {
                    let simd_a = Simd::from_slice(&data[row * self.width() + k * CHUNK_SIZE..]);
                    let simd_b =
                        Simd::from_slice(&transposed_b[column * other.height() + k * CHUNK_SIZE..]);

                    let multiplied = simd_a * simd_b;
                    total_simd += multiplied;
                }

                let mut a_simd = Simd::splat(0f64);
                let mut b_simd = Simd::splat(0f64);

                for k in left..self.width() {
                    a_simd[k - left] = data[row * self.width() + k];
                    b_simd[k - left] = transposed_b[column * other.height() + k];
                }

                total_simd += a_simd * b_simd;
                total_simd.reduce_sum()
            })
            .collect_into_vec(&mut out);

        Ok(Matrix::new(other.width(), self.height(), out).unwrap())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.width + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.data[row * self.width + col]
    }
}
