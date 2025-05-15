use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use nix::sys::termios;
use nix::sys::signal::{self, Signal};
use nix::unistd;

struct Terminal {
    orig_termios: termios::Termios,
    tty: Option<File>,
    need_reset: bool,
    mode: i32,
}

impl Terminal {
    fn new() -> io::Result<Self> {
        let tty = File::open("/dev/tty").or_else(|_| {
            if unistd::isatty(0)? {
                unsafe { Ok(File::from_raw_fd(0)) }
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "not a tty"))
            }
        })?;

        let orig_termios = termios::tcgetattr(tty.as_raw_fd())?;
        Ok(Self {
            orig_termios,
            tty: Some(tty),
            need_reset: false,
            mode: -1,
        })
    }

    fn set_raw_mode(&mut self, mode: i32) -> io::Result<()> {
        if mode == self.mode || mode == -1 {
            return Ok(());
        }

        if !self.need_reset {
            self.need_reset = true;
            // Setup atexit handler would need to be implemented differently in Rust
        }

        let mut raw = termios::tcgetattr(self.tty.as_ref().unwrap().as_raw_fd())?;
        
        if mode != 0 {
            raw.local_flags.remove(termios::LocalFlags::ECHO);
            raw.control_chars[termios::SpecialCharacterIndices::VMIN as usize] = 1;
            raw.control_chars[termios::SpecialCharacterIndices::VTIME as usize] = 0;
        } else {
            raw.local_flags.insert(termios::LocalFlags::ECHO);
        }

        termios::tcsetattr(self.tty.as_ref().unwrap().as_raw_fd(), termios::SetArg::TCSANOW, &raw)?;
        self.mode = mode;
        termios::tcflush(self.tty.as_ref().unwrap().as_raw_fd(), termios::FlushArg::TCIFLUSH)?;
        Ok(())
    }

    fn cleanup(&mut self) -> io::Result<()> {
        if self.need_reset {
            termios::tcsetattr(
                self.tty.as_ref().unwrap().as_raw_fd(),
                termios::SetArg::TCSANOW,
                &self.orig_termios,
            )?;
            self.need_reset = false;
        }
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn ask_confirmation(prompt: &str) -> io::Result<bool> {
    let mut term = Terminal::new()?;
    let mut input = String::new();
    
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        term.set_raw_mode(1)?;
        let mut buf = [0u8; 1];
        term.tty.as_ref().unwrap().read_exact(&mut buf)?;
        term.set_raw_mode(0)?;
        
        println!(); // Newline after raw input
        
        match buf[0] as char {
            'y' | 'Y' => return Ok(true),
            'n' | 'N' => return Ok(false),
            _ => continue,
        }
    }
}

fn main() {
    // Example usage
    match ask_confirmation("Continue? [y/n] ") {
        Ok(true) => println!("Proceeding..."),
        Ok(false) => println!("Aborting..."),
        Err(e) => eprintln!("Error: {}", e),
    }
}