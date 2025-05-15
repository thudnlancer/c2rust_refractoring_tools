use std::env;
use std::ffi::{CString, CStr};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let peer_addr = stream.peer_addr().unwrap();
    
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            if n == 1 && buffer[0] == b'\n' {
                return;
            }
            
            let response = "HTTP/1.0 200 Ok\r\n\
                            Server: test_httpd/0x200207\r\n\
                            Connection: close\r\n\
                            Content-type: text/plain\r\n\r\n\
                            Just a trivial test for Rust\nto show that it's serving data.\r\n";
            
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Write error: {}", e);
            }
        }
        Ok(_) => {}
        Err(e) => {
            eprintln!("Read error: {}", e);
        }
    }
    
    println!("Connection shutdown from: {}", peer_addr);
}

fn ticker() {
    while RUNNING.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(5));
        println!("Ticker woken up");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        process::exit(1);
    }

    let port: u16 = match args[1].parse() {
        Ok(p) if p > 0 && p < 65535 => p,
        _ => {
            eprintln!("Illegal port: {}", args[1]);
            process::exit(1);
        }
    };

    println!("This is TEST_HTTPD, a Rust test using socket I/O.\n");
    println!("Multiple connections are accepted on the specified port.");
    println!("For each connection a separate thread is spawned which");
    println!("reads a HTTP request the socket and writes back a constant");
    println!("(and useless) HTTP response to the socket.");
    println!("Additionally a useless ticker thread awakens every 5s.");
    println!("Hit CTRL-C for stopping this test.\n");

    let running = Arc::new(RUNNING);
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");

    thread::spawn(ticker);

    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))
        .expect("Failed to bind to port");

    println!("Listening on port {}...", port);

    for stream in listener.incoming() {
        if !RUNNING.load(Ordering::Relaxed) {
            break;
        }
        
        match stream {
            Ok(stream) => {
                let peer = stream.peer_addr().unwrap();
                println!("Connection established from: {}", peer);
                
                thread::spawn(move || {
                    handler(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    println!("Server shutting down");
}