use std::{
    thread, 
    thread::JoinHandle, 
    sync::{mpsc, Arc, Mutex},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    //🔴optional my own implementation instead of new, to handle errors more efficent:
    pub fn build(size: usize) -> Result<Self, &'static str> {
        match size {
            n if n == 0 => Err("number of threads cannot be zero"),
            n => {
                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                
                for id in 0..n {
                    let worker = Worker::new(id, Arc::clone(&receiver));

                    workers.push(worker);
                }

                Ok(ThreadPool { workers, sender: Some(sender) })
            }
        }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap(); 
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() { 
                thread.join().unwrap(); 
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>, 
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {  
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                },
            }
        });

        Worker { id, thread: Some(thread) } 
    }
}



















