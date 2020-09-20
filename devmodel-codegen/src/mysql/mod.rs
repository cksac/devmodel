use devmodel_core::*;

pub mod column;
pub mod types;
pub use column::Column;
pub use types::Type;

pub struct Schema;
impl<DE, ME, FE, EE> Generator<DE, ME, FE, EE> for Schema
where
    FE: Extension<Column>,
{
    type Output = ();
    type Error = ();
    fn generate(&mut self, domain: &Domain<DE, ME, FE, EE>) -> Result<Self::Output, Self::Error> {
        for (_, model) in domain.models.iter() {
            println!("Table {}", model.name);
            for field in model.fields.iter() {
                if let Some(e) = field.extensions.get() {
                    println!(
                        "\t- {}, optional: {}, sql type: {:?}",
                        field.name, field.optional, e.ty
                    );
                }
            }
        }
        Ok(())
    }
}
