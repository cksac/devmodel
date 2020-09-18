use crate::Domain;
use serde::Serialize;

use std::fmt::Debug;
pub trait Generator<DE, ME, FE, EE>
where
    DE: Debug + Serialize + Default,
    ME: Debug + Serialize + Default,
    FE: Debug + Serialize + Default,
    EE: Debug + Serialize + Default,
{
    type Output;
    type Error;
    fn generate(&mut self, domain: &Domain<DE, ME, FE, EE>) -> Result<Self::Output, Self::Error>;
}
