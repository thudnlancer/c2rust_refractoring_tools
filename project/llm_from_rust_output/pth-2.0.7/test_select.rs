use std::ffi::CString;
use std::io::{self, Read, Write};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;

fn ticker() {
    eprintln!("ticker: start");
    loop {
        thread::sleep(Duration::from_secs(5));
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        eprintln!("ticker was woken up on {}", now);
    }
}

fn main() -> io::Result<()> {
    eprintln!("This is TEST_SELECT, a Pth test using select.");
    eprintln!();
    eprintln!("Enter data. Hit CTRL-C to stop this test.");
    eprintln!();

    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let ticker_handle = thread::spawn(move || {
        ticker();
        tx.send(()).unwrap();
    });

    let mut buffer = [0; 1];
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                eprintln!("main: read stdin '{}'", buffer[0] as char);
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                eprintln!("main: timeout - repeating");
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    ticker_handle.join().unwrap();
    Ok(())
}