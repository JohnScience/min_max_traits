#![no_std]
#![doc = include_str!("../README.md")]

/// Trait ensuring that there is a minimal value that is also constant
pub trait Min {
    const MIN: Self;
}

/// Trait ensuring that there is a maximal value that is also constant
pub trait Max {
    const MAX: Self;
}

macro_rules! impl_min {
    ($t:ty) => {
        impl Min for $t {
            const MIN: Self = <$t>::MIN;
        }
    };
}

macro_rules! impl_max {
    ($t:ty) => {
        impl Max for $t {
            const MAX: Self = <$t>::MAX;
        }
    };
}

// https://doc.rust-lang.org/stable/reference/types/numeric.html
macro_rules! call_for_prim_numeric_types {
    ($macro:ident) => {
        $macro!(i8);
        $macro!(u8);
        $macro!(i16);
        $macro!(u16);
        $macro!(i32);
        $macro!(u32);
        $macro!(i64);
        $macro!(u64);
        $macro!(i128);
        $macro!(u128);
        $macro!(isize);
        $macro!(usize);
        $macro!(f32);
        $macro!(f64);
    };
}

call_for_prim_numeric_types!(impl_min);
call_for_prim_numeric_types!(impl_max);