use devmodel_codegen::mysql;
use devmodel_core::types::*;
use devmodel_core::*;

use std::fmt::Debug;

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

pub trait ModelHelper {
    fn time_fields(self) -> Self;
}

impl<ME, FE, EE> ModelHelper for Model<ME, FE, EE>
where
    ME: Default,
    FE: Default + Extension<mysql::Column>,
    EE: Default,
{
    fn time_fields(self) -> Self {
        self.field("created_at", |f| {
            f.ty(Usize::new()).extension(mysql::Column::timestamp(6))
        })
        .field("updated_at", |f| {
            f.ty(Usize::new()).extension(mysql::Column::timestamp(6))
        })
    }
}

fn main() {
    let domain = Domain::<(), (), FieldExt, ()>::new("School")
        .model(
            Model::new("Teacher")
                .field("tid", |f| {
                    f.ty(Usize::new()).extension(mysql::Column::int(20))
                })
                .field("name", |f| {
                    f.ty(Str::new()).extension(mysql::Column::varchar(256))
                })
                .time_fields(),
        )
        .model(
            Model::new("Course")
                .field("cid", |f| {
                    f.ty(Usize::new()).extension(mysql::Column::int(20))
                })
                .field("title", |f| {
                    f.ty(Str::new()).extension(mysql::Column::varchar(256))
                })
                .field("is_remote", |f| {
                    f.ty(Bool::new()).extension(mysql::Column::tinyint(1))
                })
                .time_fields(),
        )
        .model(
            Model::new("Student")
                .field("sid", |f| {
                    f.ty(Usize::new()).extension(mysql::Column::int(20))
                })
                .field("name", |f| {
                    f.ty(Str::new()).extension(mysql::Column::varchar(256))
                })
                .field("year", |f| {
                    f.ty(U8::new()).extension(mysql::Column::tinyint(1))
                })
                .time_fields(),
        );

    let _ = mysql::Schema.generate(&domain);
}
