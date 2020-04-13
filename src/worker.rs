use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Box<dyn FnOnce() + Send + 'static> is a trait object;
// FnOnce trait is a closure which will be executed once;
// Send means the value can be sent across multiple trait;
// 'static bound is the object lifetime.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

// A Message a worker can receive is either a new job to execute,
// or a Terminate instruction.
pub enum Message {
    NewJob(Job),
    Terminate,
}

// A Worker has its own ID, which can be used for debug purposes.
// A Worker is basically a thread with an endless loop,
// in which it waits for Message through a mpsc channel receiver.
// When a Terminate Message arrives, the Worker exits its loop, dying.
pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // To create a new Worker a receiver is passed as an atomic reference to a channel
    // shared between ThreadPool workers, guarded by a Mutex.
    // This way a single worker obtain the access to critical section, therefore only a
    // worker consumes a message and processes it.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
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
