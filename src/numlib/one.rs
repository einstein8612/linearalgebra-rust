pub trait One {
    fn one() -> Self;
}

macro_rules! impl_One {
    (for $($t:ty),+) => {
        $(impl One for $t {
            fn one() -> $t {
                1 as $t
            }
        })*
    }
}

impl_One!(for u8, u16, u32, u64, u128, usize);
impl_One!(for i8, i16, i32, i64, i128, isize);
impl_One!(for f32, f64);