extern crate rayon;

use std::ops::{Add, Index, IndexMut, Mul};

use super::Matrix;
use crate::{numlib::Zero, vector::Vector};

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
    ) -> Matrix<T> {
        Matrix {
            width,
            height,
            size: width * height,
            data: (0..width * height).map(|_| supplier()).collect(),
        }
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

impl<T: PartialOrd + Copy> Matrix<T> {
    pub fn max(&self) -> T {
        *self.data.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn min(&self) -> T {
        *self.data.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()
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
        scaled.apply(|&x| x*scalar);

        scaled
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
