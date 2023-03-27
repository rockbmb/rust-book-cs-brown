use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    //
    // The bind function in this scenario works like the new function in that it
    // will return a new TcpListener instance.
    //
    // The function is called bind because, in networking, connecting to a port
    // to listen to is known as “binding to a port.”
    //
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream : TcpStream) {
    // new BufReader instance that wraps a mutable reference to the stream.
    // BufReader adds buffering by managing calls to the std::io::Read trait
    // methods for us.
    let buf_reader = BufReader::new(&mut stream);

    // We indicate that we want to collect these lines in a vector by adding the
    // Vec<_> type annotation.
    let http_request : Vec<_> = buf_reader
        // BufReader implements the std::io::BufRead trait, which provides the
        // lines method. The lines method returns an iterator of Result<String,
        // std::io::Error> by splitting the stream of data whenever it sees a
        // newline byte.
        .lines()
        // To get each String, we map and unwrap each Result. The Result might
        // be an error if the data isn’t valid UTF-8 or if there was a problem
        // reading from the stream
        .map(|result| result.unwrap())
        // The browser signals the end of an HTTP request by sending two newline
        // characters in a row, so to get one request from the stream, we take
        // lines until we get a line that is the empty string.
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Then we call as_bytes on our response to convert the string data to
    // bytes. The write_all method on stream takes a &[u8] and sends those bytes
    // directly down the connection. Because the write_all operation could fail,
    // we use unwrap on any error result as before. Again, in a real application
    // you would add error handling here.
    stream.write_all(response.as_bytes()).unwrap();
}