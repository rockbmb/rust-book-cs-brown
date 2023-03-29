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
    // The bind function in this scenario works like the new function in that it
    // will return a new TcpListener instance.
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
        /*
        How the code would look if we had opted for the naive technique of spawning
        a unique thread per connection request, unbound both in the sense of each thread
        being unbound, and of the server not being bound either by limitations on it, or the OS'
        resources, to be prevented from doing this:

        thread::spawn(|| {
            handle_connection(stream);
        });
        */

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
    // new BufReader instance that wraps a mutable reference to the stream.
    // BufReader adds buffering by managing calls to the std::io::Read trait
    // methods for us.
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html") 
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}