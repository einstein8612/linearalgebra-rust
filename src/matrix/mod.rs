use std::ops::{Mul, Add, Sub};

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

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Vector<T> {
        Vector {
            size: self.size,
            data: self.data.clone(),
        }
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
            *val = (*val)*scalar;
        }

        Vector::new(res)
    }
}

impl<T: Copy + Add<T, Output = T>> Vector<T> {
    pub fn add(&self, other: &Vector<T>) -> Result<Vector<T>, &'static str> {
        let to_add = other.as_vec();
        if other.size != self.size {
            return Err("Vectors didn't match in size");
        }
        let mut res = self.data.to_vec();
        for index in 0..self.size {
            if let Some(elem) = res.get_mut(index) {
                // Unreachable since they have the same size and thus all indexes we loop through are
                // Also present in to_add
                *elem = *elem + match to_add.get(index) {Some(value) => *value, None => unreachable!()};
            }
        }

        Ok(Vector::new(res))
    }
}

impl<T: Copy + Sub<T, Output = T>> Vector<T> {
    pub fn sub(&self, other: &Vector<T>) -> Result<Vector<T>, &'static str> {
        let to_add = other.as_vec();
        if other.size != self.size {
            return Err("Vectors didn't match in size");
        }
        let mut res = self.data.to_vec();
        for index in 0..self.size {
            if let Some(elem) = res.get_mut(index) {
                // Unreachable since they have the same size and thus all indexes we loop through are
                // Also present in to_add
                *elem = *elem - match to_add.get(index) {Some(value) => *value, None => unreachable!()};
            }
        }

        Ok(Vector::new(res))
    }
}