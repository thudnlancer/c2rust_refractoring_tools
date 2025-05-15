use std::sync::{Once, ONCE_INIT};
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ThreadError {
    CreationFailed,
    JoinFailed,
    NoThreadSupport,
}

impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThreadError::CreationFailed => write!(f, "Thread creation failed"),
            ThreadError::JoinFailed => write!(f, "Thread join failed"),
            ThreadError::NoThreadSupport => write!(f, "Thread support not available"),
        }
    }
}

impl Error for ThreadError {}

pub type GlThread = thread::JoinHandle<()>;

pub fn gl_thread_create<F, T>(func: F) -> Result<GlThread, ThreadError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new()
        .spawn(move || {
            func();
        })
        .map_err(|_| ThreadError::CreationFailed)
}

pub fn gl_thread_join(thread: GlThread) -> Result<(), ThreadError> {
    thread.join().map_err(|_| ThreadError::JoinFailed)
}

pub fn gl_thread_self() -> thread::ThreadId {
    thread::current().id()
}

pub fn gl_thread_exit() -> ! {
    std::process::exit(0)
}

pub struct ThreadAtFork {
    prepare: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
    parent: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
    child: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
}

impl ThreadAtFork {
    pub fn new(
        prepare: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
        parent: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
        child: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
    ) -> Self {
        ThreadAtFork {
            prepare,
            parent,
            child,
        }
    }

    pub fn execute(&self) {
        if let Some(prepare) = &self.prepare {
            let prepare = prepare.lock().unwrap();
            prepare();
        }
        
        // In Rust, fork() is not directly supported due to safety concerns
        // This is a placeholder for the fork handling logic
        // Actual fork handling would require unsafe code and careful consideration
    }
}

static INIT: Once = ONCE_INIT;

pub fn init_thread_support() {
    INIT.call_once(|| {
        // Initialization code if needed
    });
}

pub mod windows_thread {
    use super::*;
    
    pub fn create_thread<F, T>(func: F) -> Result<GlThread, ThreadError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        gl_thread_create(func)
    }
    
    pub fn join_thread(thread: GlThread) -> Result<(), ThreadError> {
        gl_thread_join(thread)
    }
    
    pub fn thread_self() -> thread::ThreadId {
        gl_thread_self()
    }
    
    pub fn thread_exit() -> ! {
        gl_thread_exit()
    }
}