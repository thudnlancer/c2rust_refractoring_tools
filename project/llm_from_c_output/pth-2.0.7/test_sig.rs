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
**  test_sig.c: Pth test program (signal handling)
*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use signal_hook::{iterator::Signals, SIGINT, SIGUSR1, SIGUSR2};
use crossbeam_channel::{Sender, Receiver, unbounded};

static CHILD1: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
static CHILD2: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);

fn inthandler() {
    eprintln!("inthandler: enter");

    let mut signals = Signals::new(&[SIGINT]).unwrap();
    
    for n in 0..3 {
        if let Some(sig) = signals.forever().next() {
            if sig == SIGINT {
                eprintln!("inthandler: SIGINT received (#{})", n);
            }
        }
    }

    eprintln!("inthandler: cancelling child1 and child2");
    if let Some(child1) = CHILD1.lock().unwrap().take() {
        child1.join().unwrap();
    }
    if let Some(child2) = CHILD2.lock().unwrap().take() {
        child2.join().unwrap();
    }

    eprintln!("inthandler: leave");
}

fn child_cleanup(name: &str) {
    eprintln!("{}: running cleanup", name);
}

fn child(name: String) {
    eprintln!("{}: enter", name);

    // Establish cleanup handler
    let _cleanup = scopeguard::guard((), |_| child_cleanup(&name));

    let mut blocked_signals = Signals::empty().unwrap();
    blocked_signals.add_signal(SIGINT).unwrap();
    
    if name == "child1" {
        blocked_signals.add_signal(SIGUSR1).unwrap();
        blocked_signals.remove_signal(SIGUSR2).unwrap();
    } else {
        blocked_signals.remove_signal(SIGUSR1).unwrap();
        blocked_signals.add_signal(SIGUSR2).unwrap();
    }

    for i in 0..10 {
        let usr1_blocked = blocked_signals.contains(SIGUSR1);
        let usr2_blocked = blocked_signals.contains(SIGUSR2);
        
        eprintln!("{}: SIGUSR1: {}", name, if usr1_blocked { "blocked" } else { "unblocked" });
        eprintln!("{}: SIGUSR2: {}", name, if usr2_blocked { "blocked" } else { "unblocked" });
        eprintln!("{}: leave to scheduler", name);
        
        thread::sleep(Duration::from_secs(1));
        
        eprintln!("{}: reentered from scheduler", name);
    }

    eprintln!("{}: leave", name);
}

fn main() {
    eprintln!("This is TEST_SIG, a Pth test using signals.");
    eprintln!();
    eprintln!("Hit CTRL-C three times to stop this test.");
    eprintln!("But only after all threads were terminated.");
    eprintln!();

    eprintln!("main: init");

    // Block signals
    let mut blocked_signals = Signals::empty().unwrap();
    blocked_signals.add_signal(SIGUSR1).unwrap();
    blocked_signals.add_signal(SIGUSR2).unwrap();
    blocked_signals.add_signal(SIGINT).unwrap();

    // Spawn children
    let child1_name = "child1".to_string();
    let child2_name = "child2".to_string();
    
    let child1_handle = thread::Builder::new()
        .name(child1_name.clone())
        .spawn(move || child(child1_name))
        .unwrap();
    
    let child2_handle = thread::Builder::new()
        .name(child2_name.clone())
        .spawn(move || child(child2_name))
        .unwrap();
    
    let inthandler_handle = thread::Builder::new()
        .name("inthandler".to_string())
        .spawn(inthandler)
        .unwrap();

    *CHILD1.lock().unwrap() = Some(child1_handle);
    *CHILD2.lock().unwrap() = Some(child2_handle);

    // Wait until children are finished
    inthandler_handle.join().unwrap();

    eprintln!("main: exit");
}