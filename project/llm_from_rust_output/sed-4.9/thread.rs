use std::thread;
use std::io::{self, Error};

pub type GlThreadId = thread::ThreadId;

pub fn gl_thread_create<F>(func: F) -> Result<GlThreadId, Error>
where
    F: FnOnce() + Send + 'static,
{
    let handle = thread::spawn(func);
    Ok(handle.thread().id())
}