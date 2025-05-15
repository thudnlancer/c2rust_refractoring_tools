use std::ffi::CStr;
use std::io::{self, Write};
use std::panic;

type GslErrorHandler = Box<dyn Fn(&str, &str, i32, i32) + Send + Sync>;

static GSL_ERROR_HANDLER: std::sync::OnceLock<Option<GslErrorHandler>> = std::sync::OnceLock::new();

pub fn gsl_error(reason: &str, file: &str, line: i32, gsl_errno: i32) {
    if let Some(handler) = GSL_ERROR_HANDLER.get().and_then(|h| h.as_ref()) {
        handler(reason, file, line, gsl_errno);
        return;
    }

    gsl_stream_printf("ERROR", file, line, reason);
    io::stdout().flush().ok();
    eprintln!("Default GSL error handler invoked.");
    io::stderr().flush().ok();
    panic::abort();
}

pub fn gsl_set_error_handler(new_handler: Option<GslErrorHandler>) -> Option<GslErrorHandler> {
    GSL_ERROR_HANDLER.get_or_init(|| None).take()
}

pub fn gsl_set_error_handler_off() -> Option<GslErrorHandler> {
    gsl_set_error_handler(Some(Box::new(|_, _, _, _| {})))
}

fn gsl_stream_printf(label: &str, file: &str, line: i32, reason: &str) {
    eprintln!("{}: {}:{}: {}", label, file, line, reason);
}

fn no_error_handler(_reason: &str, _file: &str, _line: i32, _gsl_errno: i32) {
    // No-op handler
}