use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::io;

pub type gl_thread_t = thread::JoinHandle<()>;

pub fn gl_thread_create<F>(func: F) -> io::Result<gl_thread_t>
where
    F: FnOnce() + Send + 'static,
{
    thread::Builder::new().spawn(func)
}