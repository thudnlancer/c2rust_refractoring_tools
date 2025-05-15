use std::env;
use std::io;

pub fn xgetcwd() -> Result<String, io::Error> {
    env::current_dir().map(|path| path.to_string_lossy().into_owned())
}