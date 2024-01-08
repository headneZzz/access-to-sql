use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub access_odbc_connection: String,
    pub sql_server_odbc_connection: String,
    pub file_path: String,
}

pub fn read_config() -> Config {
    let contents = fs::read_to_string("config.toml").expect("Не найден конфиг файл config.toml");
    toml::from_str(&contents).expect("Нет необходимых параметров в конфиг файле config.toml")
}