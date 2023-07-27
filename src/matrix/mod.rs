mod mat_impl;
mod mat_display;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    size: usize,
    data: Vec<T>
}