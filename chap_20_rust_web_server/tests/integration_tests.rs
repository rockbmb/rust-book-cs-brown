use chap_20_rust_web_server as server;
use chap_20_rust_web_server::util as util;

use std::{fs, io::{self, Write}, time, thread};

fn message(filename: &str) -> String {
    format!("Hello, {filename}!\n")
}

/// Create a file with content given by the `message` function above, after at least
/// the given amount of time has elapsed.
fn concurrent_create_file(filename: &str, worker_delay_millis: u64) -> io::Result<()> {
    let mut fd = fs::File::create(filename).unwrap();
    thread::sleep(time::Duration::from_millis(worker_delay_millis));

    fd.write_all(message(filename).as_bytes())
}

#[test]
/// The idea for this test is to ensure the thread pool handles requests in
/// a truly concurrent way.
///
/// The structure is as follows:
/// * A thread pool with size `N > 0` is created
/// * `N` requests are made to the pool that a file be created,
///   with unique content w.r.t. other worker's files
/// * The request, encoded in `concurrent_create_file`, can only occur after
///   a given amount of time, `t \in ]0, 2]` seconds
/// * Note: `t` can be changed from test to test, but in a given run is the sam
///   for all workers.
/// * Given some `ε <<< t`, the test verifies that every file has correct
///   content after `t + ε` time
///
/// This can only occur if the the requests are indeed being processed simultaneously.
/// Were this not the case, the test would fail, only passing after `N * t + ε` time
/// had elapsed.
fn thread_pool_concurrency() {
    util::init_logging_infrastructure(None).unwrap();

    let thread_pool_size: usize = 5;
    let pool = server::ThreadPool::build(thread_pool_size).unwrap();

    // This vector will contain the filenames that will be individually passed to
    // each worker in the thread pool.
    let mut filenames: Vec<String> = Vec::new();

    for char in ('a'..='z')
        .take(thread_pool_size) {
        let mut filename = char.to_string();
        filename.push_str(".txt");
        filenames.push(filename);
    }

    // This is the delay passed to `concurrent_create_file` to ensure
    // the file is only created by the worker thread after that many milliseconds.
    let worker_delay = 1000;
    let main_thread_delay = 100;

    for filename in filenames.clone() {
        pool
            .execute(move ||
                concurrent_create_file(&filename, worker_delay)
            )
            .unwrap();
    }

    // Have the main thread block for at least as long as the worker threads,
    // to guarantee all the OS syscalls for file creation have been processed,
    // and every worker's file exists.
    thread::sleep(time::Duration::from_millis(worker_delay + main_thread_delay));

    for filename in filenames {
        let expected = message(&filename);
        let actual = fs::read_to_string(filename);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!(expected, actual);
    }
}