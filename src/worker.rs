use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Box<dyn FnOnce() + Send + 'static> is a trait object;
// FnOnce trait is a closure which will be executed once;
// Send means the value can be sent across multiple trait;
// 'static bound is the object lifetime.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    _id: usize,
    _f: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
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
