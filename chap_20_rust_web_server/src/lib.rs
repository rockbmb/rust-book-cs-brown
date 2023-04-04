//! # Mini Web Server using Rust
//!
//! This crate is an implementation of the [Rust book's](https://rust-book.cs.brown.edu/ch20-00-final-project-a-web-server.html)
//! final project, a concurrent web server that serves simple requests with basic HTML.
//!
//! It showcases some common Rust techniques such as `Arc + Mutex`, `mpsc::channel`,
//! and shared ownership in a (thread) concurrent setting.

use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
};
use simplelog;

pub mod util;

/// A [`ThreadPool`]'s individual worker.
///
/// Each is assigned a `usize` ID, and the handle of the spawned thread assigned to it.
pub struct Worker {
    /// ID of the worker thread. It is the responsibility of `ThreadPool::build`'s caller
    /// to ensure each worker receives a unique ID.
    id: usize,
    /// Handle of the thread assigned to this thread, spawned in `Worker::build`.
    handle: Option<thread::JoinHandle<()>>,
}

/// Enum representing possible errors when `Worker::build`ing an instance of
/// [`Worker`].
#[derive(Debug)]
pub enum WorkerBuildError {
    /// This variant occurs when it wasn't possible to spawn a thread to
    /// associate the worker with.
    ThreadCreationError(io::Error),
}

