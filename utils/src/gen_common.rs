use crate::enums::ColTypesEnum;

//Add structs and their implementations to each of the created files
pub fn get_col_type(col_type: &str) -> ColTypesEnum {
    match col_type {
        "VARCHAR" => ColTypesEnum::VARCHAR,
        "INTEGER" => ColTypesEnum::INTEGER,
        "FLOAT" => ColTypesEnum::FLOAT,
        "UUID" => ColTypesEnum::UUID,
        "BOOLEAN" => ColTypesEnum::BOOLEAN,
        _ => ColTypesEnum::VARCHAR,
    }
}

//Add structs and their implementations to each of the created files
pub fn get_data_type(col_type: ColTypesEnum) -> String {
    match col_type {
        ColTypesEnum::VARCHAR => "String".to_string(),
        ColTypesEnum::INTEGER => "i32".to_string(),
        ColTypesEnum::FLOAT => "f64".to_string(),
        ColTypesEnum::UUID => "uuid::uuid".to_string(),
        ColTypesEnum::BOOLEAN => "bool".to_string(),
    }
}
