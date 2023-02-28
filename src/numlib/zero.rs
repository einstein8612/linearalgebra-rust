pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32  {
    fn zero() -> i32 {
        return 0i32 
    }
}

impl Zero for f64  {
    fn zero() -> f64 {
        return 0f64
    }
}