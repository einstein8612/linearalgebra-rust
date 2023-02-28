use crate::numlib::{One, Zero};
use std::{
    ops::{Add, Mul, Sub},
    usize,
};

pub struct Vector<T> {
    data: Vec<T>,
    size: usize,
}

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

    pub const fn len(&self) -> usize {
        self.size
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

impl<T: Copy> Vector<T> {
    pub fn apply(mut self, f: &dyn Fn(T) -> T) -> Vector<T> {
        for val in &mut self.data {
            *val = f(*val);
        }
        self
    }
}

impl<T: Copy + Mul<T, Output = T>> Vector<T> {
    pub fn scale(&self, scalar: T) -> Vector<T> {
        let mut res = self.data.to_vec();

        for val in &mut res {
            *val = (*val) * scalar;
        }

        Vector::new(res)
    }
}

impl<T: Copy + Add<T, Output = T>> Vector<T> {
    pub fn add(&self, other: &Vector<T>) -> Result<Vector<T>, &'static str> {
        let to_add = other.as_vec();
        if other.size != self.size {
            return Err("Vectors don't match in size");
        }
        let mut res = self.data.to_vec();
        for index in 0..self.size {
            if let Some(elem) = res.get_mut(index) {
                // Unreachable since they have the same size and thus all indexes we loop through are
                // Also present in to_add
                *elem = *elem
                    + match to_add.get(index) {
                        Some(value) => *value,
                        None => unreachable!(),
                    };
            }
        }

        Ok(Vector::new(res))
    }
}

impl<T: Copy + Sub<T, Output = T>> Vector<T> {
    pub fn sub(&self, other: &Vector<T>) -> Result<Vector<T>, &'static str> {
        let to_add = other.as_vec();
        if other.size != self.size {
            return Err("Vectors don't match in size");
        }
        let mut res = self.data.to_vec();
        for index in 0..self.size {
            if let Some(elem) = res.get_mut(index) {
                // None is unreachable since they have the same size and thus all indexes we loop through are
                // Also present in to_add
                *elem = *elem
                    - match to_add.get(index) {
                        Some(value) => *value,
                        None => unreachable!(),
                    };
            }
        }

        Ok(Vector::new(res))
    }
}

impl<T: Copy + Zero + Add<T, Output = T> + Mul<T, Output = T>> Vector<T> {
    pub fn dot(&self, other: &Vector<T>) -> Result<T, &'static str> {
        if self.size != other.size {
            return Err("Vectors don't match in size");
        }

        let mut res = T::zero();

        if self.size == 0 {
            return Ok(res);
        }

        for index in 0..self.size {
            if let Some(elem) = self.data.get(index) {
                // None is unreachable since they have the same size and thus all indexes we loop through are
                // Also present in to_add
                res = res
                    + (*elem
                        * match other.data.get(index) {
                            Some(value) => *value,
                            None => unreachable!(),
                        });
            }
        }

        Ok(res)
    }
}

impl Vector<f64> {
    pub fn abs(&self) -> f64 {
        self.dot(&self).unwrap().sqrt()
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
