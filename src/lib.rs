pub use errors::PoolCreationError;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};

mod errors;
mod worker;

pub struct ThreadPool {
    _workers: Vec<worker::Worker>,
    sender: mpsc::Sender<worker::Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::new(size));
        }
        let (sender, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut _workers = Vec::with_capacity(size);
        for i in 0..size {
            _workers.push(worker::Worker::new(i, Arc::clone(&rx)));
        }
        Ok(ThreadPool { _workers, sender })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: (FnOnce() -> ()) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job)?;
        Ok(())
    }
}
