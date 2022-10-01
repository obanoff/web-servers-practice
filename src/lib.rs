pub mod multi_threaded {

    use std::{
        thread, 
        thread::JoinHandle, 
        sync::{mpsc, Arc, Mutex},
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
        time::Duration,
        process,
    };
    

    pub fn run(threads: usize) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
            println!("Cannot set connection on this address: {err}");
            process::exit(1);
        });
    
        let pool = ThreadPool::build(threads).unwrap_or_else(|err| {
            println!("Problem spawning threads: {err}");
            process::exit(1);
        });
    
        // for stream in listener.incoming().take(2) { // to see code in action, 2 request and gracefull shutting down
        for stream in listener.incoming() {
            let stream = stream.unwrap_or_else(|err| {
                println!("Problem encountered: {err}");
                process::exit(1);
            });
    
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    
        println!("Shutting down"); 
    
        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&mut stream);
            let request_line = buf_reader.lines().next().unwrap().unwrap();
        
            let (status_line, filename) = match &request_line[..] {
                "GET / HTTP/1.1" | "GET /index HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
                "GET /sleep HTTP/1.1" => {
                    thread::sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK", "index.html")
                },
                _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
            };
        
            let contents = fs::read_to_string(filename).unwrap();
            let length = contents.len();
        
            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    }

    struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    impl ThreadPool {
        fn build(size: usize) -> Result<Self, &'static str> {
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
    
        fn execute<F>(&self, f: F) 
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
}


pub mod async_sinle_threaded {
    // missing code
}














