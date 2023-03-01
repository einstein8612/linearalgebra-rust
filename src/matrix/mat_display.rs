use std::fmt::Display;

use super::Matrix;

const MATRIX_TOP_LEFT_CORNER: char = '┌';
const MATRIX_TOP_RIGHT_CORNER: char = '┐';
const MATRIX_BOTTOM_LEFT_CORNER: char = '└';
const MATRIX_BOTTOM_RIGHT_CORNER: char = '┘';
const MATRIX_SIDE: char = '│';

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity((self.width * 2 + 3) * (self.height * 2 + 3));
        res.push(MATRIX_TOP_LEFT_CORNER);
        res.push_str(&" ".repeat(self.width * 2 + 1));
        res.push(MATRIX_TOP_RIGHT_CORNER);

        // If 0 sized matrix don't worry about the inside stuff
        if self.size > 0 {
            res.push('\n');
            res.push(MATRIX_SIDE);
            res.push(' ');
            let mut printed = 0usize;
            for index in 0..self.size {
                if printed == self.width {
                    res.push(MATRIX_SIDE);
                    res.push('\n');
                    res.push(MATRIX_SIDE);
                    res.push(' ');

                    printed = 0;
                }

                res.push_str(&format!("{} ", self.data.get(index).unwrap()));
                printed += 1;
            }
            res.push(MATRIX_SIDE);
        }

        res.push('\n');
        res.push(MATRIX_BOTTOM_LEFT_CORNER);
        res.push_str(&" ".repeat(self.width * 2 + 1));
        res.push(MATRIX_BOTTOM_RIGHT_CORNER);

        write!(f, "{}", res)
    }
}
