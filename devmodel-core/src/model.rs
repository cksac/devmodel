use crate::{Edge, Field};

pub struct Model<ME, FE, EE> {
    pub name: String,
    pub fields: Vec<Field<FE>>,
    pub edges: Vec<Edge<EE>>,
    pub extensions: ME,
}

impl<ME, FE, EE> Model<ME, FE, EE>
where
    ME: Default,
    FE: Default,
    EE: Default,
{
    pub fn new(name: impl Into<String>) -> Model<ME, FE, EE> {
        Model {
            name: name.into(),
            fields: Vec::new(),
            edges: Vec::new(),
            extensions: Default::default(),
        }
    }

    pub fn field<F>(mut self, name: impl Into<String>, func: F) -> Model<ME, FE, EE>
    where
        F: Fn(Field<FE>) -> Field<FE>,
    {
        let f = Field::new(name);
        self.fields.push(func(f));
        self
    }

    pub fn edge<F>(mut self, name: impl Into<String>, func: F) -> Model<ME, FE, EE>
    where
        F: Fn(Edge<EE>) -> Edge<EE>,
    {
        let e = Edge::new(name);
        self.edges.push(func(e));
        self
    }
}
