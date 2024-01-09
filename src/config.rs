use std::fs;
use serde::Deserialize;
use crate::utils::exit_gracefully_with_error_code;

#[derive(Deserialize)]
pub struct Config {
    pub access_odbc_connection: String,
    pub sql_server_odbc_connection: String,
    pub file_path: String,
}

pub fn read_config() -> Config {
    let contents = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Ошибка при чтении конфиг файла config.toml: {}", e);
            exit_gracefully_with_error_code();
        }
    };

    toml::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("Ошибка при чтении необходимых параметров в конфиг файле config.toml: {}", e);
        exit_gracefully_with_error_code();
    })
}