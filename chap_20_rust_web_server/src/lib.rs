use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct Worker {
    id : usize,
    handle : Option<thread::JoinHandle<()>>,
}

#[derive(Debug)]
pub enum WorkerError {
    ThreadCreationError(io::Error)
}

fn worker_func(worker_id : usize, job_receiver : Arc<Mutex<mpsc::Receiver<Job>>>) {
    
}

impl Worker {
    /// Create a new worker to handle requests from the server.
    ///
    /// Fails with `WorkerError` if it's not possible to start the worker's thread
    /// due to OS issues.
    ///
    /// # Panics
    ///
    /// This function never panics.
    fn build(id: usize, job_receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, WorkerError> {
        let builder = thread::Builder::new().name(format!("Worker-{}", id));

        let thread_res = builder
            .spawn(move || loop {
                //let receiver_guard = job_receiver.lock();
/*                 let receiver = match receiver_guard {
                    Err(err) => {
                        eprintln!(
                            "Worker thread {} failed to acquire lock on job queue. Error: {:?}",
                            worker_id,
                            err
                        );
                        continue;
                    },
                    Ok(r) => r,
                }; */
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
                        job();
                    }
                }
            });

        let res = match thread_res {
            Ok(thread_handle) => {
                let thread_handle = Some(thread_handle);
                Ok (Worker { id, handle: thread_handle })
            },
            Err(err) => Err(WorkerError::ThreadCreationError(err)),
        };

        res
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub enum ThreadPoolError {
    ZeroThreadThreadPoolError,
    WorkerError(WorkerError),
    InexistentJobSenderError,
    JobTransmissionError(mpsc::SendError<Job>)
}

pub struct ThreadPool {
    workers : Vec<Worker>,
    job_sender : Option<mpsc::Sender<Job>>,
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
    pub fn build(size : usize) -> Result<ThreadPool, ThreadPoolError> {
        if size == 0 {
            return Err(ThreadPoolError::ZeroThreadThreadPoolError)
        }

        let (job_sender, job_receiver) = mpsc::channel();

        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut workers = Vec::with_capacity(size);

        for n in 0..size {
            let worker_res = Worker::build(n, Arc::clone(&job_receiver));
            match worker_res {
                Ok(worker) => workers.push(worker),
                Err(w_err) => return Err(ThreadPoolError::WorkerError(w_err))
            }
        }

        Ok(ThreadPool { workers, job_sender: Some(job_sender) })
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
    pub fn execute<F>(&self, f : F) -> Result<(), ThreadPoolError>
    where
        // We still use the () after FnOnce because this FnOnce represents a
        // closure that takes no parameters and returns the unit type ()
        F : FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        let sender = match self.job_sender.as_ref() {
            None => return Err(ThreadPoolError::InexistentJobSenderError),
            Some(s) => s
        };
        sender.send(job).map_err(ThreadPoolError::JobTransmissionError)
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        std::mem::drop(self.job_sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}