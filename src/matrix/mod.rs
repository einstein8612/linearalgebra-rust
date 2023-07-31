mod mat_impl;
mod mat_display;

#[cfg(feature = "fast")]
pub mod fast;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    size: usize,
    data: Vec<T>
}