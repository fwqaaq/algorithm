use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read_exact(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    // let get = b"GET / HTTP/1.1\r\n";
    let contents =
        fs::read_to_string("/Users/feiwu/Project/node/algorithm/rustt/src/index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\nContent-Length: {}\nContent-Type: text/plain\n\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("{:?}", stream);
}

// pub struct ThreadPool {
//     threads: Vec<Worker>,
//     sender: mpsc::Sender<Job>,
// }

// // struct Job;

// impl ThreadPool {
//     pub fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);
//         let (sender, receiver) = mpsc::channel();
//         let mut workers = Vec::with_capacity(size);

//         let rec = Arc::new(Mutex::new(receiver));

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&rec)));
//         }

//         ThreadPool {
//             threads: workers,
//             sender,
//         }
//     }

//     pub fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);
//         self.sender.send(job).unwrap();
//     }
// }

// struct Worker {
//     id: usize,
//     thread: thread::JoinHandle<()>,
// }

// type Job = Box<dyn FnOnce() + Send + 'static>;

// impl Worker {
//     fn new(id: usize, rec: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let job = rec.lock().unwrap().recv().unwrap();
//             // println!("worker {:?} get a job; execting", job);
//             job();
//         });
//         Worker { id, thread }
//     }
// }
