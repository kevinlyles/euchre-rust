use log::{Level, Metadata, Record};

pub(crate) struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        println!("{} - {}", record.level(), record.args())
    }

    fn flush(&self) {}
}
