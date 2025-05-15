use std::env;
use std::io;

pub fn xgetcwd() -> io::Result<String> {
    env::current_dir().map(|path| path.to_string_lossy().into_owned())
}