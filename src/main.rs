use std:: {
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use std::process;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("Cannot set connection on this address: {err}");
        process::exit(1);
    });

    let pool = ThreadPool::build(4).unwrap_or_else(|err| {
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
}

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



































