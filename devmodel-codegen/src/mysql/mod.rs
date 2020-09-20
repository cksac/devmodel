use devmodel_core::*;

// TODO: add validator to check params
#[derive(Debug)]
pub enum SqlType {
    CHAR(usize),
    VARCHAR(usize),
    BINARY(usize),
    VARBINARY(usize),
    TINYBLOB,
    TINYTEXT,
    TEXT(usize),
    BLOB(usize),
    MEDIUMTEXT,
    MEDIUMBLOB,
    LONGTEXT,
    LONGBLOB,
    ENUM { values: Vec<String> },
    SET { values: Vec<String> },
    BIT(usize),
    TINYINT(usize),
    BOOL,
    SMALLINT(usize),
    MEDIUMINT(usize),
    INT(usize),
    BIGINT(usize),
    FLOAT(usize, usize),
    DOUBLE(usize, usize),
    DECIMAL(usize, usize),
    DATE,
    DATETIME,
    TIMESTAMP,
    TIME,
    YEAR,
}

#[derive(Debug)]
pub struct FieldExt {
    sql_ty: SqlType,
}

impl FieldExt {
    pub fn sql_ty(t: SqlType) -> FieldExt {
        FieldExt { sql_ty: t }
    }
}

pub trait MySqlFieldExtension<FE> {
    fn mysql_int(self, size: usize) -> Field<FE>;
}

impl<FE> MySqlFieldExtension<FE> for Field<FE>
where
    FE: Extension<FieldExt>,
{
    fn mysql_int(mut self, size: usize) -> Field<FE> {
        let ext = FieldExt::sql_ty(SqlType::INT(size));
        self.extensions.set(ext);
        self
    }
}

pub struct Schema;
impl<DE, ME, FE, EE> Generator<DE, ME, FE, EE> for Schema
where
    FE: Extension<FieldExt>,
{
    type Output = ();
    type Error = ();
    fn generate(&mut self, domain: &Domain<DE, ME, FE, EE>) -> Result<Self::Output, Self::Error> {
        for (_, model) in domain.models.iter() {
            println!("Table {}", model.name);
            for field in model.fields.iter() {
                if let Some(ext) = field.extensions.get() {
                    println!(
                        "\t- {}, optional: {}, sql_ty: {:?}",
                        field.name, field.optional, ext.sql_ty
                    );
                }
            }
        }
        Ok(())
    }
}
