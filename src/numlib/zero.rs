pub trait Zero {
    fn zero() -> Self;
}

impl Zero for u8 {
    fn zero() -> u8 {
        0
    }
}

impl Zero for i32 {
    fn zero() -> i32 {
        return 0i32;
    }
}

impl Zero for f64 {
    fn zero() -> f64 {
        return 0f64;
    }
}
