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
**  test_std.c: Pth standard test program
*/
                             /* ``Understanding a problem is knowing why
                                it is hard to solve it, and why the most
                                straightforward approaches won't work.''
                                                  -- Karl Popper        */

use std::io::{self, Write};
use std::process;
use std::thread;
use std::sync::{Arc, Mutex};
use std::error::Error;

// Mock Pth implementation for Rust
mod pth {
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub const PTH_ATTR_DEFAULT: () = ();
    pub const PTH_PRIO_MAX: i32 = 10;
    pub const FALSE: i32 = 0;

    pub type pth_attr_t = ();
    pub type pth_t = thread::JoinHandle<()>;

    pub fn pth_version() -> i32 {
        0x12345678
    }

    pub fn pth_init() -> i32 {
        1
    }

    pub fn pth_kill() {}

    pub fn pth_attr_new() -> pth_attr_t {
        ()
    }

    pub fn pth_attr_set(_attr: pth_attr_t, _key: i32, _value: impl Sized) -> i32 {
        1
    }

    pub fn pth_spawn(_attr: pth_attr_t, f: impl FnOnce() + Send + 'static, _arg: ()) -> Option<pth_t> {
        Some(thread::spawn(f))
    }

    pub fn pth_attr_destroy(_attr: pth_attr_t) {}

    pub fn pth_join(handle: pth_t, _rval: &mut Option<Box<dyn std::any::Any>>) -> i32 {
        handle.join().is_ok() as i32
    }

    pub fn pth_yield() {}
}

macro_rules! failed_if {
    ($expr:expr) => {
        if $expr {
            eprintln!("*** ERROR, TEST FAILED:\n*** errno={}\n\n", std::io::Error::last_os_error().raw_os_error().unwrap_or(0));
            process::exit(1);
        }
    };
}

fn t1_func(arg: i64) -> i64 {
    let mut val = arg;
    failed_if!(val != 123);
    
    for _ in 0..100 {
        val += 10;
        pth::pth_yield();
    }
    val
}

fn t2_func(arg: i64) -> i64 {
    let mut rval;
    if arg < 9 {
        let new_val = arg + 1;
        eprintln!("Spawning thread {}", new_val);
        
        let handle = pth::pth_spawn(pth::PTH_ATTR_DEFAULT, || {
            let res = t2_func(new_val);
            Box::new(res) as Box<dyn std::any::Any>
        }, ());
        
        failed_if!(handle.is_none());
        
        let mut join_result = None;
        let rc = pth::pth_join(handle.unwrap(), &mut join_result);
        eprintln!("Joined thread {}", new_val);
        failed_if!(rc == pth::FALSE);
        
        let child_rval = *join_result.unwrap().downcast::<i64>().unwrap();
        rval = arg * child_rval;
    } else {
        rval = arg;
    }
    rval
}

fn main() {
    eprintln!("\n=== TESTING GLOBAL LIBRARY API ===\n");
    {
        eprintln!("Fetching library version");
        let version = pth::pth_version();
        failed_if!(version == 0x0);
        eprintln!("version = 0x{:X}", version);
    }

    eprintln!("\n=== TESTING BASIC OPERATION ===\n");
    {
        eprintln!("Initializing Pth system (spawns scheduler and main thread)");
        let rc = pth::pth_init();
        failed_if!(rc == pth::FALSE);
        eprintln!("Killing Pth system for testing purposes");
        pth::pth_kill();
        eprintln!("Re-Initializing Pth system");
        let rc = pth::pth_init();
        failed_if!(rc == pth::FALSE);
    }

    eprintln!("\n=== TESTING BASIC THREAD OPERATION ===\n");
    {
        eprintln!("Creating attribute object");
        let attr = pth::pth_attr_new();
        failed_if!(attr == ());
        let rc = pth::pth_attr_set(attr, 0, "test1");
        failed_if!(rc == pth::FALSE);
        let rc = pth::pth_attr_set(attr, 1, pth::PTH_PRIO_MAX);
        failed_if!(rc == pth::FALSE);

        eprintln!("Spawning thread");
        let handle = pth::pth_spawn(attr, || {
            let res = t1_func(123);
            Box::new(res) as Box<dyn std::any::Any>
        }, ());
        failed_if!(handle.is_none());
        pth::pth_attr_destroy(attr);

        eprintln!("Joining thread");
        let mut val = None;
        let rc = pth::pth_join(handle.unwrap(), &mut val);
        failed_if!(rc == pth::FALSE);
        let result = *val.unwrap().downcast::<i64>().unwrap();
        failed_if!(result != 1123);
    }

    eprintln!("\n=== TESTING NESTED THREAD OPERATION ===\n");
    {
        eprintln!("Spawning thread 1");
        let handle = pth::pth_spawn(pth::PTH_ATTR_DEFAULT, || {
            let res = t2_func(1);
            Box::new(res) as Box<dyn std::any::Any>
        }, ());
        failed_if!(handle.is_none());

        let mut val = None;
        let rc = pth::pth_join(handle.unwrap(), &mut val);
        eprintln!("Joined thread 1");
        failed_if!(rc == pth::FALSE);
        let result = *val.unwrap().downcast::<i64>().unwrap();
        failed_if!(result != 1*2*3*4*5*6*7*8*9);
    }

    pth::pth_kill();
    eprintln!("\nOK - ALL TESTS SUCCESSFULLY PASSED.\n");
    process::exit(0);
}