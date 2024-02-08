use crate::matrix::Matrix;

use rayon::prelude::*;

use std::ops::{Add, Mul};
use crate::numlib::Zero;

/**
 * Logic for generic matrix multiplication.
 * Naive and pretty slow, but it works.
 */
impl<T: Send + Sync + Copy + Zero + Add<T, Output = T> + Mul<T, Output = T> + std::ops::Sub<Output = T>>
    Matrix<T>
{
    pub fn product_matrix(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.width != other.height {
            return Err("Matrices have mismatched sizes");
        }

        // Transpose for better alignment in memory
        let mut other_t = Vec::with_capacity(other.size);
        for row in 0..other.width {
            for col in 0..other.height {
                other_t.push(other[(col, row)]);
            }
        }

        // Rayon parallel iterator for faster computation
        let res: Vec<T> = (0..self.height() * other.width())
            .into_par_iter()
            .map(|index| {
                let row = index / other.width();
                let col = index % other.width();

                let mut entry = T::zero();
                for index in 0..self.width {
                    entry = entry
                        + self.data[row * self.width + index]
                            * other_t[col * other.width + index];
                }
                entry
        }).collect();

        Matrix::new(other.width, self.height, res)
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
    #[deprecated(
        since = "0.2.0",
        note = "trivial_product_matrix is a slower method use product_matrix to use the fastest algorithm"
    )]
    pub fn trivial_product_matrix(&self, other: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.width != other.height {
            return Err("Matrices have mismatched sizes");
        }

        // Transpose for better alignment in memory
        let mut other_t = Vec::with_capacity(other.size);
        for row in 0..other.width {
            for col in 0..other.height {
                other_t.push(other[(col, row)]);
            }
        }
        
        let mut res = vec![T::zero(); self.height * other.width];
        for row in 0..self.height {
            for col in 0..other.width {
                let mut entry = res[row * other.width + col];
                for index in 0..self.width {
                    entry = entry
                        + self.data[row * self.width + index]
                            * other_t[col * other.width + index];
                }
                res[row * other.width + col] = entry;
            }
        }

        Matrix::new(other.width, self.height, res)
    }
}
