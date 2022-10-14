use std::{
    fmt,
    error::Error,
    thread,
    sync::{mpsc, Arc, Mutex},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                },

                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { 
            id,
            thread: Some(thread), 
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(fmt::Debug)]
pub struct PoolCreationError {}

impl Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to create a thread pool with given size")
    }
}

impl ThreadPool {
    
    /// Create a new Threadpool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool::build(size).unwrap()
    }

    /// Creates a new Threadpool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// This function doesn't panic like ThreadPool::new. 
    /// Instead it returns a Result
    /// 
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError {});
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
 
        Ok(ThreadPool {workers, sender: Some(sender)})
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
            println!("Shutting down worker {}", worker.id);
        }
    }
}
