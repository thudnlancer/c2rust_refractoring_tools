/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  test_mp.rs: Pth test program (message port handling) - Rust translation
*/
                             /* ``Failure is not an option.
                                It comes bundled with software.'' */
use std::io::{self, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Simplified message port simulation
struct MessagePort<T> {
    name: String,
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>,
}

impl<T> MessagePort<T> {
    fn new(name: &str) -> Self {
        let (sender, receiver) = mpsc::channel();
        MessagePort {
            name: name.to_string(),
            sender,
            receiver,
        }
    }

    fn send(&self, msg: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(msg)
    }

    fn recv(&self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }

    fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

// Global registry for message ports
lazy_static::lazy_static! {
    static ref MSG_PORTS: Mutex<HashMap<String, Arc<MessagePort<Query>>>> = Mutex::new(HashMap::new());
}

// Query structure
struct Query {
    string: String,
    reply_port: Arc<MessagePort<Query>>,
}

fn worker() {
    println!("worker: start");
    
    let port = Arc::new(MessagePort::new("worker"));
    {
        let mut ports = MSG_PORTS.lock().unwrap();
        ports.insert("worker".to_string(), port.clone());
    }

    loop {
        match port.recv() {
            Ok(query) => {
                println!("worker: recv query <{}>", query.string);
                let upper = query.string.to_uppercase();
                println!("worker: send reply <{}>", upper);
                let reply = Query {
                    string: upper,
                    reply_port: query.reply_port.clone(),
                };
                query.reply_port.send(reply).unwrap();
            }
            Err(_) => {
                thread::yield_now();
                continue;
            }
        }
    }
}

fn ticker() {
    println!("ticker: start");
    loop {
        thread::sleep(Duration::from_secs(5));
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("ticker was woken up on {}", now);
    }
}

fn main() {
    println!("This is TEST_MP, a Pth test using message ports.\n");
    println!("Lines on stdin are send to a worker thread via message");
    println!("ports, translated to upper case by the worker thread and");
    println!("send back to the main thread via message ports.");
    println!("Additionally a useless ticker thread awakens every 5s.");
    println!("Enter \"quit\" on stdin for stopping this test.\n");

    // Spawn worker thread
    let worker_handle = thread::Builder::new()
        .name("worker".to_string())
        .spawn(worker)
        .unwrap();

    // Spawn ticker thread
    let ticker_handle = thread::Builder::new()
        .name("ticker".to_string())
        .spawn(ticker)
        .unwrap();

    // Get worker port
    let worker_port = {
        let ports = MSG_PORTS.lock().unwrap();
        ports.get("worker").unwrap().clone()
    };

    // Create main port
    let main_port = Arc::new(MessagePort::new("main"));
    {
        let mut ports = MSG_PORTS.lock().unwrap();
        ports.insert("main".to_string(), main_port.clone());
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "quit" {
            println!("main: quit");
            break;
        }

        println!("main: out --> <{}>", line);
        
        let query = Query {
            string: line,
            reply_port: main_port.clone(),
        };
        
        worker_port.send(query).unwrap();
        
        match main_port.recv() {
            Ok(reply) => println!("main: in <-- <{}>", reply.string),
            Err(e) => {
                println!("main: error receiving reply: {:?}", e);
                break;
            }
        }
    }

    // Cleanup
    worker_handle.join().unwrap();
    ticker_handle.join().unwrap();
}