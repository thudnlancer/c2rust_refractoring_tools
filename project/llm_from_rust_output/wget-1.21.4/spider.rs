use std::ffi::{CStr, CString};
use std::collections::HashSet;

pub enum LogOptions {
    Verbose,
    NotQuiet,
    NonVerbose,
    Always,
    Progress,
}

static mut NONEXISTING_URLS_SET: Option<HashSet<CString>> = None;

pub fn nonexisting_url(url: &CStr) {
    if is_robots_txt_url(url) {
        return;
    }

    unsafe {
        if NONEXISTING_URLS_SET.is_none() {
            NONEXISTING_URLS_SET = Some(HashSet::new());
        }
        if let Some(set) = &mut NONEXISTING_URLS_SET {
            set.insert(url.to_owned());
        }
    }
}

pub fn print_broken_links() {
    unsafe {
        if NONEXISTING_URLS_SET.is_none() {
            logprintf(
                LogOptions::NotQuiet,
                &CStr::from_bytes_with_nul(b"Found no broken links.\n\n\0").unwrap(),
            );
            return;
        }

        let num_elems = NONEXISTING_URLS_SET.as_ref().unwrap().len();
        let msg = if num_elems == 1 {
            CStr::from_bytes_with_nul(b"Found %d broken link.\n\n\0").unwrap()
        } else {
            CStr::from_bytes_with_nul(b"Found %d broken links.\n\n\0").unwrap()
        };

        logprintf(LogOptions::NotQuiet, msg, num_elems);

        for url in NONEXISTING_URLS_SET.as_ref().unwrap() {
            logprintf(
                LogOptions::NotQuiet,
                &CStr::from_bytes_with_nul(b"%s\n\0").unwrap(),
                url.as_c_str(),
            );
        }

        logputs(LogOptions::NotQuiet, &CStr::from_bytes_with_nul(b"\n\0").unwrap());
    }
}

fn logprintf(_: LogOptions, _: &CStr, _: ...) {
    // Implementation omitted
}

fn logputs(_: LogOptions, _: &CStr) {
    // Implementation omitted
}

fn is_robots_txt_url(_: &CStr) -> bool {
    // Implementation omitted
    false
}