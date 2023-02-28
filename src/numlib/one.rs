pub trait One {
    fn one() -> Self;
}

impl One for i32  {
    fn one() -> i32 {
        return 1i32 
    }
}

impl One for f64  {
    fn one() -> f64 {
        return 1f64 
    }
}