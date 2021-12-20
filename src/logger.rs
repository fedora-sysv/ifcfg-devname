use log::LevelFilter;
use syslog::{BasicLogger, Error, Facility, Formatter3164, Logger, LoggerBackend};

pub fn init() {
    if let Ok(connection) = connect_syslog() {
        setup_syslog(connection);
    } else {
        setup_stderr_logging();
    }
}

fn connect_syslog() -> Result<Logger<LoggerBackend, Formatter3164>, Error> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "ifcfg-devname".into(),
        pid: 0,
    };

    syslog::unix(formatter)
}

fn setup_syslog(logger: Logger<LoggerBackend, Formatter3164>) {
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
}

fn setup_stderr_logging() {
    stderrlog::new().module(module_path!()).init().unwrap();
}
