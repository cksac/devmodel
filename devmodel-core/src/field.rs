use serde::Serialize;

use std::fmt::Debug;

#[derive(Serialize, Debug)]
pub struct Field<FE>
where
    FE: Debug + Serialize + Default,
{
    pub name: String,
    pub optional: bool,
    pub extensions: FE,
}

impl<FE> Field<FE>
where
    FE: Debug + Serialize + Default,
{
    pub fn new(name: impl Into<String>) -> Field<FE> {
        Field {
            name: name.into(),
            optional: false,
            extensions: Default::default(),
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}
