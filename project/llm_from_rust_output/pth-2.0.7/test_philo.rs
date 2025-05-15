use std::ffi::CString;
use std::ptr;
use std::process;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum PhilosopherState {
    Thinking,
    Hungry,
    Eating,
}

struct Table {
    status: [PhilosopherState; 5],
    condition: [Arc<(Mutex<()>, Condvar)>; 5],
}

impl Table {
    fn new() -> Self {
        let conditions = [
            Arc::new((Mutex::new(()), Condvar::new())),
            Arc::new((Mutex::new(()), Condvar::new())),
            Arc::new((Mutex::new(()), Condvar::new())),
            Arc::new((Mutex::new(()), Condvar::new())),
            Arc::new((Mutex::new(()), Condvar::new())),
        ];
        
        Table {
            status: [PhilosopherState::Thinking; 5],
            condition: conditions,
        }
    }

    fn print_state(&self) {
        let states: Vec<String> = self.status.iter().map(|s| match s {
            PhilosopherState::Thinking => "thinking".to_string(),
            PhilosopherState::Hungry => "hungry  ".to_string(),
            PhilosopherState::Eating => "EATING  ".to_string(),
        }).collect();
        
        println!("| {} | {} | {} | {} | {} |", 
            states[0], states[1], states[2], states[3], states[4]);
    }

    fn test(&mut self, i: usize) {
        if self.status[i] == PhilosopherState::Hungry &&
            self.status[(i + 1) % 5] != PhilosopherState::Eating &&
            self.status[(i + 4) % 5] != PhilosopherState::Eating 
        {
            self.status[i] = PhilosopherState::Eating;
            let (lock, cvar) = &*self.condition[i];
            let _guard = lock.lock().unwrap();
            cvar.notify_one();
        }
    }

    fn pickup(&mut self, i: usize) {
        let (lock, cvar) = &*self.condition[i];
        let _guard = lock.lock().unwrap();
        
        self.status[i] = PhilosopherState::Hungry;
        self.print_state();
        
        self.test(i);
        if self.status[i] != PhilosopherState::Eating {
            let _ = cvar.wait(_guard).unwrap();
        }
        
        self.print_state();
    }

    fn putdown(&mut self, i: usize) {
        let (lock, _) = &*self.condition[i];
        let _guard = lock.lock().unwrap();
        
        self.status[i] = PhilosopherState::Thinking;
        self.print_state();
        
        self.test((i + 1) % 5);
        self.test((i + 4) % 5);
    }
}

fn philosopher(table: Arc<Mutex<Table>>, id: usize) {
    loop {
        thread::sleep(Duration::from_secs((id + 1) as u64));
        table.lock().unwrap().pickup(id);
        thread::sleep(Duration::from_secs(1));
        table.lock().unwrap().putdown(id);
    }
}

fn main() {
    println!("This is TEST_PHILO, a Rust implementation showing the Five Dining Philosophers");
    println!();
    println!("This is a demonstration showing the famous concurrency problem of the");
    println!("Five Dining Philosophers as analysed 1965 by E.W.Dijkstra:");
    println!();
    println!("Five philosophers are sitting around a round table, each with a bowl of");
    println!("Chinese food in front of him. Between periods of talking they may start");
    println!("eating whenever they want to, with their bowls being filled frequently.");
    println!("But there are only five chopsticks available, one each to the left of");
    println!("each bowl - and for eating Chinese food one needs two chopsticks. When");
    println!("a philosopher wants to start eating, he must pick up the chopstick to");
    println!("the left of his bowl and the chopstick to the right of his bowl. He");
    println!("may find, however, that either one (or even both) of the chopsticks is");
    println!("unavailable as it is being used by another philosopher sitting on his");
    println!("right or left, so he has to wait.");
    println!();
    println!("This situation shows classical contention under concurrency (the");
    println!("philosophers want to grab the chopsticks) and the possibility of a");
    println!("deadlock (all philosophers wait that the chopstick to their left becomes");
    println!("available).");
    println!();
    println!("The demonstration runs max. 60 seconds. To stop before, press CTRL-C.");
    println!();
    println!("+----P1----+----P2----+----P3----+----P4----+----P5----+");

    let table = Arc::new(Mutex::new(Table::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let table = table.clone();
        let handle = thread::spawn(move || {
            philosopher(table, i);
        });
        handles.push(handle);
    }

    thread::sleep(Duration::from_secs(60));

    for handle in handles {
        handle.thread().unpark();
    }

    println!("+----------+----------+----------+----------+----------+");
}