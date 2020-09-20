use crate::Domain;

pub trait Generator<DE, ME, FE, EE> {
    type Output;
    type Error;
    fn generate(&mut self, domain: &Domain<DE, ME, FE, EE>) -> Result<Self::Output, Self::Error>;
}
