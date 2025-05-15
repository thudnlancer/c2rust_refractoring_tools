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
**  test_select.c: Pth test program (select)
*/
                             /* ``Most computer problems are located
                                between the keyboard and the chair.'' */

use std::io::{self, Read, Write};
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// A useless ticker thread
fn ticker(running: Arc<AtomicBool>) {
    eprintln!("ticker: start");
    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(5));
        let now = SystemTime::now();
        eprintln!("ticker was woken up on {:?}", now);
    }
}

fn main() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let ticker_running = running.clone();

    eprintln!("This is TEST_SELECT, a Pth test using select.");
    eprintln!();
    eprintln!("Enter data. Hit CTRL-C to stop this test.");
    eprintln!();

    // Spawn ticker thread
    let ticker_handle = thread::spawn(move || {
        ticker(ticker_running);
    });

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = [0u8; 1];

    loop {
        let mut rfds = [0; 1];
        let mut timeout = Duration::from_secs(10);

        // Simulate select with a timeout
        let start = SystemTime::now();
        let result = {
            let mut fds = mio::unix::EventedFd(&0);
            let poll = mio::Poll::new()?;
            poll.register(&fds, mio::Token(0), mio::Ready::readable(), mio::PollOpt::edge())?;
            let mut events = mio::Events::with_capacity(1);
            poll.poll(&mut events, Some(timeout))?;
            events.iter().count()
        };

        if result == 0 {
            eprintln!("main: timeout - repeating");
            continue;
        }

        match stdin.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => {
                eprintln!("main: read stdin '{}'", buf[0] as char);
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    running.store(false, Ordering::Relaxed);
    ticker_handle.join().unwrap();

    Ok(())
}