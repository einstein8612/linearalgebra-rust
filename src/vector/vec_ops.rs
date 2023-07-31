use std::ops::{Mul, Add, Sub};

use crate::numlib::Zero;

use super::Vector;

impl<T: Copy> Vector<T> {
    pub fn apply<F: Fn(&T) -> T>(&mut self, f: F) {
        for val in &mut self.data {
            *val = f(val);
        }
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