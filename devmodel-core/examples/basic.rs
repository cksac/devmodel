use devmodel_core::types::*;
use devmodel_core::*;
use serde::Serialize;

use std::fmt::Debug;

pub struct Describe;
impl<DE, ME, FE, EE> Generator<DE, ME, FE, EE> for Describe
where
    DE: Debug + Serialize + Default,
    ME: Debug + Serialize + Default,
    FE: Debug + Serialize + Default,
    EE: Debug + Serialize + Default,
{
    type Output = ();
    type Error = ();
    fn generate(&mut self, domain: &Domain<DE, ME, FE, EE>) -> Result<Self::Output, Self::Error> {
        for (_, model) in domain.models.iter() {
            println!("Model {}", model.name);
            for field in model.fields.iter() {
                println!("\t- {}, optional: {}", field.name, field.optional);
            }
        }
        Ok(())
    }
}

// Trait to extend the Model, can use to modify extensions as well
// Use trait to extend Model modification function of fields/edges/extensions?
pub trait TimeFields<C: Default> {
    fn time_fields_with_config(self, config: C) -> Self;
    fn time_fields(self) -> Self
    where
        Self: Sized,
    {
        self.time_fields_with_config(Default::default())
    }
}

#[derive(Serialize, Debug)]
pub struct TimeFieldsConfig {
    created_at: String,
    created_by: String,
}
impl Default for TimeFieldsConfig {
    fn default() -> Self {
        TimeFieldsConfig {
            created_at: "create_at".to_string(),
            created_by: "create_by".to_string(),
        }
    }
}

impl<ME, FE, EE> TimeFields<TimeFieldsConfig> for Model<ME, FE, EE>
where
    ME: Debug + Serialize + Default,
    FE: Debug + Serialize + Default,
    EE: Debug + Serialize + Default,
{
    fn time_fields_with_config(self, config: TimeFieldsConfig) -> Self {
        self.field(config.created_at, |f| f)
            .field(config.created_by, |f| f)
    }
}

fn main() {
    let domain = Domain::<(), (), (), ()>::new("Test")
        .model(
            Model::new("Alert")
                .field("alert_id", |f| f.ty(Isize::default(10)))
                .field("latest_detection_id", |f| f.ty(Isize::new()))
                .time_fields(),
        )
        .model(
            Model::new("Detection")
                .field("detection_id", |f| f)
                .field("alert_id", |f| f)
                .field("extra_info_id", |f| f.optional())
                .time_fields_with_config(TimeFieldsConfig {
                    created_at: "create_gmt".to_string(),
                    created_by: "create_user".to_string(),
                }),
        );

    let _ = Describe.generate(&domain);
}
