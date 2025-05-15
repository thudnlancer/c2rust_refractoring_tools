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
**  test_philo.rs: Rust version of Pth test application showing the 5 philosopher problem
*/

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::io::{self, Write};

const PHILNUM: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PhilStat {
    Thinking,
    Hungry,
    Eating,
}

impl PhilStat {
    fn as_str(&self) -> &'static str {
        match self {
            PhilStat::Thinking => "thinking",
            PhilStat::Hungry => "hungry  ",
            PhilStat::Eating => "EATING  ",
        }
    }
}

struct Table {
    status: [PhilStat; PHILNUM],
}

impl Table {
    fn new() -> Self {
        Table {
            status: [PhilStat::Thinking; PHILNUM],
        }
    }
}

fn printstate(table: &Mutex<Table>) {
    let table = table.lock().unwrap();
    print!("|");
    for stat in &table.status {
        print!(" {} |", stat.as_str());
    }
    println!();
    io::stdout().flush().unwrap();
}

fn test(table: &Mutex<Table>, condvars: &[Condvar; PHILNUM], i: usize) -> bool {
    let mut table = table.lock().unwrap();
    if table.status[i] == PhilStat::Hungry
        && table.status[(i + 1) % PHILNUM] != PhilStat::Eating
        && table.status[(i + PHILNUM - 1) % PHILNUM] != PhilStat::Eating
    {
        table.status[i] = PhilStat::Eating;
        condvars[i].notify_one();
        true
    } else {
        false
    }
}

fn pickup(table: &Mutex<Table>, condvars: &[Condvar; PHILNUM], k: usize) {
    let mut table = table.lock().unwrap();
    table.status[k] = PhilStat::Hungry;
    printstate(table);
    drop(table);
    
    let mut table = condvars[k]
        .wait_while(table.lock().unwrap(), |t| t.status[k] != PhilStat::Eating)
        .unwrap();
    printstate(&table);
}

fn putdown(table: &Mutex<Table>, condvars: &[Condvar; PHILNUM], k: usize) {
    let mut table = table.lock().unwrap();
    table.status[k] = PhilStat::Thinking;
    printstate(&table);
    
    let next = (k + 1) % PHILNUM;
    let prev = (k + PHILNUM - 1) % PHILNUM;
    drop(table);
    
    test(table, condvars, next);
    test(table, condvars, prev);
}

fn philosopher(
    who: usize,
    table: Arc<Mutex<Table>>,
    condvars: Arc<[Condvar; PHILNUM]>,
    running: Arc<AtomicBool>,
) {
    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs((who + 1) as u64));
        pickup(&table, &condvars, who);
        thread::sleep(Duration::from_secs(1));
        putdown(&table, &condvars, who);
    }
}

fn main() {
    println!("This is TEST_PHILO, a Pth test showing the Five Dining Philosophers\n");
    println!("This is a demonstration showing the famous concurrency problem of the");
    println!("Five Dining Philosophers as analysed 1965 by E.W.Dijkstra:\n");
    println!("Five philosophers are sitting around a round table, each with a bowl of");
    println!("Chinese food in front of him. Between periods of talking they may start");
    println!("eating whenever they want to, with their bowls being filled frequently.");
    println!("But there are only five chopsticks available, one each to the left of");
    println!("each bowl - and for eating Chinese food one needs two chopsticks. When");
    println!("a philosopher wants to start eating, he must pick up the chopstick to");
    println!("the left of his bowl and the chopstick to the right of his bowl. He");
    println!("may find, however, that either one (or even both) of the chopsticks is");
    println!("unavailable as it is being used by another philosopher sitting on his");
    println!("right or left, so he has to wait.\n");
    println!("This situation shows classical contention under concurrency (the");
    println!("philosophers want to grab the chopsticks) and the possibility of a");
    println!("deadlock (all philosophers wait that the chopstick to their left becomes");
    println!("available).\n");
    println!("The demonstration runs max. 60 seconds. To stop before, press CTRL-C.\n");
    println!("+----P1----+----P2----+----P3----+----P4----+----P5----+");

    let table = Arc::new(Mutex::new(Table::new()));
    let condvars = Arc::new(array_init::array_init(|_| Condvar::new()));
    let running = Arc::new(AtomicBool::new(true));

    let mut handles = vec![];
    for i in 0..PHILNUM {
        let table = table.clone();
        let condvars = condvars.clone();
        let running = running.clone();
        handles.push(thread::spawn(move || {
            philosopher(i, table, condvars, running);
        }));
    }

    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        running.store(false, Ordering::Relaxed);
        tx.send(()).unwrap();
    }).unwrap();

    match rx.recv_timeout(Duration::from_secs(60)) {
        Ok(_) => println!("\nCTRL-C received, stopping..."),
        Err(_) => println!("\n60 seconds elapsed, stopping..."),
    }

    running.store(false, Ordering::Relaxed);
    for handle in handles {
        handle.join().unwrap();
    }

    println!("+----------+----------+----------+----------+----------+");
}