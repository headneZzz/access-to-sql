use std::fs;
use std::fs::File;
use std::path::Path;
use chrono::Local;
use log::{LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TerminalMode, TermLogger, WriteLogger};

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
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Stderr,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            config,
            log_file,
        ),
    ]).expect("Ошибка при инициализации логирования");
}