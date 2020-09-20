use std::fmt::Debug;

pub trait Type {}

// represent unit type ()
#[derive(Debug)]
pub struct Unit;
impl Type for Unit {}

macro_rules! define_ty {
    ($name:ident, $ty:ty) => {
        #[derive(Debug)]
        pub struct $name {
            pub default_val: Option<$ty>,
        }
        impl $name {
            pub fn new() -> $name {
                $name { default_val: None }
            }

            pub fn default(val: $ty) -> $name {
                $name {
                    default_val: Some(val),
                }
            }
        }

        impl Type for $name {}
    };
}

define_ty!(Isize, isize);
define_ty!(I8, i8);
define_ty!(I16, i16);
define_ty!(I32, i32);
define_ty!(I64, i64);
define_ty!(I128, i128);

define_ty!(Usize, usize);
define_ty!(U8, u8);
define_ty!(U16, u16);
define_ty!(U32, u32);
define_ty!(U64, u64);
define_ty!(U128, u128);

define_ty!(Str, String);
define_ty!(Bool, bool);
