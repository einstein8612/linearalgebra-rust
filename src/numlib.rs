pub trait Zero {
    const fn zero() -> Self;
}

pub impl Zero for i32  {
    fn zero() -> i32 {
        return 0i32 
    }
}