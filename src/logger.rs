use std::fs::OpenOptions;
use log::{debug, error, log_enabled, info, Level};
use std::io::Write;

const LOG_FILE: &str = "log.txt";

pub fn logger(message: &str) {
    info!("{}", message);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(LOG_FILE)
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}