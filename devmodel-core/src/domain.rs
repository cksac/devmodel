use crate::Model;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
#[derive(Serialize, Debug)]
pub struct Domain<DE, ME, FE, EE>
where
    DE: Debug + Serialize + Default,
    ME: Debug + Serialize + Default,
    FE: Debug + Serialize + Default,
    EE: Debug + Serialize + Default,
{
    pub name: String,
    pub models: HashMap<String, Model<ME, FE, EE>>,
    pub extensions: DE,
}

impl<DE, ME, FE, EE> Domain<DE, ME, FE, EE>
where
    DE: Debug + Serialize + Default,
    ME: Debug + Serialize + Default,
    FE: Debug + Serialize + Default,
    EE: Debug + Serialize + Default,
{
    pub fn new(name: impl Into<String>) -> Domain<DE, ME, FE, EE> {
        Domain {
            name: name.into(),
            models: HashMap::new(),
            extensions: Default::default(),
        }
    }

    pub fn model(mut self, model: Model<ME, FE, EE>) -> Domain<DE, ME, FE, EE> {
        self.models.insert(model.name.clone(), model);
        self
    }
}
