use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process,
    thread,
    time::Duration
};
use chap_20_rust_web_server::ThreadPool;

fn main() {
    //
    // The function is called bind because, in networking, connecting to a port
    // to listen to is known as “binding to a port.”
    //
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Desired API:
    // Also, ideally we don't create a thread pool with every new request,
    // causing the old one to dropped with the every iteration of the loop below,
    // and then have to fix cryptic `Recv/PoisonError` problems :)
    let pool = ThreadPool::build(4).unwrap_or_else(|err| {
        eprintln!("main: Problem creating the server's threadpool: {:?}", err);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let execution_res = pool.execute(|| {
            handle_connection(stream);
        });

        match execution_res {
            Err(err) =>
                eprintln!("main: problem sending job to pool; {:?}", err),
            _ => {}
        }
    }
}

fn handle_connection(mut stream : TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

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

    let contents = fs::read_to_string(filename).unwrap();

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}