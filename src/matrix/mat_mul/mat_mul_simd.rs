use crate::matrix::Matrix;
use crate::numlib::Zero;
use std::simd::SimdElement;
use std::ops::AddAssign;
use std::simd::Simd;
use std::ops::Mul;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 8usize;

impl<T: Send + Sync + Zero + SimdElement + AddAssign> Matrix<T> where Simd<T, 8>: AddAssign + Mul<Output = Simd<T,8>> {
    // impl<T> Matrix<T> {
        pub fn simd_product_matrix(
            &self,
            other: &Matrix<T>,
        ) -> Result<Matrix<T>, &'static str> {
            let data = &self.data;

            let mut transposed_b = Vec::with_capacity(other.size);
            for row in 0..other.width {
                for col in 0..other.height {
                    transposed_b.push(other[(col, row)]);
                }
            }
    
            let chunks = self.width() / CHUNK_SIZE;
            let left = chunks * CHUNK_SIZE;
    
            let res = (0..self.height() * other.width())
                .into_par_iter()
                .map(|index| {
                    let row = index / other.width();
                    let column = index % self.height();
    
                    let mut total_simd = Simd::<T, CHUNK_SIZE>::splat(T::zero());
                    for k in 0..chunks {
                        let simd_a =
                            Simd::from_slice(&data[row * self.width() + k * CHUNK_SIZE..]);
                        let simd_b =
                            Simd::from_slice(&transposed_b[column * other.height() + k * CHUNK_SIZE..]);
    
                        let multiplied = simd_a * simd_b;
                        total_simd += multiplied;
                    }
    
                    let mut a_simd = Simd::splat(T::zero());
                    let mut b_simd = Simd::splat(T::zero());
    
                    for k in left..self.width() {
                        a_simd[k - left] = data[row * self.width() + k];
                        b_simd[k - left] = transposed_b[column * other.height() + k];
                    }
    
                    total_simd += a_simd * b_simd;
    
                    let mut total = T::zero();
                    for end in total_simd.to_array().iter() {
                        total += *end;
                    }
                    total
                }).collect();
    
            Ok(Matrix::new(other.width(), self.height(), res).unwrap())
        }
    }