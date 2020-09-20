use devmodel_codegen::mysql;
use devmodel_core::types::*;
use devmodel_core::*;
use serde::Serialize;

use std::fmt::Debug;

// #[derive(Debug, Default)]
// pub struct FieldExt {
//     pub mysql_col: Option<mysql::Column>,
// }

// define_ext! will generate following extension struct
// #[derive(Debug, Default)]
// pub struct FieldExt {
//     pub mysql_col: Option<mysql::Column>,
// }
// impl Extension<mysql::Column> for FieldExt {
//     fn get(&self) -> Option<&mysql::Column> {
//         self.mysql_col.as_ref()
//     }
//     fn set(&mut self, ext: mysql::Column) {
//         self.mysql_col = Some(ext);
//     }
// }

define_ext! {
    FieldExt {
        mysql_col: mysql::Column
    }
}

pub struct Describe;
impl<DE, ME, FE, EE> Generator<DE, ME, FE, EE> for Describe {
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
    ME: Default,
    FE: Default,
    EE: Default,
{
    fn time_fields_with_config(self, config: TimeFieldsConfig) -> Self {
        self.field(config.created_at, |f| f)
            .field(config.created_by, |f| f)
    }
}

fn main() {
    let domain = Domain::<(), (), FieldExt, ()>::new("Test")
        .model(
            Model::new("Alert")
                .field("alert_id", |f| f.ty(Isize::default(10)))
                .field("latest_detection_id", |f| {
                    f.ty(Isize::new())
                        .extension(mysql::column::int(20))
                        .extension(mysql::column::int(10))
                })
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

    let _ = mysql::Schema.generate(&domain);
}
