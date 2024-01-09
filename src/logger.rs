use std::fs;
use std::fs::File;
use std::path::Path;
use chrono::Local;
use log::{LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TerminalMode, TermLogger, WriteLogger};
use crate::utils::exit_gracefully_with_error_code;

pub fn config_logger() {
    let log_dir = Path::new("log");

    if !log_dir.exists() {
        fs::create_dir_all(log_dir).unwrap();
    }
    let now = Local::now();
    let log_filename = format!("{}_{}.log", "access_to_sql", now.format("%Y-%m-%d_%H-%M-%S"));
    let new_log = log_dir.join(log_filename);

    let log_file = File::create(new_log).unwrap();

    let config = ConfigBuilder::new().set_time_offset_to_local().unwrap().build();
    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Stderr,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            config,
            log_file,
        ),
    ]) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Ошибка при инициализации логирования: {}", e);
            exit_gracefully_with_error_code();
        }
    }
}