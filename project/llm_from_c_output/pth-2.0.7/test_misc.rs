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
**  test_misc.c: Pth test program (misc functions)
*/
                             /* ``Study it forever and you'll still wonder.
                                  Fly it once and you'll know.''
                                                 -- Henry Spencer  */

use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static mut MUTEX: Option<Arc<Mutex<()>>> = None;

fn my_reader() {
    let mut buf = [0u8; 3];
    
    loop {
        let n = match io::stdin().read(&mut buf[..1]) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("reader: error");
                break;
            }
        };
        
        if n == 0 {
            eprintln!("reader: EOF");
            break;
        }
        
        if n == 1 && buf[0] == b'\n' {
            buf[0] = b'\\';
            buf[1] = b'n';
            eprintln!("reader: bytes=2, char='\\n'");
        } else {
            buf[n] = 0;
            let s = String::from_utf8_lossy(&buf[..n]);
            eprintln!("reader: bytes={}, char='{}'", n, s);
        }
        
        if buf[0] == b'Q' || buf[0] == b'q' {
            break;
        }
        
        unsafe {
            if let Some(mutex) = &MUTEX {
                if buf[0] == b'L' || buf[0] == b'l' {
                    eprintln!("reader: ACQUIRE MUTEX");
                    let _guard = mutex.lock().unwrap();
                    // Hold the lock until next iteration
                    thread::sleep(Duration::from_secs(1));
                }
                if buf[0] == b'U' || buf[0] == b'u' {
                    eprintln!("reader: RELEASE MUTEX");
                    // Lock will be automatically released when guard drops
                }
            }
        }
        
        io::stderr().flush().unwrap();
    }
}

fn my_child(name: &'static str) {
    for i in 0..10 {
        unsafe {
            if let Some(mutex) = &MUTEX {
                let _guard = mutex.lock().unwrap();
                eprintln!("{}: {}", name, i);
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    unsafe {
        MUTEX = Some(Arc::new(Mutex::new(())));
    }
    
    eprintln!("This is TEST_MISC, a Pth test using various stuff.");
    eprintln!("\n");
    eprintln!("A stdin reader child and various looping childs are");
    eprintln!("spawned. When you enter 'l' you can lock a mutex which");
    eprintln!("blocks the looping childs. 'u' unlocks this mutex.");
    eprintln!("Enter 'q' to quit.");
    eprintln!("\n");

    eprintln!("Main Startup");

    let mut handles = vec![];

    // Spawn child threads
    let names = ["foo", "bar", "baz", "quux", "killer", "killer II"];
    for name in names.iter() {
        let name = *name;
        let handle = thread::spawn(move || {
            my_child(name);
        });
        handles.push(handle);
    }

    // Spawn reader thread
    let reader_handle = thread::spawn(my_reader);
    handles.push(reader_handle);

    // Main loop
    loop {
        let active_threads = handles.iter().filter(|h| !h.is_finished()).count();
        if active_threads <= 1 {
            break;
        }
        eprintln!("Main Loop ({} total threads still running)", active_threads);
        thread::sleep(Duration::from_millis(500));
    }

    eprintln!("Main Exit");

    // Wait for remaining threads
    for handle in handles {
        handle.join().unwrap();
    }
}