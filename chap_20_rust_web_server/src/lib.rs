use std::{sync::mpsc, thread};

pub struct Worker {
    worker_id : usize,
    thread_handle : thread::JoinHandle<()>
}

impl Worker {
    pub fn new(worker_id: usize) -> Worker {
        // TODO
        // Note: If the operating system can’t create a thread because there
        // aren’t enough system resources, thread::spawn will panic. That will
        // cause our whole server to panic, even though the creation of some
        // threads might succeed.
        //
        // For simplicity’s sake, this behavior is fine, but in a production
        // thread pool implementation, you’d likely want to use
        // std::thread::Builder and its spawn method that returns Result
        // instead.
        let thread_handle = thread::spawn(|| {});
        Worker { worker_id, thread_handle}
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreadPoolCreationError
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn creation_helper(size : usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        for n in 0..size {
            workers.push(Worker::new(n));
        }

        ThreadPool { workers }
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        Self::creation_helper(size)
    }

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
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroThreadPoolCreationError)
        }

        Ok(Self::creation_helper(size))
    }

    /*
    signature for `std::thread::spawn`, to serve as a possible starting point for `execute`.

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    */
    // We can be further confident that FnOnce is the trait we want to use
    // because the thread for running a request will only execute that request’s
    // closure one time, which matches the Once in FnOnce.
    pub fn execute<F>(&self, f : F)
    where
        // We still use the () after FnOnce because this FnOnce represents a
        // closure that takes no parameters and returns the unit type ()
        F : FnOnce() + Send + 'static
    {

    }
}