pub trait Extension<T> {
    fn get(&self) -> Option<&T>;
    fn set(&mut self, val: T);
}

#[macro_export]
macro_rules! define_ext {
    ($name:ident {
        $($field:ident: $ext_ty:ty),+ $(,)?
    }) => {
        #[derive(Debug, Default)]
        pub struct $name {
            $(
            pub $field: Option<$ext_ty>,
            )*
        }

        $(
        impl $crate::Extension<$ext_ty> for $name {
            fn get(&self) -> Option<&$ext_ty> {
                self.$field.as_ref()
            }
            fn set(&mut self, ext: $ext_ty) {
                self.$field = Some(ext);
            }
        }
        )*
    }
}
