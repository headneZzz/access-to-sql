mod config;
mod logger;

mod db {
    pub mod access;
    pub mod sql_server;
}

mod models {
    pub mod access;
    pub mod sql_server;
}

mod processors {
    pub mod access_to_sql;
}

use std::fs;
use std::str::Lines;
use odbc::*;
use crate::config::read_config;
use crate::logger::config_logger;
use crate::processors::access_to_sql::process;

fn main() {
    config_logger();
    let config = read_config();
    let file_content = fs::read_to_string(config.file_path).expect("Не найден файл с фондами и описями");
    let lines: Lines = file_content.lines();

    let env = create_environment_v3_with_os_db_encoding("windows-1251", "windows-1251").unwrap();
    let access_conn = env.connect_with_connection_string(&*config.access_odbc_connection).unwrap();
    let sql_server_conn = env.connect_with_connection_string(&config.sql_server_odbc_connection).unwrap();

    process(lines, access_conn, sql_server_conn);
}
