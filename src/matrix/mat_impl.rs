use std::ops::{Add, Mul};

use crate::{vector::Vector, numlib::Zero};

 use super::Matrix;

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
}

impl<T: Copy> Matrix<T> {
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
        let index = number*self.width;
        for offset in 0..self.width {
            vector.push(self.data[index+offset]);
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
}

impl <T: Copy> Matrix<T> {
    pub fn transpose(&self) -> Matrix<T> {
        let mut list = Vec::with_capacity(self.size);
        for col in self.get_cols() {
            list.extend(col.as_vec())
        }

        let width = self.height;
        let height = self.width;

        return Matrix::new(width, height, list).unwrap()
    }
}

impl<T: Copy + Zero + Add<T, Output = T> + Mul<T, Output = T>> Matrix<T> {
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
        if self.width != other.height {
            return Err("Matrices have mismatched sizes");
        }
        let mut res = Vec::with_capacity(self.height*other.width);
        for col in other.get_cols() {
            res.extend(self.product_vector(&col).unwrap().as_vec());
        }

        Ok(Matrix::new(self.height, other.width, res).unwrap().transpose())
    }
}