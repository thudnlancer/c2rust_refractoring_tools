use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

type PthTime = SystemTime;
type PthStatus = u32;
type PthState = u32;

const PTH_STATUS_PENDING: PthStatus = 0;
const PTH_STATUS_OCCURRED: PthStatus = 1;
const PTH_STATUS_FAILED: PthStatus = 2;

const PTH_STATE_SCHEDULER: PthState = 0;
const PTH_STATE_NEW: PthState = 1;
const PTH_STATE_READY: PthState = 2;
const PTH_STATE_WAITING: PthState = 3;
const PTH_STATE_DEAD: PthState = 4;

struct PthThread {
    state: AtomicU32,
    cancelreq: AtomicBool,
    cancelstate: AtomicU32,
    joinable: bool,
    join_arg: Option<Arc<Mutex<Option<i32>>>>,
}

impl PthThread {
    fn new() -> Self {
        PthThread {
            state: AtomicU32::new(PTH_STATE_NEW),
            cancelreq: AtomicBool::new(false),
            cancelstate: AtomicU32::new(0),
            joinable: false,
            join_arg: None,
        }
    }

    fn cancel_state(&self, newstate: Option<i32>, oldstate: &mut Option<i32>) {
        if let Some(old) = oldstate {
            *old = self.cancelstate.load(Ordering::Relaxed) as i32;
        }
        if let Some(new) = newstate {
            self.cancelstate.store(new as u32, Ordering::Relaxed);
        }
    }

    fn cancel_point(&self) {
        if self.cancelreq.load(Ordering::Relaxed) 
            && self.cancelstate.load(Ordering::Relaxed) & (1 << 0) != 0 
        {
            self.cancelreq.store(false, Ordering::Relaxed);
            thread::exit();
        }
    }
}

struct PthQueue {
    threads: Vec<Arc<PthThread>>,
}

impl PthQueue {
    fn new() -> Self {
        PthQueue { threads: Vec::new() }
    }

    fn contains(&self, thread: &Arc<PthThread>) -> bool {
        self.threads.contains(thread)
    }

    fn delete(&mut self, thread: &Arc<PthThread>) {
        self.threads.retain(|t| !Arc::ptr_eq(t, thread));
    }

    fn insert(&mut self, thread: Arc<PthThread>) {
        self.threads.push(thread);
    }
}

struct PthSystem {
    current: Arc<PthThread>,
    nq: PthQueue,
    rq: PthQueue,
    wq: PthQueue,
    dq: PthQueue,
}

impl PthSystem {
    fn new() -> Self {
        PthSystem {
            current: Arc::new(PthThread::new()),
            nq: PthQueue::new(),
            rq: PthQueue::new(),
            wq: PthQueue::new(),
            dq: PthQueue::new(),
        }
    }

    fn cancel(&mut self, thread: Arc<PthThread>) -> bool {
        if Arc::ptr_eq(&thread, &self.current) {
            return false;
        }

        if thread.state.load(Ordering::Relaxed) == PTH_STATE_DEAD {
            return false;
        }

        thread.cancelreq.store(true, Ordering::Relaxed);

        if thread.cancelstate.load(Ordering::Relaxed) & (1 << 0) != 0
            && thread.cancelstate.load(Ordering::Relaxed) & (1 << 2) != 0
        {
            let q = match thread.state.load(Ordering::Relaxed) {
                PTH_STATE_NEW => &mut self.nq,
                PTH_STATE_READY => &mut self.rq,
                PTH_STATE_WAITING => &mut self.wq,
                _ => return false,
            };

            if !q.contains(&thread) {
                return false;
            }

            q.delete(&thread);

            if thread.joinable {
                thread.join_arg = Some(Arc::new(Mutex::new(Some(-1))));
                thread.state.store(PTH_STATE_DEAD, Ordering::Relaxed);
                self.dq.insert(thread);
            }
        }

        true
    }

    fn abort(&mut self, thread: Arc<PthThread>) -> bool {
        if Arc::ptr_eq(&thread, &self.current) {
            return false;
        }

        if thread.state.load(Ordering::Relaxed) == PTH_STATE_DEAD && thread.joinable {
            // Simulate join behavior
            true
        } else {
            thread.joinable = false;
            thread.cancelstate.store((1 << 0) | (1 << 2), Ordering::Relaxed);
            self.cancel(thread)
        }
    }
}