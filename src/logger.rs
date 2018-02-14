use log::{Level, Log, Metadata, Record};
use colored::*;

pub struct ColoredTermLogger {
    level: Level,
    #[allow(dead_code)]
    colored: bool,
}

impl ColoredTermLogger {
    pub fn new(level: Level, colored: bool) -> Self {
        ColoredTermLogger { level, colored }
    }

    pub fn with_level(level: Level) -> Self {
        Self::new(level, !cfg!(windows))
    }
}

impl Default for ColoredTermLogger {
    fn default() -> Self {
        Self::with_level(Level::Warn)
    }
}

impl Log for ColoredTermLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    // TODO: Disable colors if colored == false
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Trace => eprintln!("{} {}", "debug:".white().bold(), record.args()),
                Level::Debug => eprintln!("{} {}", "info:".blue().bold(), record.args()),
                Level::Info => eprintln!("{} {}", "success:".green().bold(), record.args()),
                Level::Warn => eprintln!("{} {}", "warn:".yellow().bold(), record.args()),
                Level::Error => eprintln!("{} {}", "error:".red().bold(), record.args()),
            }
        }
    }

    fn flush(&self) {}
}
