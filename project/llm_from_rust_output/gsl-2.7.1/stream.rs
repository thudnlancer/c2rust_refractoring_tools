use std::ffi::CString;
use std::io::{self, Write};
use std::sync::OnceLock;

type GslStreamHandler = Box<dyn Fn(&str, &str, i32, &str)>;

static GSL_STREAM: OnceLock<Box<dyn Write + Send + Sync>> = OnceLock::new();
static GSL_STREAM_HANDLER: OnceLock<GslStreamHandler> = OnceLock::new();

pub fn gsl_stream_printf(label: &str, file: &str, line: i32, reason: &str) -> io::Result<()> {
    if let Some(handler) = GSL_STREAM_HANDLER.get() {
        handler(label, file, line, reason);
        return Ok(());
    }

    let stream = GSL_STREAM.get_or_init(|| Box::new(io::stderr()));
    writeln!(stream, "gsl: {}:{}: {}: {}", file, line, label, reason)?;
    Ok(())
}

pub fn gsl_set_stream_handler(new_handler: Option<GslStreamHandler>) -> Option<GslStreamHandler> {
    if let Some(handler) = new_handler {
        GSL_STREAM_HANDLER.set(handler).ok()
    } else {
        GSL_STREAM_HANDLER.take()
    }
}

pub fn gsl_set_stream(new_stream: Box<dyn Write + Send + Sync>) -> Option<Box<dyn Write + Send + Sync>> {
    GSL_STREAM.set(new_stream).ok()
}