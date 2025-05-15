use std::sync::{Mutex, Once};
use std::ffi::c_void;
use nix::unistd::Pid;
use nix::errno::errno;

type ForkHandler = Option<fn(*mut c_void)>;

struct ForkHandlers {
    prepare: ForkHandler,
    parent: ForkHandler,
    child: ForkHandler,
    arg: *mut c_void,
}

impl ForkHandlers {
    fn new() -> Self {
        ForkHandlers {
            prepare: None,
            parent: None,
            child: None,
            arg: std::ptr::null_mut(),
        }
    }
}

struct AtFork {
    handlers: Mutex<Vec<ForkHandlers>>,
}

impl AtFork {
    fn new() -> Self {
        AtFork {
            handlers: Mutex::new(Vec::with_capacity(128)),
        }
    }

    fn push(&self, prepare: ForkHandler, parent: ForkHandler, child: ForkHandler, arg: *mut c_void) -> Result<(), i32> {
        let mut handlers = self.handlers.lock().unwrap();
        if handlers.len() >= 128 {
            return Err(12); // ENOMEM
        }
        handlers.push(ForkHandlers {
            prepare,
            parent,
            child,
            arg,
        });
        Ok(())
    }

    fn pop(&self) -> Result<(), i32> {
        let mut handlers = self.handlers.lock().unwrap();
        if handlers.is_empty() {
            return Err(0);
        }
        handlers.pop();
        Ok(())
    }

    fn fork(&self) -> Result<Pid, i32> {
        let handlers = self.handlers.lock().unwrap();
        
        // Run prepare handlers in reverse order
        for handler in handlers.iter().rev() {
            if let Some(prepare) = handler.prepare {
                prepare(handler.arg);
            }
        }

        match nix::unistd::fork() {
            Ok(Pid::Parent(pid)) => {
                // Run parent handlers in order
                for handler in handlers.iter() {
                    if let Some(parent) = handler.parent {
                        parent(handler.arg);
                    }
                }
                Ok(Pid::Parent(pid))
            }
            Ok(Pid::Child) => {
                // Run child handlers in order
                for handler in handlers.iter() {
                    if let Some(child) = handler.child {
                        child(handler.arg);
                    }
                }
                Ok(Pid::Child)
            }
            Err(_) => Err(errno() as i32),
        }
    }
}

lazy_static! {
    static ref AT_FORK: AtFork = AtFork::new();
}

pub fn pth_atfork_push(
    prepare: ForkHandler,
    parent: ForkHandler,
    child: ForkHandler,
    arg: *mut c_void,
) -> i32 {
    AT_FORK.push(prepare, parent, child, arg).map_or_else(|e| e, |_| 1)
}

pub fn pth_atfork_pop() -> i32 {
    AT_FORK.pop().map_or_else(|e| e, |_| 1)
}

pub fn pth_fork() -> i32 {
    AT_FORK.fork().map_or_else(|e| e, |pid| match pid {
        Pid::Parent(p) => p.as_raw(),
        Pid::Child => 0,
    })
}