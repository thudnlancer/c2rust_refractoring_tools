use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;

#[derive(Debug)]
struct Acc {
    host: Option<String>,
    acc: String,
    passwd: Option<String>,
    next: Option<Box<Acc>>,
}

impl Acc {
    fn new() -> Self {
        Acc {
            host: None,
            acc: String::new(),
            passwd: None,
            next: None,
        }
    }
}

static mut NETRC_LIST: Option<Box<Acc>> = None;
static mut PROCESSED_NETRC: bool = false;

fn search_netrc(
    host: &str,
    acc: &mut Option<String>,
    passwd: &mut Option<String>,
    slack_default: bool,
    fp_netrc: Option<File>,
) {
    unsafe {
        if !PROCESSED_NETRC {
            NETRC_LIST = None;
            PROCESSED_NETRC = true;

            if let Some(fp) = fp_netrc {
                NETRC_LIST = parse_netrc_fp(".netrc", fp);
            } else if let Some(homedir) = opt.homedir {
                let path = Path::new(&homedir).join(NETRC_FILE_NAME);
                if path.exists() {
                    NETRC_LIST = parse_netrc(&path);
                }
            }
        }

        if NETRC_LIST.is_none() {
            return;
        }

        if acc.is_some() && passwd.is_some() {
            return;
        }

        let mut current = NETRC_LIST.as_ref();
        while let Some(entry) = current {
            if let Some(entry_host) = &entry.host {
                if entry_host.eq_ignore_ascii_case(host) {
                    break;
                }
            }
            current = entry.next.as_ref().map(|b| b.as_ref());
        }

        if let Some(entry) = current {
            if let Some(acc_str) = acc {
                if entry.acc == *acc_str {
                    *passwd = entry.passwd.clone();
                } else {
                    *passwd = None;
                }
            } else {
                *acc = Some(entry.acc.clone());
                if entry.passwd.is_some() {
                    *passwd = entry.passwd.clone();
                }
            }
            return;
        } else {
            if !slack_default || acc.is_some() {
                return;
            }

            let mut default_entry = NETRC_LIST.as_ref();
            while let Some(entry) = default_entry {
                if entry.host.is_none() {
                    break;
                }
                default_entry = entry.next.as_ref().map(|b| b.as_ref());
            }

            if let Some(entry) = default_entry {
                *acc = Some(entry.acc.clone());
                if passwd.is_none() {
                    *passwd = entry.passwd.clone();
                }
            }
        }
    }
}

fn parse_netrc_fp(path: &str, fp: File) -> Option<Box<Acc>> {
    let reader = BufReader::new(fp);
    let mut current = None;
    let mut retval = None;
    let mut last_token = Token::Nothing;
    let mut premature_token = None;

    for (ln, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut p = line.trim_start();

        if last_token == Token::Macdef && p.is_empty() {
            last_token = Token::Nothing;
        }

        while !p.is_empty() && last_token != Token::Macdef {
            p = p.trim_start();
            if p.starts_with('#') || p.is_empty() {
                break;
            }

            let mut qmark = false;
            let mut tok = p;
            if p.starts_with('"') {
                qmark = true;
                tok = &p[1..];
            }

            let end = tok.find(|c| {
                if qmark {
                    c == '"'
                } else {
                    c.is_whitespace()
                }
            }).unwrap_or(tok.len());

            let token = &tok[..end];
            p = &tok[end..];

            if qmark && !p.is_empty() && p.starts_with('"') {
                p = &p[1..];
            }

            match last_token {
                Token::Login => {
                    if let Some(ref mut curr) = current {
                        curr.acc = token.to_string();
                    } else {
                        premature_token = Some("login");
                    }
                }
                Token::Machine => {
                    maybe_add_to_list(&mut current, &mut retval);
                    if let Some(ref mut curr) = current {
                        curr.host = Some(token.to_string());
                    }
                }
                Token::Password => {
                    if let Some(ref mut curr) = current {
                        curr.passwd = Some(token.to_string());
                    } else {
                        premature_token = Some("password");
                    }
                }
                Token::Macdef => {
                    if current.is_none() {
                        premature_token = Some("macdef");
                    }
                }
                Token::Account | Token::Port | Token::Force => {
                    if current.is_none() {
                        premature_token = Some(match last_token {
                            Token::Account => "account",
                            Token::Port => "port",
                            Token::Force => "force",
                            _ => unreachable!(),
                        });
                    }
                }
                Token::Nothing => {}
            }

            if let Some(token) = premature_token {
                eprintln!(
                    "{}: {}:{}: warning: {} token appears before any machine name",
                    exec_name, path, ln + 1, token
                );
                premature_token = None;
            }

            if last_token != Token::Nothing {
                last_token = Token::Nothing;
            } else {
                last_token = match token {
                    "account" => Token::Account,
                    "default" => {
                        maybe_add_to_list(&mut current, &mut retval);
                        Token::Nothing
                    }
                    "login" => Token::Login,
                    "macdef" => Token::Macdef,
                    "machine" => Token::Machine,
                    "password" => Token::Password,
                    "port" => Token::Port,
                    "force" => Token::Force,
                    _ => {
                        eprintln!("{}: {}:{}: unknown token \"{}\"", exec_name, path, ln + 1, token);
                        Token::Nothing
                    }
                };
            }
        }
    }

    maybe_add_to_list(&mut current, &mut retval);
    current = None;

    let mut reversed = None;
    while let Some(mut entry) = retval {
        let next = entry.next.take();
        entry.next = reversed;
        reversed = Some(entry);
        retval = next;
    }

    reversed
}

fn parse_netrc(path: &Path) -> Option<Box<Acc>> {
    match File::open(path) {
        Ok(fp) => parse_netrc_fp(path.to_str().unwrap(), fp),
        Err(e) => {
            eprintln!(
                "{}: Cannot read {} ({})",
                exec_name,
                path.display(),
                e
            );
            None
        }
    }
}

fn maybe_add_to_list(newentry: &mut Option<Box<Acc>>, list: &mut Option<Box<Acc>>) {
    if let Some(entry) = newentry.take() {
        if entry.acc.is_empty() {
            // Free allocated space
            return;
        }

        let mut new_entry = Box::new(Acc::new());
        new_entry.next = list.take();
        *list = Some(new_entry);
    }

    *newentry = Some(Box::new(Acc::new()));
}

#[derive(PartialEq)]
enum Token {
    Nothing,
    Account,
    Login,
    Macdef,
    Machine,
    Password,
    Port,
    Force,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_netrc() {
        let tests = vec![
            ("a\\b", "ab"),
            ("a\\\\b", "a\\b"),
            ("\"a\\\\b\"", "a\\b"),
            ("\"a\\\"b\"", "a\"b"),
            ("a\"b", "a\"b"),
            ("a\\\\\\\\b", "a\\\\b"),
            ("a\\\\", "a\\"),
            ("\"a\\\\\"", "a\\"),
            ("a\\", "a"),
            ("\"a b\"", "a b"),
            ("a b", "a"),
        ];

        for (input, expected) in tests {
            let netrc = format!(
                "machine localhost\n\tlogin me\n\tpassword {}",
                input
            );
            let cursor = Cursor::new(netrc);
            let acc = parse_netrc_fp("memory", cursor).unwrap();
            assert_eq!(acc.passwd.unwrap(), expected);
        }
    }
}