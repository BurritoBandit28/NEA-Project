use std::time::SystemTime;
use log::{Level, LevelFilter, Metadata, Record};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use num::integer::sqrt;
use num::pow;

pub fn mul_vec(vec : &mut (f32, f32), val : f32) {
    vec.0 *= val;
    vec.1 *= val;
}

pub fn normalise_vec(vec : &mut (f32, f32)) {
    // get the square root of the object
    let mag = f32::sqrt(vec.0 * vec.0) + (vec.1 * vec.1);
    vec.0 /= mag;
    vec.1 /= mag;
}


struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {

    // following code from https://github.com/estk/log4rs/blob/main/examples/log_to_file.rs
    // slightly edited

    let level = log::LevelFilter::Info;
    let file_path = format!("log/{:}.log", chrono::offset::Local::now().to_string().replace(" ", "_").replace(":", "-"));

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);
}