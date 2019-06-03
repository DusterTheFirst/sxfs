use log::{Record, Level, Metadata};
use log::{SetLoggerError, LevelFilter};
use colored::*;

/// The logger for SXFS
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}{}", record.target(), match record.level() {
                Level::Trace => "TRACE: ".purple(),
                Level::Debug => "DEBUG: ".cyan(),
                Level::Info => "INFO: ".bright_black(),
                Level::Warn => "WARN: ".yellow(),
                Level::Error => "ERROR: ".red()
            }, record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level))
}