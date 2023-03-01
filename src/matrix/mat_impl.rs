use crate::vector::Vector;

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
}
