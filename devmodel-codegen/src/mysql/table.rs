// use devmodel_core::*;

// // Trait to extend the Model, can use to modify extensions as well
// // Use trait to extend Model modification function of fields/edges/extensions?
// pub trait TimeFields<C: Default> {
//     fn time_fields_with_config(self, config: C) -> Self;
//     fn time_fields(self) -> Self
//     where
//         Self: Sized,
//     {
//         self.time_fields_with_config(Default::default())
//     }
// }

// #[derive(Debug)]
// pub struct TimeFieldsConfig {
//     created_at: String,
//     created_by: String,
// }
// impl Default for TimeFieldsConfig {
//     fn default() -> Self {
//         TimeFieldsConfig {
//             created_at: "create_at".to_string(),
//             created_by: "create_by".to_string(),
//         }
//     }
// }

// impl<ME, FE, EE> TimeFields<TimeFieldsConfig> for Model<ME, FE, EE>
// where
//     ME: Default,
//     FE: Default,
//     EE: Default,
// {
//     fn time_fields_with_config(self, config: TimeFieldsConfig) -> Self {
//         self.field(config.created_at, |f| f)
//             .field(config.created_by, |f| f)
//     }
// }
