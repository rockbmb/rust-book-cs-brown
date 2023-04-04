use chap_20_rust_web_server::{ThreadPool, util};

use std::{net::TcpListener, process};

fn main() {
    // Setup logging infra
    let log_file_name = Some("rust_web_server.log");
    util::init_logging_infrastructure(log_file_name, log::LevelFilter::Trace)
        .unwrap_or_else(|err| {
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
            "Problem creating TCP listener on address \"{addr}\". Error: {:?}",
            err
        );
        simplelog::error!("Exiting");
        process::exit(1);
    });

    let thread_pool_size = 4;

    // The below was left inside the `for` loop by mistake, which caused
    // many problems.
    //
    // Ideally we don't create a thread pool with every new request,
    // causing the old one to dropped with the every iteration of the loop below,
    // and then have to fix cryptic `Recv/PoisonError` problems :)
    let pool = ThreadPool::build(thread_pool_size)
        .unwrap_or_else(|err| {
            simplelog::error!("Problem creating the server's threadpool: {:?}", err);
            simplelog::error!("Exiting");
            process::exit(1);
        });

    // The `take(3)` is to simulated a server being shutdown while it is
    // serving requests, to test graceful termination. Remove it if unneeded.
    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap_or_else(|err| {
            simplelog::error!("Could not read from TCP stream. Error: {:?}", err);
            simplelog::error!("Exiting");
            process::exit(1);
        });

        let execution_res = pool.execute(|| util::handle_connection(stream));

        execution_res.unwrap_or_else(|err| {
            simplelog::warn!("problem sending job to pool; {:?}", err)
        });
    }
}