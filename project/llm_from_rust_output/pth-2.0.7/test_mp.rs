use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::{self, BufRead};

struct Query {
    string: String,
}

fn worker(rx: Receiver<Query>, tx: Sender<Query>) {
    println!("worker: start");
    
    while let Ok(mut query) = rx.recv() {
        println!("worker: recv query <{}>", query.string);
        
        query.string = query.string.to_uppercase();
        
        println!("worker: send reply <{}>", query.string);
        tx.send(query).unwrap();
    }
}

fn ticker() {
    println!("ticker: start");
    
    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("ticker was woken up on {}", now);
    }
}

fn main() -> io::Result<()> {
    println!("This is TEST_MP, a Pth test using message ports.");
    println!();
    println!("Lines on stdin are send to a worker thread via message");
    println!("ports, translated to upper case by the worker thread and");
    println!("send back to the main thread via message ports.");
    println!("Additionally a useless ticker thread awakens every 5s.");
    println!("Enter \"quit\" on stdin for stopping this test.");
    println!();

    let (worker_tx, worker_rx) = channel();
    let (main_tx, main_rx) = channel();
    
    let worker_handle = thread::spawn(move || {
        worker(worker_rx, main_tx);
    });

    let _ticker_handle = thread::spawn(ticker);

    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        let line = line?;
        
        if line == "quit" {
            println!("main: quit");
            break;
        }
        
        println!("main: out --> <{}>", line);
        
        let query = Query { string: line };
        worker_tx.send(query).unwrap();
        
        if let Ok(reply) = main_rx.recv() {
            println!("main: in <-- <{}>", reply.string);
        }
    }

    drop(worker_tx);
    worker_handle.join().unwrap();
    
    Ok(())
}