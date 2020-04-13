pub use errors::PoolCreationError;
use std::sync::{mpsc, Arc, Mutex};
use std::{error::Error, thread};

mod errors;

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
            _workers.push(Worker::new(i, Arc::clone(&rx)));
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

// Box<dyn FnOnce() + Send + 'static> is a trait object;
// FnOnce trait is a closure which will be executed once;
// Send means the value can be sent across multiple trait;
// 'static bound is the object lifetime.
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    _f: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            _id: id,
            _f: thread::spawn(move || loop {
                let job = receiver
                    .lock()
                    .expect("error occurred while trying to acquire lock")
                    .recv()
                    .expect("error occurred while trying to receive from channel");

                println!("worker {} got a job: executing.", id);
                job();
            }),
        }
    }
}
