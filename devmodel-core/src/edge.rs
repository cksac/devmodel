pub struct Edge<EE> {
    pub name: String,
    pub extensions: EE,
}

impl<EE> Edge<EE>
where
    EE: Default,
{
    pub fn new(name: impl Into<String>) -> Edge<EE> {
        Edge {
            name: name.into(),
            extensions: Default::default(),
        }
    }
}
