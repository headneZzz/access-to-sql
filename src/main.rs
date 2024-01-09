mod config;
mod logger;
mod utils;

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
use crate::utils::exit_gracefully_with_error_code;

fn main() {
    config_logger();
    let config = read_config();
    let file_content = match fs::read_to_string(config.file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Ошибка при чтении файла с фондами и описями: {}", e);
            exit_gracefully_with_error_code();
        }
    };
    let lines: Lines = file_content.lines();

    let env = create_environment_v3_with_os_db_encoding("windows-1251", "windows-1251").unwrap();
    let access_conn = match env.connect_with_connection_string(&*config.access_odbc_connection) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Ошибка при подключении к MS Access файлу: {}", e);
            exit_gracefully_with_error_code();
        }
    };
    let sql_server_conn = match env.connect_with_connection_string(&config.sql_server_odbc_connection) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Ошибка при подключении к MS SQL Server: {}", e);
            exit_gracefully_with_error_code();
        }
    };

    process(lines, access_conn, sql_server_conn);
}