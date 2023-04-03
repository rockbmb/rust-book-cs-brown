use chap_20_rust_web_server as server;

use std::{collections::HashMap, fs, io::{self, Write}, time, thread};

fn message(filename: &str, worker_id: usize) -> String {
    format!("Hello, {filename}!\n This is worker {worker_id}.\n")
}

fn concurrent_create_file(filename: &str, worker_id: usize, worker_delay_millis: u64) -> io::Result<()> {
    let mut fd = fs::File::create(filename).unwrap();
    thread::sleep(time::Duration::from_millis(worker_delay_millis));

    fd.write_all(message(filename, worker_id).as_bytes())
}

#[test]
fn thread_pool_concurrency() {
    let thread_pool_size: usize = 5;
    let pool = server::ThreadPool::build(thread_pool_size).unwrap();

    let mut filenames: HashMap<usize, String> = HashMap::new();

    for (ix, char) in ('a'..='z')
        .take(thread_pool_size)
        .enumerate() {
        let mut filename = char.to_string();
        filename.push_str(".txt");
        filenames.insert(ix, filename);
    }

    let mut sorted_filenames = filenames.into_iter().collect::<Vec<_>>();
    sorted_filenames.sort_by_key(|tup| tup.0);

    let filenames: Vec<(usize, String)> = sorted_filenames.clone();

    let worker_delay = 1000;
    let main_thread_delay = 200;

    for (ix, filename) in filenames.clone() {
        pool.execute(move ||
            concurrent_create_file(&filename, ix, worker_delay)
        ).unwrap();
    }

    thread::sleep(time::Duration::from_millis(worker_delay + main_thread_delay));

    for (ix, filename) in filenames {
        let expected = message(&filename, ix);
        let actual = fs::read_to_string(filename);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!(expected, actual);
    }
}