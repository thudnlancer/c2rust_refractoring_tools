/* Extract RCS keyword string values from working files.

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1990, 1991, 1992, 1993, 1994, 1995 Paul Eggert
   Copyright (C) 1982, 1988, 1989 Walter Tichy

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::str;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

const KDELIM: char = '$';
const VDELIM: char = ':';
const SINGLE: usize = 1;
const keylength: usize = 32;

struct Fro {
    file: Option<File>,
    buffer: Vec<u8>,
    pos: usize,
}

impl Fro {
    fn open(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        Ok(Fro {
            file: Some(file),
            buffer: Vec::new(),
            pos: 0,
        })
    }

    fn close(&mut self) {
        self.file = None;
    }

    fn bob(&mut self) {
        self.pos = 0;
    }

    fn get_char(&mut self) -> io::Result<Option<char>> {
        if self.pos >= self.buffer.len() {
            self.buffer.clear();
            self.pos = 0;
            if let Some(file) = &mut self.file {
                let mut buf = [0; 1024];
                let n = file.read(&mut buf)?;
                if n == 0 {
                    return Ok(None);
                }
                self.buffer.extend_from_slice(&buf[..n]);
            } else {
                return Ok(None);
            }
        }

        let c = self.buffer[self.pos] as char;
        self.pos += 1;
        Ok(Some(c))
    }
}

struct Prev {
    author: Option<String>,
    date: Option<String>,
    name: Option<String>,
    rev: Option<String>,
    state: Option<String>,
    valid: bool,
}

struct Mani {
    filename: String,
    prev: Prev,
}

fn sorry(save: bool, msg: Option<&str>) -> Option<String> {
    if save {
        // Simulate finish_string and brush_off
    }
    if let Some(msg) = msg {
        eprintln!("{}", msg);
    }
    None
}

fn badly_terminated(save: bool) -> Option<String> {
    sorry(save, Some("badly terminated keyword value"))
}

fn get0val(c: Option<char>, fp: &mut Fro, save: bool, optional: bool) -> Option<String> {
    let mut val = String::new();
    let mut got1 = false;

    let mut current_char = c;
    loop {
        match current_char {
            Some(ch) => match ch {
                ' ' | '\t' => {
                    if save {
                        if got1 {
                            return Some(val);
                        } else {
                            return None;
                        }
                    } else {
                        return if got1 { Some("non-NULL".to_string()) } else { None };
                    }
                }
                KDELIM => {
                    if !got1 && optional {
                        return None;
                    }
                    return badly_terminated(save);
                }
                '\n' | '\0' => return badly_terminated(save),
                _ => {
                    got1 = true;
                    if save {
                        val.push(ch);
                    }
                }
            },
            None => return badly_terminated(save),
        }
        current_char = fp.get_char().unwrap_or(None);
    }
}

fn keepid(c: Option<char>, fp: &mut Fro) -> Option<String> {
    let maybe = get0val(c, fp, true, false)?;
    // Simulate checksid
    // if erroneous {
    //     return None;
    // }
    Some(maybe)
}

fn getval(fp: &mut Fro, save: bool, optional: bool) -> Option<String> {
    let c = fp.get_char().unwrap_or(None)?;
    get0val(Some(c), fp, save, optional)
}

fn keepdate(fp: &mut Fro) -> Option<char> {
    let d = getval(fp, true, false)?;
    let t = getval(fp, true, false)?;
    let c = fp.get_char().unwrap_or(None)?;

    let mut buf = String::new();
    if d.chars().take(2).all(|c| c.is_ascii_digit()) && !d.chars().nth(2).map_or(false, |c| c.is_ascii_digit()) {
        buf.push_str("19");
    }
    buf.push_str(&d);
    buf.push(' ');
    buf.push_str(&t);
    if !t.contains('-') && !t.contains('+') {
        buf.push_str("+0000");
    }

    // Simulate intern
    let date = buf;
    // Set PREV(date) = date
    Some(c)
}

fn keeprev(fp: &mut Fro) -> Option<String> {
    let s = getval(fp, true, false)?;
    let mut dotcount = 0;
    let mut valid = true;

    for c in s.chars() {
        match c {
            '.' => dotcount += 1,
            '0'..='9' => continue,
            _ => {
                valid = false;
                break;
            }
        }
    }

    if valid && dotcount % 2 == 1 {
        Some(s)
    } else {
        eprintln!("{} is not a revision number", s);
        None
    }
}

fn getoldkeys(fp: Option<&mut Fro>, mani: &mut Mani) -> bool {
    if mani.prev.valid {
        return true;
    }

    let mut needs_closing = false;
    let mut fp_inner = if let Some(fp) = fp {
        fp
    } else {
        let mut new_fp = Fro::open(&mani.filename).unwrap();
        needs_closing = true;
        &mut new_fp
    };

    let mut c = Some('\0');
    let mut keyword = String::with_capacity(keylength + 1);

    loop {
        if c == Some(KDELIM) {
            loop {
                keyword.clear();
                c = fp_inner.get_char().unwrap_or(None);
                match c {
                    Some(ch) => match ch {
                        KDELIM => continue,
                        VDELIM => break,
                        '\n' => continue,
                        _ => {
                            if keyword.len() < keylength {
                                keyword.push(ch);
                            } else {
                                break;
                            }
                        }
                    },
                    None => break,
                }
            }

            if c != Some(VDELIM) {
                continue;
            }

            keyword.push(VDELIM);
            c = fp_inner.get_char().unwrap_or(None);
            match c {
                Some(' ') | Some('\t') => (),
                _ => continue,
            }

            // Simulate recognize_keyword
            let match_i = match keyword.as_str() {
                "Author" => 0,
                "Date" => 1,
                "Header" | "Id" => 2,
                "Locker" => 3,
                "Log" | "RCSfile" | "Source" => 4,
                "Name" => 5,
                "Revision" => 6,
                "State" => 7,
                _ => continue,
            };

            match match_i {
                0 => {
                    mani.prev.author = keepid(None, fp_inner);
                    c = None;
                }
                1 => {
                    c = keepdate(fp_inner);
                }
                2 => {
                    if getval(fp_inner, false, false).is_some()
                        && keeprev(fp_inner).is_some()
                        && keepdate(fp_inner).is_some()
                        && keepid(None, fp_inner).is_some()
                        && keepid(None, fp_inner).is_some()
                    {
                        if getval(fp_inner, false, true).is_some()
                            && getval(fp_inner, false, true).is_some()
                        {
                            c = None;
                        } else {
                            c = Some(KDELIM);
                        }
                    } else {
                        return false;
                    }
                }
                3 => {
                    getval(fp_inner, false, false);
                    c = None;
                }
                4 => {
                    if getval(fp_inner, false, false).is_none() {
                        return false;
                    }
                    c = None;
                }
                5 => {
                    mani.prev.name = getval(fp_inner, true, false);
                    if let Some(name) = &mani.prev.name {
                        if name.is_empty() {
                            mani.prev.name = None;
                        }
                        // Simulate checkssym
                    }
                    c = None;
                }
                6 => {
                    if keeprev(fp_inner).is_none() {
                        return false;
                    }
                    c = None;
                }
                7 => {
                    mani.prev.state = keepid(None, fp_inner);
                    c = None;
                }
                _ => continue,
            }

            if c.is_none() {
                c = fp_inner.get_char().unwrap_or(None);
            }

            if c != Some(KDELIM) {
                eprintln!("closing {} missing on keyword", KDELIM);
                return false;
            }

            if mani.prev.name.is_some()
                && mani.prev.author.is_some()
                && mani.prev.date.is_some()
                && mani.prev.rev.is_some()
                && mani.prev.state.is_some()
            {
                break;
            }
        }
        c = fp_inner.get_char().unwrap_or(None);
    }

    if needs_closing {
        fp_inner.close();
    } else {
        fp_inner.bob();
    }

    // Prune empty strings
    if let Some(author) = &mani.prev.author {
        if author.is_empty() {
            mani.prev.author = None;
        }
    }
    if let Some(date) = &mani.prev.date {
        if date.is_empty() {
            mani.prev.date = None;
        }
    }
    if let Some(name) = &mani.prev.name {
        if name.is_empty() {
            mani.prev.name = None;
        }
    }
    if let Some(rev) = &mani.prev.rev {
        if rev.is_empty() {
            mani.prev.rev = None;
        }
    }
    if let Some(state) = &mani.prev.state {
        if state.is_empty() {
            mani.prev.state = None;
        }
    }

    mani.prev.valid = true;
    true
}