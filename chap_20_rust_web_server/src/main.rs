use chap_20_rust_web_server::{ThreadPool, util};

use std::{net::TcpListener, process};

fn main() {
    // Setup logging infra
    let log_file_name = Some("rust_web_server.log");
    util::init_logging_infrastructure(log_file_name).unwrap_or_else(|err| {
        eprintln!("Could not init logging infrastructure! Error: {:?}", err);
        eprintln!("Exiting");
        std::process::exit(1);
    });

    //
    // The function is called bind because, in networking, connecting to a port
    // to listen to is known as “binding to a port.”
    //
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
        simplelog::error!(
            "Problem creating TCP listener on address \"{}\". Error: {:?}",
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
        simplelog::error!("Problem creating the server's threadpool: {:?}", err);
        process::exit(1);
    });

    // The `take(3)` is to simulated a server being shutdown while it is
    // serving requests, to test graceful termination. Remove it if unneeded.
    for stream in listener.incoming().take(3) {
        let stream = match stream {
            Err(err) => {
                simplelog::error!("Could not read from TCP stream. Error: {:?}", err);
                process::exit(1);
            }
            Ok(s) => s,
        };

        let execution_res = pool.execute(|| util::handle_connection(stream));

        match execution_res {
            Err(err) => simplelog::warn!("problem sending job to pool; {:?}", err),
            _ => continue,
        }
    }
}