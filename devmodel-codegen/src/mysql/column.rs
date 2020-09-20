use crate::mysql::types::Type;
use devmodel_core::{Extension, Field};

#[derive(Debug)]
pub struct Column {
    pub ty: Type,
}

impl Column {
    pub fn ty(ty: Type) -> Column {
        Column { ty }
    }
}

// Note: extend Field<E>
// pub trait ColumnExtension<FE> {
//     fn mysql_column<F>(self, col: Column) -> Field<FE>;
// }

// impl<FE> ColumnExtension<FE> for Field<FE>
// where
//     FE: Extension<Column>,
// {
//     fn mysql_column<F>(mut self, col: Column) -> Field<FE> {
//         self.extensions.set(col);
//         self
//     }
// }

pub fn int(size: usize) -> Column {
    Column::ty(Type::INT(size))
}
