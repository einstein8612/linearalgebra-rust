mod vec_impl;
mod vec_ops;

pub enum Axis {
    Column,
    Row,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vector<T> {
    data: Vec<T>,
    size: usize,
}
