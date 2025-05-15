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
**  test_sfio.rs: Pth test program (Sfio support) - Rust translation
*/
                             /* ``No, I'm not going to explain it.
                                  If you can't figure it out, you
                                  didn't want to know anyway...''
                                            -- Larry Wall          */

use std::io::{self, BufRead, Write};
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::mpsc;

#[cfg(feature = "pth_sfio")]
mod pth_sfio {
    use super::*;

    fn worker() {
        let stdin = io::stdin();
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        
        for line in stdin.lock().lines() {
            match line {
                Ok(input) => {
                    writeln!(handle, "you entered '{}' on stdin", input).unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }

    fn ticker() {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        writeln!(handle, "ticker: start").unwrap();
        
        loop {
            thread::sleep(Duration::from_secs(5));
            let now = SystemTime::now();
            writeln!(handle, "ticker was woken up on {:?}", now).unwrap();
        }
    }

    pub fn main() -> io::Result<()> {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        
        writeln!(handle, "This is TEST_SFIO, a Pth test using Sfio disciplines.")?;
        writeln!(handle, "")?;
        writeln!(handle, "Stdout/Stdin are connected to Sfio streams with a Pth")?;
        writeln!(handle, "discipline on top of the streams in order to use Pth's")?;
        writeln!(handle, "I/O operations. It demonstrates that the process this")?;
        writeln!(handle, "way does not block. Instead only one thread blocks.")?;
        writeln!(handle, "")?;

        let worker_thread = thread::Builder::new()
            .name("worker".to_string())
            .spawn(worker)?;

        let ticker_thread = thread::Builder::new()
            .name("ticker".to_string())
            .spawn(ticker)?;

        worker_thread.join().unwrap();
        ticker_thread.join().unwrap();

        Ok(())
    }
}

#[cfg(not(feature = "pth_sfio"))]
fn main() {
    eprintln!("You have to build Pth with --with-sfio to run this test!");
}

#[cfg(feature = "pth_sfio")]
fn main() {
    if let Err(e) = pth_sfio::main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}