pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_Zero {
    (for $($t:ty),+) => {
        $(impl Zero for $t {
            fn zero() -> $t {
                0 as $t
            }
        })*
    }
}

impl_Zero!(for u8, u16, u32, u64, u128, usize);
impl_Zero!(for i8, i16, i32, i64, i128, isize);
impl_Zero!(for f32, f64);