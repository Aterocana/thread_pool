use std::{error::Error, thread};

// Box<dyn FnOnce() + Send + 'static> is a trait object;
// FnOnce trait is a closure which will be executed once;
// Send means the value can be sent across multiple trait;
// 'static bound is the object lifetime.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    f: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id: id,
            f: thread::spawn(move || loop {
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
