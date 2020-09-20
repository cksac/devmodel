use devmodel_core::*;

pub mod column;
pub mod table;
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
            let mut cols = Vec::with_capacity(model.fields.len());
            for field in model.fields.iter() {
                if let Some(e) = field.extensions.get() {
                    cols.push(format!(
                        "\t{} {:?} {}",
                        field.name,
                        e.ty,
                        if field.optional { "NULL" } else { "NOT NULL" }
                    ));
                }
            }
            println!("CREATE TABLE {} {{", model.name);
            println!("{}", cols.join(",\n"));
            println!("}};")
        }
        Ok(())
    }
}
