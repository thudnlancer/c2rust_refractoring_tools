use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "You have to build Pth with --with-sfio to run this test!")?;
    Ok(())
}