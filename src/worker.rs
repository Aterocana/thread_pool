use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Box<dyn FnOnce() + Send + 'static> is a trait object;
// FnOnce trait is a closure which will be executed once;
// Send means the value can be sent across multiple trait;
// 'static bound is the object lifetime.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let mut thread = thread::spawn(move || loop {
            let msg = receiver
                .lock()
                .expect("error occurred while trying to acquire lock")
                .recv()
                .expect("error occurred while trying to receive from channel");
            match msg {
                Message::NewJob(job) => {
                    // println!("worker {} got a job: executing.", id);
                    job();
                }
                Message::Terminate => {
                    // println!("job {} terminating", id);
                    break;
                }
            }
        });
        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
