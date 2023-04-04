use std::{fs, io::{self, prelude::*}, net, thread, time};

use log::SetLoggerError;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};

/// Function to initialize logging infrastructure.
///
/// In the context of the project in Rust book's chapter 20, which was a 
/// concurrent web server with thread-pooling, it would be interesting to test
/// both terminal logging, and logging to a file, which `simplelog` allows
/// straightforwardly as can be seen below.
///
/// The default logging configuration is used, which is then modified to allow
/// source-code information on every log message, not just errors.
pub fn init_logging_infrastructure(opt_log_file_name : Option<&str>, log_level: LevelFilter) -> Result<(), SetLoggerError>{
    let config = ConfigBuilder::new()
        // This enables source-code location in logging message of any level
        .set_location_level(LevelFilter::Error)
        .build();
    let term_logger = TermLogger::new(
        // This is the field used to control the granularity of logs shown in the terminal.
        log_level,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    // Terminal logging is always used, but file_based logging will
    // depend on the log file name the program user may or may not provide.
    let mut logger_vec: Vec<Box<dyn SharedLogger>> = vec![term_logger];

    match opt_log_file_name {
        None => {
            println!("No log file name provided.");
            println!("Terminal-only logging will be done instead.");
        }
        Some(log_file_name) => {
            let log_file = fs::File::create(log_file_name);
            match log_file {
                Err(err) => {
                    eprintln!("Could not create logging file! Error: {:?}", err);
                    eprintln!("Terminal-only logging will be attempted.");
                }
                Ok(file) => {
                    let file_logger = WriteLogger::new(
                        LevelFilter::Info,
                        config,
                        file
                    );
                    logger_vec.push(file_logger);
                }
            }
        }
    };
    CombinedLogger::init(logger_vec)
}

/// This function is passed to each worker thread's closure, so that they may concurrently
/// serve the various requests in the `net::TcpStream` passed via the closure.
pub fn handle_connection(mut stream: net::TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename)?;

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()
}
