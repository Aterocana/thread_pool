pub use errors::PoolCreationError;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};

mod errors;
mod worker;

pub struct ThreadPool {
    workers: Vec<worker::Worker>,
    sender: mpsc::Sender<worker::Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::new(size));
        }
        let (sender, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(worker::Worker::new(i, Arc::clone(&rx)));
        }
        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: (FnOnce() -> ()) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(worker::Message::NewJob(job))?;
        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender
                .send(worker::Message::Terminate)
                .expect("error on sending");
        }

        for worker in &mut self.workers {
            // the take method on Option takes the Some variant out
            // and leaves None in its place
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}
