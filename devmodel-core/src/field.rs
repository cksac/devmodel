use crate::extension::Extension;
use crate::types::{self, Type};

pub struct Field<FE> {
    pub name: String,
    pub ty: Box<dyn Type>,
    pub optional: bool,
    pub extensions: FE,
}

impl<FE> Field<FE>
where
    FE: Default,
{
    pub fn new(name: impl Into<String>) -> Field<FE> {
        Field {
            name: name.into(),
            optional: false,
            ty: Box::new(types::Unit),
            extensions: Default::default(),
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn ty(mut self, ty: impl Type + 'static) -> Self {
        self.ty = Box::new(ty);
        self
    }

    pub fn extension<E>(mut self, ext: E) -> Self
    where
        FE: Extension<E>,
    {
        self.extensions.set(ext);
        self
    }
}
