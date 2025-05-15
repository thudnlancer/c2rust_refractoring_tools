use std::ffi::{CString, CStr};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct PthMutex {
    inner: Mutex<()>,
}

impl PthMutex {
    fn new() -> Self {
        PthMutex {
            inner: Mutex::new(()),
        }
    }

    fn acquire(&self) {
        let _guard = self.inner.lock().unwrap();
    }

    fn release(&self) {
        // Mutex is automatically released when guard goes out of scope
    }
}

fn my_reader(mutex: Arc<PthMutex>) {
    let mut buf = [0; 3];
    loop {
        let n = io::stdin().read(&mut buf[..1]).unwrap();
        if n == 0 {
            eprintln!("reader: EOF");
            break;
        }

        if buf[0] == b'\n' {
            buf[0] = b'\\';
            buf[1] = b'n';
            eprintln!("reader: bytes=2, char='\\n'");
        } else {
            eprintln!("reader: bytes=1, char='{}'", buf[0] as char);
        }

        match buf[0] as char {
            'Q' | 'q' => break,
            'L' | 'l' => {
                eprintln!("reader: ACQUIRE MUTEX");
                mutex.acquire();
            }
            'U' | 'u' => {
                eprintln!("reader: RELEASE MUTEX");
                mutex.release();
            }
            _ => {}
        }
        io::stderr().flush().unwrap();
    }
}

fn my_child(name: &'static str, mutex: Arc<PthMutex>) {
    for i in 0..10 {
        mutex.acquire();
        eprintln!("{}: {}", name, i);
        mutex.release();
        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let mutex = Arc::new(PthMutex::new());
    
    eprintln!("This is TEST_MISC, a Pth test using various stuff.");
    eprintln!("\nA stdin reader child and various looping childs are");
    eprintln!("spawned. When you enter 'l' you can lock a mutex which");
    eprintln!("blocks the looping childs. 'u' unlocks this mutex.");
    eprintln!("Enter 'q' to quit.\n");

    let mut handles = vec![];

    // Spawn child threads
    let names = ["foo", "bar", "baz", "quux", "killer", "killer II"];
    for &name in &names {
        let m = mutex.clone();
        handles.push(thread::spawn(move || my_child(name, m)));
    }

    // Spawn reader thread
    let m = mutex.clone();
    handles.push(thread::spawn(move || my_reader(m)));

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    eprintln!("Main Exit");
}