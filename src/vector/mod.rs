mod vec_impl;
mod vec_ops;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vector<T> {
    data: Vec<T>,
    size: usize,
}
