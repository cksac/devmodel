use crate::Model;

use std::collections::HashMap;

pub struct Domain<DE, ME, FE, EE> {
    pub name: String,
    pub models: HashMap<String, Model<ME, FE, EE>>,
    pub extensions: DE,
}

impl<DE, ME, FE, EE> Domain<DE, ME, FE, EE>
where
    DE: Default,
    ME: Default,
    FE: Default,
    EE: Default,
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
