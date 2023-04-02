use chap_20_rust_web_server::ThreadPool;
use log::SetLoggerError;
use std::{
    fs,
    io::{self, prelude::*},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};

fn init_logging_infrastructure(log_file_name : &str) -> Result<(), SetLoggerError>{
    // Init logging infrastructure. Just for curiosity's sake, we'll combine both
    // terminal and file logging.
    let log_file = fs::File::create(log_file_name);
    let term_logger = TermLogger::new(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    let mut logger_vec: Vec<Box<dyn SharedLogger>> = vec![term_logger];
    match log_file {
        Err(err) => {
            eprintln!("main: Could not create logging file! Error: {:?}", err);
            eprintln!("main: Terminal-only logging will be attempted.");
        }
        Ok(file) => {
            let file_logger = WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                file
            );
            logger_vec.push(file_logger);
        }
    };
    CombinedLogger::init(logger_vec)
}

fn main() {
    let log_file_name = "rust_web_server.log";
    match init_logging_infrastructure(log_file_name) {
        Err(err) =>  {
            eprintln!("main: Could not init logging infrastructure! Error: {:?}", err);
            eprintln!("main: Exiting");
            std::process::exit(1);
        }
        Ok(_) => {}
    }

    //
    // The function is called bind because, in networking, connecting to a port
    // to listen to is known as “binding to a port.”
    //
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
        eprintln!(
            "main: Problem creating TCP listener on address \"{}\". Error: {:?}",
            addr, err
        );
        process::exit(1);
    });

    // The below was left inside the `for` loop by mistake, which caused
    // many problems.
    //
    // Ideally we don't create a thread pool with every new request,
    // causing the old one to dropped with the every iteration of the loop below,
    // and then have to fix cryptic `Recv/PoisonError` problems :)
    let pool = ThreadPool::build(4).unwrap_or_else(|err| {
        eprintln!("main: Problem creating the server's threadpool: {:?}", err);
        process::exit(1);
    });

    // The `take(3)` is to simulated a server being shutdown while it is
    // serving requests, to test graceful termination. Remove it if not needed.
    for stream in listener.incoming().take(3) {
        let stream = match stream {
            Err(err) => {
                eprintln!("main: could not read from TCP stream. Error: {:?}", err);
                process::exit(1);
            }
            Ok(s) => s,
        };

        let execution_res = pool.execute(|| handle_connection(stream));

        match execution_res {
            Err(err) => eprintln!("main: problem sending job to pool; {:?}", err),
            _ => continue,
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
