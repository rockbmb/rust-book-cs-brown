pub struct ThreadPool;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreadPoolCreationError
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroThreadPoolCreationError)
        }

        Ok(ThreadPool)
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