use crate::enums::ColTypesEnum;

///
/// Stucts for modelling the database schema
pub struct DataTable {
    pub table_name: String,
    pub columns: Vec<DataColumn>,
}

impl DataTable {
    pub fn new(table_name: String, columns: Vec<DataColumn>) -> DataTable {
        DataTable {
            table_name: table_name,
            columns: columns,
        }
    }
}

pub struct DataColumn {
    pub col_type: ColTypesEnum,
    pub col_name: String,
}

impl DataColumn {
    pub fn new(col_type: ColTypesEnum, col_name: String) -> DataColumn {
        DataColumn {
            col_type: col_type,
            col_name: col_name,
        }
    }
}