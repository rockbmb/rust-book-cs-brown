//! # Mini Web Server using Rust
//!
//! This crate is an implementation of the [Rust book's](https://rust-book.cs.brown.edu/ch20-00-final-project-a-web-server.html)
//! final project, a concurrent web server that serves simple requests with basic HTML.
//!
//! It showcases some common Rust techniques such as `Arc + Mutex`, `mpsc::channel`,
//! and shared ownership in a (thread) concurrency setting.

use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// A `ThreadPool`'s individual worker.
///
/// Each is assigned a `usize` ID, and the handle of the spawned thread assigned to it.
pub struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

#[derive(Debug)]
pub enum WorkerError {
    ThreadCreationError(io::Error),
}

/// Helper used to contain the closure each worker thread is spawned with.
///
/// Having so much code inline makes it hard to understand what is part of
/// `Worker::build`, and what is the thread's spawning closure.
fn worker_func(id: usize, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        // IMPORTANT
        // The `.lock()` must be immediately followed by `.unwrap()`, or sequential behavior will
        // be observable.
        //
        // This is because there is no `.unlock()` method - the lock is held as long as the
        // corresponding `MutexGuard`'s lifetim:
        // * in the below case, it starts and ends in this line, so it is released
        //   as soon as the line is executed.
        // * in the case where `match job_receiver.lock() { ... }` to handle possible `PoisonError`s,
        //   the lock will be held for much longer than expected! - possibly until after the thread
        //   has finished running its `job()`.
        let msg = job_receiver.lock().unwrap().recv();
        match msg {
            Err(_) => {
                eprintln!(
                    "Worker thread {} disconnected due to closure of job queue; shutting down.",
                    id,
                );
                break;
            }
            Ok(job) => {
                println!("Worker {id} got a job; executing");
                let job_result = job();
                match job_result {
                    Err(err) => eprintln!("Worker {id} failed a job with error: {:?}", err),
                    Ok(_) => println!("Worker {id} successfully completed a job."),
                }
            }
        }
    }
}

impl Worker {
    /// Create a new worker to handle requests from the server.
    ///
    /// Fails with `WorkerError` if it's not possible to start the worker's thread
    /// due to OS issues.
    ///
    /// # Panics
    ///
    /// This function doesn't panic.
    fn build(
        id: usize,
        job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, WorkerError> {
        let builder = thread::Builder::new().name(format!("Worker-{}", id));

        let thread_res = builder.spawn(move || worker_func(id, job_receiver));

        match thread_res {
            Ok(thread_handle) => {
                let thread_handle = Some(thread_handle);
                Ok(Worker {
                    id,
                    handle: thread_handle,
                })
            }
            Err(err) => Err(WorkerError::ThreadCreationError(err)),
        }
    }
}

/// Type of closures that will be sent by the main server thread to
/// its child worker threads, reprenting the computation it wishes them to perform.
///
/// Having an explicit `io::Result` return value is not needed if the result is simply
/// `unwrap`ped, but it seems like a good exercise to do this.
type Job = Box<dyn FnOnce() -> io::Result<()> + Send + 'static>;

#[derive(Debug)]
pub enum ThreadPoolError {
    ZeroThreadThreadPoolError,
    WorkerError(WorkerError),
    InexistentJobSenderError,
    JobTransmissionError(mpsc::SendError<Job>),
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    job_sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// * If the user selects an invalid thread pool size, the `Err` variant
    ///   is returned;
    /// * otherwise, a thread pool with the specified size is returned via `Ok`
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size == 0 {
            return Err(ThreadPoolError::ZeroThreadThreadPoolError);
        }

        let (job_sender, job_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let mut workers = Vec::with_capacity(size);

        for n in 0..size {
            let worker_res = Worker::build(n, Arc::clone(&job_receiver));
            // If even one of the workers could not be created, fail and exit early.
            match worker_res {
                Ok(worker) => workers.push(worker),
                Err(w_err) => return Err(ThreadPoolError::WorkerError(w_err)),
            }
        }

        Ok(ThreadPool {
            workers,
            job_sender: Some(job_sender),
        })
    }

    /*
    signature for `std::thread::spawn`, to serve as a possible starting point for `execute`.

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    */

    /// This method is used to request the `ThreadPool` to assign the given task
    /// to one of its threads, FCFS via the `mpsc` channel whose reading end each thread
    /// has access to.
    ///
    /// We can be further confident that FnOnce is the trait we want to use in the job's type
    /// because the thread for running a request will only execute that request’s
    /// closure one time, which matches the Once in FnOnce.
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        // We still use the () after FnOnce because this FnOnce represents a
        // closure that takes no parameters and returns the unit type ()
        F: FnOnce() -> io::Result<()> + Send + 'static,
    {
        let job = Box::new(f);

        let sender = match self.job_sender.as_ref() {
            None => return Err(ThreadPoolError::InexistentJobSenderError),
            Some(s) => s,
        };
        sender
            .send(job)
            .map_err(ThreadPoolError::JobTransmissionError)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // The sending half of the channel, residing in the main thread,
        // is dropped, so that the worker threads try to read from their
        // end of the channel and get a `ReceiveError`, they'll know it is
        // time to shut themselves down.
        std::mem::drop(self.job_sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                let this_id = thread::current().id();
                let thread_id = handle.thread().id();
                match handle.join() {
                    Err(err) => {
                        eprintln!(
                            "impl Drop for ThreadPool: thread {:?} failed to join on thread {:?}; error: {:?}",
                            this_id,
                            thread_id,
                            err
                        )
                    }
                    Ok(val) => {
                        println!(
                            "impl Drop for ThreadPool: thread {:?} successfully joined thread {:?} with result {:?}",
                            this_id,
                            thread_id,
                            val
                        )
                    }
                }
            }
        }
    }
}
