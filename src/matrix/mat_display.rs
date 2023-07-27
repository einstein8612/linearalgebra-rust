use std::fmt::Display;

use super::Matrix;

const MATRIX_TOP_LEFT_CORNER: char = '┌';
const MATRIX_TOP_RIGHT_CORNER: char = '┐';
const MATRIX_BOTTOM_LEFT_CORNER: char = '└';
const MATRIX_BOTTOM_RIGHT_CORNER: char = '┘';
const MATRIX_SIDE: char = '│';

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_size = if self.size != 0 {
            self.data
                .iter()
                .map(|entry| format!("{}", entry).len())
                .max()
                .unwrap()
        } else {
            0
        };

        let to_display = self
            .data
            .iter()
            .map(|entry| format!("{: <width$}", entry, width = max_size))
            .collect::<Vec<String>>();

        let mut res = String::with_capacity(
            (self.width * max_size * 2 + 3) * (self.height * max_size * 2 + 3),
        );
        res.push(MATRIX_TOP_LEFT_CORNER);
        res.push_str(&" ".repeat(self.width * (max_size + 1) + 1));
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

                res.push_str(&format!("{} ", to_display.get(index).unwrap()));
                printed += 1;
            }
            res.push(MATRIX_SIDE);
        }

        res.push('\n');
        res.push(MATRIX_BOTTOM_LEFT_CORNER);
        res.push_str(&" ".repeat(self.width * (max_size + 1) + 1));
        res.push(MATRIX_BOTTOM_RIGHT_CORNER);

        write!(f, "{}", res)
    }
}
