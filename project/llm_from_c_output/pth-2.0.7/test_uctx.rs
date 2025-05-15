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
**  test_uctx.c: Pth test program (user-space context switching)
*/
                             /* ``Engineering does not require science.
                                Science helps a lot, but people built
                                perfectly good brick walls long before
                                they knew why cement works.''
                                                        -- Alan Cox */

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;
use std::io::{self, Write};

// Assuming pth-rs provides these types and functions
use pth::{Uctx, uctx_create, uctx_make, uctx_switch, uctx_destroy};

static WORKER_DONE: [AtomicBool; 10] = [
    AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false),
    AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false),
    AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false),
    AtomicBool::new(false),
];

/*
 *  Test 1: master and worker "threads"
 */

fn worker(ctx: usize) {
    let n = ctx;
    eprintln!("worker #{}: enter", n);
    
    for i in 0..100 {
        eprintln!("worker #{}: working (step {})", n, i);
        unsafe {
            uctx_switch(uctx[n], uctx[0]);
        }
    }
    
    WORKER_DONE[n].store(true, Ordering::SeqCst);
    eprintln!("worker #{}: exit", n);
}

fn test_working() {
    eprintln!("master: startup");

    eprintln!("master: create contexts");
    let mut uctx: [Uctx; 10] = [Uctx::default(); 10];
    
    unsafe {
        uctx_create(&mut uctx[0]);
    }
    WORKER_DONE[0].store(false, Ordering::SeqCst);
    
    for i in 1..10 {
        WORKER_DONE[i].store(false, Ordering::SeqCst);
        unsafe {
            uctx_create(&mut uctx[i]);
            uctx_make(&mut uctx[i], None, 32*1024, None, worker, i, &uctx[0]);
        }
    }

    loop {
        let mut todo = false;
        for i in 1..10 {
            if !WORKER_DONE[i].load(Ordering::SeqCst) {
                eprintln!("master: switching to worker #{}", i);
                unsafe {
                    uctx_switch(uctx[0], uctx[i]);
                }
                eprintln!("master: came back from worker #{}", i);
                todo = true;
            }
        }
        if !todo {
            break;
        }
    }

    eprintln!("master: destroy contexts");
    for i in 1..10 {
        unsafe {
            uctx_destroy(uctx[i]);
        }
    }
    unsafe {
        uctx_destroy(uctx[0]);
    }

    eprintln!("master: exit");
}

/*
 *  Test 2: raw switching performance
 */

const DO_SWITCHES: i32 = 10_000_000;

static STAT_SWITCHED: AtomicI32 = AtomicI32::new(0);

fn dummy(_ctx: usize) {
    loop {
        STAT_SWITCHED.fetch_add(1, Ordering::SeqCst);
        unsafe {
            uctx_switch(uctx[1], uctx[0]);
        }
    }
}

fn test_performance() {
    let mut uctx: [Uctx; 2] = [Uctx::default(); 2];
    
    unsafe {
        uctx_create(&mut uctx[0]);
        uctx_create(&mut uctx[1]);
        uctx_make(&mut uctx[1], None, 32*1024, None, dummy, 0, &uctx[0]);
    }

    eprintln!("\nPerforming {} user-space context switches... be patient!", DO_SWITCHES);

    let stat_start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    STAT_SWITCHED.store(0, Ordering::SeqCst);
    
    for _ in 0..DO_SWITCHES {
        STAT_SWITCHED.fetch_add(1, Ordering::SeqCst);
        unsafe {
            uctx_switch(uctx[0], uctx[1]);
        }
    }
    
    let stat_end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    unsafe {
        uctx_destroy(uctx[0]);
        uctx_destroy(uctx[1]);
    }

    let duration = stat_end - stat_start;
    eprintln!(
        "We required {} seconds for performing the test, so this means we can",
        duration
    );
    eprintln!(
        "perform {} user-space context switches per second on this platform.\n",
        DO_SWITCHES / duration as i32
    );
}

fn main() {
    test_working();
    test_performance();
}