mod vec_impl;
mod vec_ops;

pub struct Vector<T> {
    data: Vec<T>,
    size: usize,
}
