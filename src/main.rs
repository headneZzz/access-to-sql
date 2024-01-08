mod config;

mod models {
    pub mod access;
    pub mod sql_server;
}

mod db {
    pub mod access;
    pub mod sql_server;
}

use std::fs;
use odbc::*;
use config::Config;
use crate::db::sql_server::TblUnit;

fn main() {
    env_logger::init();

    let contents = fs::read_to_string("config.toml").expect("Should have been able to read the file");
    let config: Config = toml::from_str(&contents).expect("Should have been able to parse the file");

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let env = create_environment_v3_with_os_db_encoding("windows-1251", "windows-1251").unwrap();
    let access_conn = env.connect_with_connection_string(&*config.access_odbc_connection).unwrap();
    let sql_server_conn = env.connect_with_connection_string(&config.sql_server_odbc_connection).unwrap();

    for line in lines {
        let fund_inventory: Vec<&str> = line.split_whitespace().collect();
        let fund_num = fund_inventory[0];
        let inventory_num = fund_inventory[1];
        let mut isn_unit = db::sql_server::get_max_isn_unit(&sql_server_conn).unwrap();

        let access_units = db::access::fetch_access_data(&access_conn, fund_num, inventory_num).expect("Ошибка при получении данных из MS Access");
        let isn_codes = db::sql_server::get_isn_codes(&sql_server_conn, fund_num, inventory_num).expect("Ошибка при получении данных из MS SQL Server");
        let units: Vec<TblUnit> = access_units.iter().map(|access_unit| {
            isn_unit += 1;
            TblUnit::new(&isn_codes, access_unit, isn_unit)
        }).collect();
        db::sql_server::insert_new_units(&sql_server_conn, units);
    }
}