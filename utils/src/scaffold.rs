#![allow(dead_code)]
use std::fs;
use std::{fs::File, io::Write};

use crate::{
    gen_common::get_col_type,
    structs::{DataColumn, DataTable},
};

//Static variables
static CREATE_TABLE: &str = "CREATE TABLE";
static EXISTS: &str = "EXISTS";

//Eventually these will come from the args when running the program.
static SQL_FILE: &str = "init_db.sql";
static ATTRIBUTE_FILE: &str = "Templates/attributes.txt";
static STRUCT_TEMPLATE_FILE: &str = "Templates/struct_template.txt";

pub fn read_files() {
    println!("---- Reading SQL Schema from: {} ----", SQL_FILE);

    let contents = fs::read_to_string(SQL_FILE).expect("Error reading the SQL file.");
    println!("---- Reading file from database schema ----");

    let tables = read_db_init_file(&contents);

    let create_file_result = write_to_rust_file(&tables);
    match create_file_result {
        Ok(_) => println!("---- Successfully created Rust file: {} ----", SQL_FILE),
        Err(e) => println!("---- Error creating Rust file: {} ----", e),
    }

    let create_file_result = generate_mod_file(&tables);
    match create_file_result {
        Ok(_) => println!("---- Successfully created Struct Mod file ----"),
        Err(e) => println!("---- Error creating Mod file: {} ----", e),
    }
}

///---------------------------------------------------------------------------------------------------------------
/// Reads the database file and creates the rust structs as defined by the database.
///---------------------------------------------------------------------------------------------------------------
fn read_db_init_file(sql_file: &str) -> Vec<DataTable> {
    //-----Tables-----
    let mut tables: Vec<DataTable> = Vec::new();
    let raw_tables: Vec<&str> = sql_file.split(EXISTS).collect(); //Gets the table name.
    let count: usize = raw_tables.len();
    println!("Table Count: {}", count);

    for table_as_string in raw_tables {
        let table_details: Vec<&str> = table_as_string.split("(").collect();
        let table_name: &str = table_details[0].trim();

        if table_name.to_string().is_empty() {
            continue;
        }

        //-----Columns-----
        println!("-------- {} --------", table_name);
        let mut table_cols: Vec<&str> = table_as_string.trim().split(",").collect();
        table_cols.remove(0);
        table_cols.pop();

        let mut columns: Vec<DataColumn> = Vec::new();

        for col in table_cols {
            // println!("col Details: {}", col.trim_start());
            let col_details_sql: Vec<&str> = col.trim_start().split_ascii_whitespace().collect();

            if col_details_sql.len() < 2 {
                panic!(
                    "Column in {} is missing type. Please fix the table. ",
                    table_name
                )
            }

            let col_type = get_col_type(col_details_sql[1]);

            columns.push(DataColumn::new(col_type, col_details_sql[1].to_string()))
        }
        tables.push(DataTable::new(table_name.to_owned(), columns))
    }

    tables
}

///---------------------------------------------------------------------------------------------------------------
/// Writes the structs as the rust file.
///---------------------------------------------------------------------------------------------------------------
fn write_to_rust_file(sql_tables: &Vec<DataTable>) -> std::io::Result<()> {
    let path = "DataStructs/";

    for table in sql_tables {
        let file_path: String = path.to_owned() + &table.table_name + ".rs";
        let mut file = File::create_new(file_path)?;
        let struct_details = generate_struct_data(&table);
        file.write(struct_details.as_bytes())?;
    }
    Ok(())
}

///---------------------------------------------------------------------------------------------------------------
/// Generates the struct data for the rust file.
///--------------------------------------------------------------------------------------------------------------
fn generate_struct_data(table: &DataTable) -> String {
    let mut struct_data: String = "".to_string();
    struct_data.push_str(&format!(
        "{}\\n
        pub struct {} {{\n
        }}",
        add_template_data(),
        table.table_name
    ));
    struct_data
}

fn add_template_data() -> String {
    fs::read_to_string(ATTRIBUTE_FILE).expect("There was an error reading the template file.")
}

///---------------------------------------------------------------------------------------------------------------
/// Generates the mod file for the rust file.
/// --------------------------------------------------------------------------------------------------------------
fn generate_mod_file(sql_tables: &Vec<DataTable>) -> std::io::Result<()> {
    let path = "DataStructs/".to_owned();
    let mut file = File::create_new(path + "mod.rs")?;

    for table in sql_tables {
        file.write(format!("pub mod {};", table.table_name).as_bytes())?;
    }
    Ok(())
}
