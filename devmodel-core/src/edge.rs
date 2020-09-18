use serde::Serialize;

use std::fmt::Debug;
#[derive(Serialize, Debug)]
pub struct Edge<EE>
where
    EE: Debug + Serialize + Default,
{
    pub name: String,
    pub extensions: EE,
}

impl<EE> Edge<EE>
where
    EE: Debug + Serialize + Default,
{
    pub fn new(name: impl Into<String>) -> Edge<EE> {
        Edge {
            name: name.into(),
            extensions: Default::default(),
        }
    }
}
