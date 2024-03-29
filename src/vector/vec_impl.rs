use crate::{
    matrix::Matrix,
    numlib::{One, Zero},
};

use super::{Vector, Axis};

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Vector {
            size: data.len(),
            data,
        }
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub const fn len(&self) -> usize {
        self.size
    }
}

impl<T: Copy> Vector<T> {
    pub fn new_of_supplier<F: FnMut() -> T>(
        size: usize,
        mut supplier: F,
    ) -> Vector<T> {
        Vector {
            size,
            data: (0..size).map(|_| supplier()).collect(),
        }
    }

    pub fn expand(&self, n: usize, axis: Axis) -> Matrix<T> {
        let expanded_matrix = Matrix::new(self.size, n, self.data.repeat(n)).unwrap();

        match axis {
            Axis::Column => expanded_matrix.transpose(),
            Axis::Row => expanded_matrix,
        }
    }
}

impl<T: Zero + Clone> Vector<T> {
    pub fn zeroes(size: usize) -> Self {
        Vector {
            size,
            data: vec![T::zero(); size],
        }
    }
}

impl<T: One + Clone> Vector<T> {
    pub fn ones(size: usize) -> Self {
        Vector {
            size,
            data: vec![T::one(); size],
        }
    }
}

impl<T: Zero + One + Clone> Vector<T> {
    pub fn cartesian_unit_vector(
        number: usize,
        dimensions: usize,
    ) -> Result<Vector<T>, &'static str> {
        if number > dimensions {
            return Err("The number cannot be larger than the dimensions");
        }
        let mut data = vec![T::zero(); dimensions];
        data[number - 1] = T::one();

        Ok(Vector {
            size: dimensions,
            data,
        })
    }
}

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Vector<T> {
        Vector {
            size: self.size,
            data: self.data.clone(),
        }
    }
}
