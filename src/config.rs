use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub access_odbc_connection: String,
    pub sql_server_odbc_connection: String,
    pub file_path: String,
}