/// Function with which each worker thread is `spawn`ed.
///
/// Having so much code inline makes it hard to understand what is part of
/// `Worker::build`, and what is the thread's spawning closure, so it was moved out.
pub fn worker_func(id: usize, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) {
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
                simplelog::warn!(
                    "<yellow>Worker {id}</> disconnected due to closure of job queue; shutting down.",
                );
                break;
            }
            Ok(job) => {
                simplelog::info!("<cyan>Worker {id}</> got a job; executing");
                match job() {
                    Err(err) => simplelog::warn!("<red>Worker {id}</> failed a job with error: {:?}", err),
                    Ok(_) => simplelog::info!("<cyan>Worker {id}</> successfully completed a job."),
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
    /// # Arguments
    ///
    /// * `id: usize`: ID of the worker being built. That it is unique must be enforced
    ///    by the caller e.g. in this case, `ThreadPool::build`.
    /// * `job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>`: reading end of a channel, whose
    ///    writing end resides in [`ThreadPool`]. As all other worker threads must also have
    ///    access to it, it must be wrapped in an `Arc<Mutex<_>>`. It is through this channel
    ///    that each channel will receive [`Job`]s.
    ///
    /// # Errors
    ///
    /// If the worker thread could not be created due to e.g. OS resource exhaustion, a
    /// `WorkerThreadError` variant wrapping `io::Error` is returned. Otherwise,
    /// the worker is returned.
    ///
    /// # Panics
    ///
    /// This function doesn't panic.
    pub fn build(
        id: usize,
        job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, WorkerBuildError> {
        let builder = thread::Builder::new().name(format!("Worker-{}", id));

        let thread_res = builder
            .spawn(move || worker_func(id, job_receiver));

        match thread_res {
            Ok(thread_handle) => {
                let thread_handle = Some(thread_handle);
                Ok(Worker {
                    id,
                    handle: thread_handle,
                })
            }
            Err(err) => Err(WorkerBuildError::ThreadCreationError(err)),
        }
    }
}

/// This type represents the requests that will be made of a [`ThreadPool`]'s `Workers`.
///
/// Having an explicit `io::Result` return value is not needed if the result is simply
/// `unwrap`ped, but it seems like a good exercise to do this.
///
/// Furthermore, it enabled different kinds of requests to be made in tests:
/// * in the actual server, a [`Job`] is a connection handler on a `net::TcpStream`
/// * on tests, a [`Job`] is a request to create a file with some content
pub type Job = Box<dyn FnOnce() -> io::Result<()> + Send + 'static>;

/// This enum represents errors that can occur in `ThreadPool::build`.
#[derive(Debug)]
pub enum ThreadPoolBuildError {
    /// A [`ThreadPool`]'s thread count is of type `usize`, which while avoiding
    /// negative inputs in `build`, leaves `0 : usize` as a possibility, which will
    /// result in this error.
    ZeroThreadThreadPoolError,

    /// When `build`ing a [`ThreadPool`], its [`Worker`]s also need to be built,
    /// which may result in `WorkerError` from `Worker::build`. If even one [`Worker`]
    /// fails with that variant, `ThreadPool::build` will fail with this one,
    /// because the OS being unable to spawn a thread is a serious problem.
    WorkerError(WorkerBuildError),
}

/// Enum representing errors that can happen in `ThreadPool::execute`, in the course of a
/// request's submission and execution.
#[derive(Debug)]
pub enum ThreadPoolError {
    /// This variant is used in the unlikely (and probably impossible) event a `Job` is to
    /// `execute`d, but the [`ThreadPool`]'s writing end of the `mpsc::channel` is `Option::None`.
    InexistentJobSenderError,

    /// This variant occurs if when attempting to insert a [`Job`] into the `mpsc::channel`,
    /// the `.send` method fails.
    JobTransmissionError(mpsc::SendError<Job>),
}

/// A thread pool used to concurrently execute requests of the same type.
///
/// A request is represented by [`Job`], seen also in this module.
///
/// A thread pool consists of two parts:
/// * the workers, each containing a thread used to run a request
/// * the sending end of an `mpsc::channel`, taking into account that
///   the writing end, wrapped in `Arc<Mutex<_>>`, is passed into the closure each
///   worker thread is spawned with.
pub struct ThreadPool {
    /// Vector for workers, whose length is an argument to `ThreadPool::build`
    workers: Vec<Worker>,
    /// Sending end of a channel. It is wrapped in an `Option`, so that when
    /// the thread pool is `drop`ped, this end of the channel is `Option::take`n,
    /// signaling to the worker threads via the subsequent `mpsc::RecvError` that
    /// they must also shut down.
    job_sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size passed as arument will be the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// * If the user selects an invalid thread pool size, the `Err` variant
    ///   is returned;
    /// * otherwise, a thread pool with the specified size is returned via `Ok`
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolBuildError> {
        if size == 0 {
            return Err(ThreadPoolBuildError::ZeroThreadThreadPoolError);
        }

        let (job_sender, job_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let mut workers = Vec::with_capacity(size);

        for n in 0..size {
            let worker_res = Worker::build(n, Arc::clone(&job_receiver));
            // If even one of the workers could not be created, fail and exit early.
            match worker_res {
                Ok(worker) => workers.push(worker),
                Err(w_err) => return Err(ThreadPoolBuildError::WorkerError(w_err)),
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

    /// This method is used to request the [`ThreadPool`] to assign the given task
    /// to one of its workers in an FCFS manner, via the `mpsc` channel whose reading
    /// end each thread has access to.
    ///
    /// We can be further confident that `FnOnce` is the trait we want to use in the job's type
    /// because the thread running a request will only execute that request’s closure one time,
    /// which matches the `Once` in `FnOnce`.
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

/// In order to gracefully shutdown the thread pool and also deallocate
/// resources correctly, some care is needed with [`ThreadPool`]'s drop instance.
///
/// The sending half of the [`ThreadPool`]'s `mpsc::channel`, owned by the main thread,
/// is dropped, in order to signal the worker threads that when they read from their
/// end of the channel and get a `ReceiveError`, it is time to shut themselves down.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        simplelog::debug!("Running impl Drop for ThreadPool");

        std::mem::drop(self.job_sender.take());

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let this_id = thread::current().id();
                let thread_id = handle.thread().id();
                match handle.join() {
                    Err(err) => {
                        simplelog::error!(
                            "parent thread {:?} failed to join on thread {:?} from worker {:?}; error: {:?}",
                            this_id,
                            thread_id,
                            worker.id,
                            err
                        )
                    }
                    Ok(val) => {
                        simplelog::debug!(
                            "parent thread {:?} successfully joined thread {:?} from worker {:?} with result {:?}",
                            this_id,
                            thread_id,
                            worker.id,
                            val
                        )
                    }
                }
            }
        }
    }
}
