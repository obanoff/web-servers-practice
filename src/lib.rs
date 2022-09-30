use std::{
    thread, 
    thread::JoinHandle, 
    sync::{mpsc, Arc, Mutex},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.send(job).unwrap();
    }

    //🔴optional my own implementation instead of new, to handle errors more efficent:
    pub fn build(size: usize) -> Result<Self, &'static str> {
        match size > 0 {
            false => Err("number of threads cannot be zero"),
            true => {
                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                
                for id in 0..size {
                    let worker = Worker::new(id, Arc::clone(&receiver));

                    workers.push(worker);
                }

                Ok(ThreadPool { workers, sender })
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job: Job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}








































