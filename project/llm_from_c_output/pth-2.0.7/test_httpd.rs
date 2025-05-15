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
**  test_httpd.rs: Pth test program (faked HTTP daemon) - Rust translation
*/
                             /* ``Unix is simple. It just takes a
                                  genius to understand its simplicity.''
                                            --- Dennis Ritchie           */

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr};
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process;
use std::str;
use std::fmt::Write as FmtWrite;
use signal_hook::consts::{SIGINT, SIGTERM, SIGPIPE};
use signal_hook::flag;

const MAXREQLINE: usize = 1024;
const REQ_MAX: usize = if cfg!(windows) { 100 } else { 1024 - 100 };

/*
 * The HTTP request handler
 */

fn handler(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; MAXREQLINE];
    
    // Read request
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 || (n == 1 && buf[0] == b'\n') {
            break;
        }
    }

    // Simulate processing
    thread::yield_now();

    // Generate response
    let response = format!(
        "HTTP/1.0 200 Ok\r\n\
         Server: test_httpd/1.0\r\n\
         Connection: close\r\n\
         Content-type: text/plain\r\n\
         \r\n\
         Just a trivial test for GNU Pth\n\
         to show that it's serving data.\r\n"
    );

    stream.write_all(response.as_bytes())?;
    eprintln!("connection shutdown (fd: {:?})", stream.peer_addr()?);
    Ok(())
}

/*
 * A useless ticker we let run just for fun in parallel
 */

fn ticker(running: Arc<AtomicBool>) -> io::Result<()> {
    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(5));
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let ct = format!("{}", now); // Simplified timestamp
        
        // In Rust we don't have direct access to scheduler load
        let avload = 0.0f32; // Placeholder
        
        eprintln!("ticker woken up on {}, average load: {:.2}", ct, avload);
    }
    Ok(())
}

/*
 * And the server main procedure
 */

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        process::exit(1);
    }

    let port = args[1].parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Illegal port: {}", args[1]);
        process::exit(1);
    });

    eprintln!("This is TEST_HTTPD, a Pth test using socket I/O.\n");
    eprintln!("\n");
    eprintln!("Multiple connections are accepted on the specified port.\n");
    eprintln!("For each connection a separate thread is spawned which\n");
    eprintln!("reads a HTTP request the socket and writes back a constant\n");
    eprintln!("(and useless) HTTP response to the socket.\n");
    eprintln!("Additionally a useless ticker thread awakens every 5s.\n");
    eprintln!("Watch the average scheduler load the ticker displays.\n");
    eprintln!("Hit CTRL-C for stopping this test.\n");
    eprintln!("\n");

    // Setup signal handling
    let running = Arc::new(AtomicBool::new(true));
    flag::register(SIGINT, Arc::clone(&running))?;
    flag::register(SIGTERM, Arc::clone(&running))?;
    flag::register(SIGPIPE, Arc::clone(&running))?;

    // Run ticker thread
    let ticker_handle = {
        let running = Arc::clone(&running);
        thread::Builder::new()
            .name("ticker".to_string())
            .spawn(move || ticker(running))?
    };

    // Create TCP listener
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))?;
    eprintln!("listening on port {} (max {} simultaneous connections)", port, REQ_MAX);

    // Main accept loop
    for stream in listener.incoming().take_while(|_| running.load(Ordering::Relaxed)) {
        match stream {
            Ok(stream) => {
                if thread::available_parallelism()?.get() >= REQ_MAX {
                    eprintln!("currently no more connections acceptable");
                    continue;
                }

                if let Ok(peer_addr) = stream.peer_addr() {
                    eprintln!(
                        "connection established (ip: {}, port: {})",
                        peer_addr.ip(),
                        peer_addr.port()
                    );
                }

                thread::Builder::new()
                    .name("handler".to_string())
                    .spawn(move || handler(stream))?;
            }
            Err(e) => {
                eprintln!("accept error: {}", e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    // Cleanup
    ticker_handle.join().unwrap()?;
    Ok(())
}