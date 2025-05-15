use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;
use std::collections::HashMap;

#[derive(Debug)]
struct NetrcEntry {
    host: Option<String>,
    login: Option<String>,
    password: Option<String>,
}

struct NetrcParser {
    entries: Vec<NetrcEntry>,
    current_entry: Option<NetrcEntry>,
}

impl NetrcParser {
    fn new() -> Self {
        NetrcParser {
            entries: Vec::new(),
            current_entry: None,
        }
    }

    fn parse_file(path: &Path) -> Result<Vec<NetrcEntry>, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut parser = NetrcParser::new();

        for line in reader.lines() {
            let line = line?;
            parser.parse_line(&line);
        }

        parser.finalize_entry();
        Ok(parser.entries)
    }

    fn parse_line(&mut self, line: &str) {
        let mut tokens = line.split_whitespace();
        while let Some(token) = tokens.next() {
            match token.to_lowercase().as_str() {
                "machine" => {
                    self.finalize_entry();
                    if let Some(host) = tokens.next() {
                        self.current_entry = Some(NetrcEntry {
                            host: Some(host.to_string()),
                            login: None,
                            password: None,
                        });
                    }
                }
                "default" => {
                    self.finalize_entry();
                    self.current_entry = Some(NetrcEntry {
                        host: None,
                        login: None,
                        password: None,
                    });
                }
                "login" => {
                    if let Some(login) = tokens.next() {
                        if let Some(ref mut entry) = self.current_entry {
                            entry.login = Some(login.to_string());
                        }
                    }
                }
                "password" => {
                    if let Some(password) = tokens.next() {
                        if let Some(ref mut entry) = self.current_entry {
                            entry.password = Some(password.to_string());
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    fn finalize_entry(&mut self) {
        if let Some(entry) = self.current_entry.take() {
            self.entries.push(entry);
        }
    }
}

pub fn search_netrc(
    host: &str,
    login: Option<&str>,
    password: Option<&str>,
    slack_default: bool,
) -> Option<(Option<String>, Option<String>)> {
    let home_dir = dirs::home_dir()?;
    let netrc_path = home_dir.join(".netrc");
    
    let entries = match NetrcParser::parse_file(&netrc_path) {
        Ok(e) => e,
        Err(_) => return None,
    };

    // First try to find exact host match
    if let Some(entry) = entries.iter().find(|e| e.host.as_ref().map(|h| h == host).unwrap_or(false)) {
        return Some((entry.login.clone(), entry.password.clone()));
    }

    // If no exact match and slack_default is true, return default entry
    if slack_default {
        if let Some(entry) = entries.iter().find(|e| e.host.is_none()) {
            return Some((entry.login.clone(), entry.password.clone()));
        }
    }

    None
}

// Safe wrapper for C interop
#[no_mangle]
pub extern "C" fn search_netrc_c(
    host: *const c_char,
    acc: *mut *const c_char,
    passwd: *mut *const c_char,
    slack_default: c_int,
    _fp_netrc: *mut c_void,
) {
    if host.is_null() || acc.is_null() || passwd.is_null() {
        return;
    }

    let host_str = unsafe { CStr::from_ptr(host) }.to_string_lossy();
    let slack = slack_default != 0;

    if let Some((login, password)) = search_netrc(&host_str, None, None, slack) {
        unsafe {
            if !login.is_none() {
                *acc = CString::new(login.unwrap()).unwrap().into_raw();
            }
            if !password.is_none() {
                *passwd = CString::new(password.unwrap()).unwrap().into_raw();
            }
        }
    }
}

// Helper to free allocated strings
#[no_mangle]
pub extern "C" fn free_netrc_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s) };
    }
}