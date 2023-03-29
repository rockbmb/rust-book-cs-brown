use std::{
    io,
    sync::{mpsc, Arc, Mutex, PoisonError},
    thread
};

pub struct Worker {
    worker_id : usize,
    thread_handle : thread::JoinHandle<()>,
}

#[derive(Debug)]
pub enum WorkerCreationError {
    JobQueueReadError,
    ThreadCreationError(io::Error)
}

impl Worker {
    fn build(worker_id: usize, job_receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, WorkerCreationError> {
        let builder = thread::Builder::new().name(format!("Worker-{}", worker_id));

        let thread_res = builder.spawn(move || loop {
            let job = job_receiver.lock().unwrap().recv().unwrap();
            println!("Worker {worker_id} got a job; executing");
            job()
        });

        match thread_res {
            Ok(thread_handle) => Ok (Worker { worker_id, thread_handle }),
            Err(err) => Err(WorkerCreationError::ThreadCreationError(err)),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreadPoolCreationError,
    WorkerError(WorkerCreationError)
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    job_sender : mpsc::Sender<Job>,
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
    pub fn build(size : usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroThreadPoolCreationError)
        }

        let (job_sender, job_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let mut workers = Vec::with_capacity(size);

        for n in 0..size {
            let worker_res = Worker::build(n, Arc::clone(&job_receiver));
            match worker_res {
                Ok(worker) => workers.push(worker),
                Err(worker_err) => return Err(PoolCreationError::WorkerError(worker_err))
            }
        }

        Ok(ThreadPool { workers, job_sender })
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
    pub fn execute<F>(&self, f : F) -> Result<(), mpsc::SendError<Job>>
    where
        // We still use the () after FnOnce because this FnOnce represents a
        // closure that takes no parameters and returns the unit type ()
        F : FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.job_sender.send(job)
    }
}