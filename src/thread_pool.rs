use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender:  mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` with the given number of threads.
    /// # Panics
    /// The `new` function will panic if the given size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = {
            let (tx, rx) = mpsc::channel();
            (tx, Arc::new(Mutex::new(rx)))
        };

        let mut threads = Vec::with_capacity(size);
        for id in 0 .. size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id:     usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        });

        Self { id, thread }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F> FnBox for F
where
    F: FnOnce(),
{
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}
