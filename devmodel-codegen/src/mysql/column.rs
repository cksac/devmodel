use crate::mysql::types::Type::{self, *};

#[derive(Debug)]
pub struct Column {
    pub ty: Type,
}

impl Column {
    pub fn ty(ty: Type) -> Column {
        Column { ty }
    }

    pub fn int(size: usize) -> Column {
        Column::ty(INT(size))
    }

    pub fn tinyint(size: usize) -> Column {
        Column::ty(TINYINT(size))
    }

    pub fn varchar(size: usize) -> Column {
        Column::ty(VARCHAR(size))
    }

    pub fn timestamp(fsp: usize) -> Column {
        Column::ty(TIMESTAMP(fsp))
    }
}
