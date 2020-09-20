use std::fmt::Debug;

pub trait Type {}

// represent unit type ()
#[derive(Debug)]
pub struct Unit;

impl Type for Unit {}

macro_rules! define_num {
    ($name:ident, $ty:ty) => {
        #[derive(Debug)]
        pub struct $name {
            pub default: Option<$ty>,
        }
        impl $name {
            pub fn new() -> $name {
                $name { default: None }
            }

            pub fn default(val: $ty) -> $name {
                $name { default: Some(val) }
            }
        }

        impl Type for $name {}
    };
}

define_num!(Isize, isize);
define_num!(I8, i8);
define_num!(I16, i16);
define_num!(I32, i32);
define_num!(I64, i64);
define_num!(I128, i128);

define_num!(Usize, usize);
define_num!(U8, u8);
define_num!(U16, u16);
define_num!(U32, u32);
define_num!(U64, u64);
define_num!(U128, u128);

pub struct Str {
    pub default: Option<String>,
}

impl Type for Str {}

pub struct Bool {
    pub default: Option<bool>,
}

impl Type for Bool {}
