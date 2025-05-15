/* Various processing of names.

   Copyright 1988-2021 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3, or (at your option) any later
   version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General
   Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program.  If not, see <http://www.gnu.org/licenses/>.  */

use std::collections::HashMap;
use std::ffi::{CString, OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};

use fnmatch::Pattern;
use libc::{self, c_char, c_int, c_void, size_t};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{fstat, FileStat, Mode};
use nix::unistd::{close, getcwd, getgid, getgrnam, getpwuid, getuid};

use crate::common::*;
use crate::errors::*;
use crate::exclude::*;
use crate::hash::*;
use crate::quotearg::*;
use crate::system::*;
use crate::tar::*;
use crate::wordsplit::*;

const NAME_FIELD_SIZE: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NeltType {
    Name,
    Chdir,
    File,
    Noop,
    Option,
}

struct NameElt {
    next: Option<Box<NameElt>>,
    prev: Option<*mut NameElt>,
    type_: NeltType,
    name: Option<String>,
    file: Option<FileElt>,
    opt: Option<OptionElt>,
}

struct FileElt {
    name: String,
    line: usize,
    term: char,
    verbatim: bool,
    fp: Option<File>,
}

struct OptionElt {
    option: i32,
    arg: Option<String>,
}

struct Name {
    name: String,
    length: usize,
    caname: Option<String>,
    next: Option<Box<Name>>,
    prev: Option<*mut Name>,
    found_count: i32,
    matching_flags: i32,
    change_dir: Option<String>,
    directory: Option<Directory>,
    parent: Option<*mut Name>,
    cmdline: bool,
    child: Option<Box<Name>>,
    sibling: Option<Box<Name>>,
}

struct Directory {
    contents: Vec<u8>,
}

static mut NAMELIST: Option<Box<Name>> = None;
static mut NAMETAIL: Option<*mut Name> = None;
static mut NAME_BUFFER: Vec<u8> = Vec::new();
static mut NAME_BUFFER_LENGTH: usize = 0;
static mut FILE_ID_LIST: Option<FileIdList> = None;
static mut UNCONSUMED_OPTION_TAIL: Option<Box<NameElt>> = None;
static mut CACHED_UNAME: Option<String> = None;
static mut CACHED_GNAME: Option<String> = None;
static mut CACHED_UID: libc::uid_t = 0;
static mut CACHED_GID: libc::gid_t = 0;
static mut CACHED_NO_SUCH_UNAME: Option<String> = None;
static mut CACHED_NO_SUCH_GNAME: Option<String> = None;
static mut CACHED_NO_SUCH_UID: libc::uid_t = 0;
static mut CACHED_NO_SUCH_GID: libc::gid_t = 0;
static mut FILENAME_TERMINATOR: char = '\n';
static mut VERBATIM_FILES_FROM_OPTION: bool = false;
static mut WILDCARDS: Wildcards = Wildcards::Default;
static mut MATCHING_FLAGS: i32 = 0;
static mut INCLUDE_ANCHORED: i32 = 0;
static mut RECURSION_OPTION: i32 = 0;
static mut UNQUOTE_OPTION: bool = false;
static mut STARTING_FILE_OPTION: bool = false;
static mut SAME_ORDER_OPTION: bool = false;
static mut VERBOSE_OPTION: bool = false;
static mut LISTED_INCREMENTAL_OPTION: bool = false;
static mut OCCURRENCE_OPTION: i32 = 0;
static mut SUBCOMMAND_OPTION: Subcommand = Subcommand::None;
static mut GNU_LIST_NAME: Option<*mut Name> = None;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wildcards {
    Default,
    Disable,
    Enable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Subcommand {
    None,
    Create,
    Update,
}

struct FileIdList {
    next: Option<Box<FileIdList>>,
    ino: libc::ino_t,
    dev: libc::dev_t,
    from_file: String,
}

fn name_add_option(option: i32, arg: Option<String>) {
    unsafe {
        let mut elt = Box::new(NameElt {
            next: None,
            prev: None,
            type_: NeltType::Option,
            name: None,
            file: None,
            opt: Some(OptionElt { option, arg }),
        });

        if NAME_HEAD.is_none() {
            NAME_HEAD = Some(Box::new(NameElt {
                next: None,
                prev: None,
                type_: NeltType::Noop,
                name: None,
                file: None,
                opt: None,
            }));
            elt.prev = Some(NAME_HEAD.as_mut().unwrap().as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().next = Some(elt);
        } else {
            elt.prev = NAME_HEAD.as_mut().unwrap().prev;
            if let Some(prev) = NAME_HEAD.as_mut().unwrap().prev {
                unsafe { (*prev).next = Some(elt) };
            }
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            elt.next = NAME_HEAD.clone();
        }
    }
}

fn name_add_dir(name: &str) {
    unsafe {
        let mut elt = Box::new(NameElt {
            next: None,
            prev: None,
            type_: NeltType::Chdir,
            name: Some(name.to_string()),
            file: None,
            opt: None,
        });

        if NAME_HEAD.is_none() {
            NAME_HEAD = Some(Box::new(NameElt {
                next: None,
                prev: None,
                type_: NeltType::Noop,
                name: None,
                file: None,
                opt: None,
            }));
            elt.prev = Some(NAME_HEAD.as_mut().unwrap().as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().next = Some(elt);
        } else {
            elt.prev = NAME_HEAD.as_mut().unwrap().prev;
            if let Some(prev) = NAME_HEAD.as_mut().unwrap().prev {
                unsafe { (*prev).next = Some(elt) };
            }
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            elt.next = NAME_HEAD.clone();
        }
    }
}

fn name_add_file(name: &str) {
    unsafe {
        let mut elt = Box::new(NameElt {
            next: None,
            prev: None,
            type_: NeltType::File,
            name: None,
            file: Some(FileElt {
                name: name.to_string(),
                line: 0,
                term: '\n',
                verbatim: false,
                fp: None,
            }),
            opt: None,
        });

        if NAME_HEAD.is_none() {
            NAME_HEAD = Some(Box::new(NameElt {
                next: None,
                prev: None,
                type_: NeltType::Noop,
                name: None,
                file: None,
                opt: None,
            }));
            elt.prev = Some(NAME_HEAD.as_mut().unwrap().as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().next = Some(elt);
        } else {
            elt.prev = NAME_HEAD.as_mut().unwrap().prev;
            if let Some(prev) = NAME_HEAD.as_mut().unwrap().prev {
                unsafe { (*prev).next = Some(elt) };
            }
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            elt.next = NAME_HEAD.clone();
        }

        FILENAME_ARGS = FilesCount::Many;
    }
}

fn name_add_name(name: &str) {
    unsafe {
        let mut elt = Box::new(NameElt {
            next: None,
            prev: None,
            type_: NeltType::Name,
            name: Some(name.to_string()),
            file: None,
            opt: None,
        });

        if NAME_HEAD.is_none() {
            NAME_HEAD = Some(Box::new(NameElt {
                next: None,
                prev: None,
                type_: NeltType::Noop,
                name: None,
                file: None,
                opt: None,
            }));
            elt.prev = Some(NAME_HEAD.as_mut().unwrap().as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            NAME_HEAD.as_mut().unwrap().next = Some(elt);
        } else {
            elt.prev = NAME_HEAD.as_mut().unwrap().prev;
            if let Some(prev) = NAME_HEAD.as_mut().unwrap().prev {
                unsafe { (*prev).next = Some(elt) };
            }
            NAME_HEAD.as_mut().unwrap().prev = Some(elt.as_mut() as *mut _);
            elt.next = NAME_HEAD.clone();
        }

        match FILENAME_ARGS {
            FilesCount::None => FILENAME_ARGS = FilesCount::One,
            FilesCount::One => FILENAME_ARGS = FilesCount::Many,
            _ => (),
        }
    }
}

fn name_init() {
    unsafe {
        NAME_BUFFER = vec![0; NAME_FIELD_SIZE + 2];
        NAME_BUFFER_LENGTH = NAME_FIELD_SIZE;
        name_list_adjust();
    }
}

fn name_term() {
    unsafe {
        NAME_BUFFER.clear();
    }
}

fn name_list_adjust() {
    unsafe {
        if let Some(mut head) = NAME_HEAD.take() {
            while let Some(prev) = head.prev {
                head = unsafe { Box::from_raw(prev) };
            }
            NAME_HEAD = Some(head);
        }
    }
}

fn add_file_id(filename: &str) -> bool {
    unsafe {
        let st = match std::fs::metadata(filename) {
            Ok(meta) => meta,
            Err(e) => {
                stat_fatal(filename);
                return true;
            }
        };

        let reading_from = file_list_name();
        let mut p = FILE_ID_LIST.as_ref();

        while let Some(entry) = p {
            if entry.ino == st.ino() && entry.dev == st.dev() {
                let oldc = set_char_quoting(None, ':', true);
                error!(
                    0,
                    "{}: file list requested from {} already read from {}",
                    quotearg_n(0, filename),
                    reading_from,
                    entry.from_file
                );
                set_char_quoting(None, ':', oldc);
                return true;
            }
            p = entry.next.as_ref();
        }

        FILE_ID_LIST = Some(Box::new(FileIdList {
            next: FILE_ID_LIST.take(),
            ino: st.ino(),
            dev: st.dev(),
            from_file: reading_from.to_string(),
        }));

        false
    }
}

fn chopslash(s: &mut String) {
    while s.ends_with('/') {
        s.pop();
    }
}

fn read_name_from_file(ent: &mut NameElt) -> ReadFileListState {
    unsafe {
        let fp = match ent.file.as_mut().unwrap().fp.as_mut() {
            Some(f) => f,
            None => return ReadFileListState::End,
        };

        let term = ent.file.as_ref().unwrap().term;
        let mut counter = 0;
        let mut buf = [0; 1];

        ent.file.as_mut().unwrap().line += 1;

        loop {
            match fp.read(&mut buf) {
                Ok(0) => break,
                Ok(1) => {
                    let c = buf[0] as char;
                    if c == term {
                        break;
                    }
                    if counter == NAME_BUFFER_LENGTH {
                        NAME_BUFFER.resize(NAME_BUFFER_LENGTH * 2, 0);
                        NAME_BUFFER_LENGTH *= 2;
                    }
                    NAME_BUFFER[counter] = c as u8;
                    counter += 1;
                    if c == '\0' {
                        return ReadFileListState::Zero;
                    }
                }
                Err(e) => {
                    error!(0, "read error: {}", e);
                    return ReadFileListState::End;
                }
            }
        }

        if counter == 0 {
            return ReadFileListState::Skip;
        }

        if counter == NAME_BUFFER_LENGTH {
            NAME_BUFFER.resize(NAME_BUFFER_LENGTH * 2, 0);
            NAME_BUFFER_LENGTH *= 2;
        }
        NAME_BUFFER[counter] = 0;
        let s = unsafe { std::str::from_utf8_unchecked(&NAME_BUFFER[..counter]) };
        let mut s = s.to_string();
        chopslash(&mut s);
        NAME_BUFFER[..s.len()].copy_from_slice(s.as_bytes());
        NAME_BUFFER[s.len()] = 0;

        if counter == 0 {
            ReadFileListState::End
        } else {
            ReadFileListState::Success
        }
    }
}

#[derive(Debug, PartialEq)]
enum ReadFileListState {
    Success,
    End,
    Zero,
    Skip,
}

fn handle_option(str: &str, ent: &NameElt) -> bool {
    let mut ws = WordSplit::new(str);
    ws.ws_offs = 1;
    if ws.split(WRDSF_DEFFLAGS | WRDSF_DOOFFS).is_err() {
        fatal!(
            "cannot split string '{}': {}",
            str,
            ws.strerror()
        );
        return false;
    }

    let mut args = vec![program_name.to_string()];
    args.extend(ws.ws_wordv[ws.ws_offs..].iter().map(|s| s.to_string()));

    let loc = OptionLocus {
        source: OptsSource::File,
        name: ent.file.as_ref().unwrap().name.clone(),
        line: ent.file.as_ref().unwrap().line,
    };

    more_options(&args, &loc);
    true
}

fn read_next_name(ent: &mut NameElt, ret: &mut NameElt) -> bool {
    unsafe {
        if ent.file.as_ref().unwrap().fp.is_none() {
            if ent.file.as_ref().unwrap().name == "-" {
                request_stdin("-T");
                ent.file.as_mut().unwrap().fp = Some(stdin);
            } else {
                if add_file_id(&ent.file.as_ref().unwrap().name) {
                    name_list_advance();
                    return true;
                }
                match File::open(&ent.file.as_ref().unwrap().name) {
                    Ok(f) => ent.file.as_mut().unwrap().fp = Some(f),
                    Err(e) => {
                        open_fatal(&ent.file.as_ref().unwrap().name);
                        return true;
                    }
                }
            }
            ent.file.as_mut().unwrap().term = FILENAME_TERMINATOR;
            ent.file.as_mut().unwrap().verbatim = VERBATIM_FILES_FROM_OPTION;
        }

        loop {
            match read_name_from_file(ent) {
                ReadFileListState::Skip => continue,
                ReadFileListState::Zero => {
                    warn!(
                        WARN_FILENAME_WITH_NULS,
                        "{}: file name read contains nul character",
                        quotearg_colon(&ent.file.as_ref().unwrap().name)
                    );
                    ent.file.as_mut().unwrap().term = '\0';
                }
                ReadFileListState::Success => {
                    if !ent.file.as_ref().unwrap().verbatim {
                        if UNQUOTE_OPTION {
                            unquote_string(&mut NAME_BUFFER);
                        }
                        let s = unsafe {
                            std::str::from_utf8_unchecked(
                                &NAME_BUFFER[..NAME_BUFFER.iter().position(|&c| c == 0).unwrap_or(NAME_BUFFER.len())]
                            )
                        };
                        if handle_option(s, ent) {
                            name_list_adjust();
                            return true;
                        }
                    }
                    ret.type_ = NeltType::Name;
                    ret.name = Some(
                        unsafe {
                            std::str::from_utf8_unchecked(
                                &NAME_BUFFER[..NAME_BUFFER.iter().position(|&c| c == 0).unwrap_or(NAME_BUFFER.len())]
                            )
                        }
                        .to_string(),
                    );
                    return false;
                }
                ReadFileListState::End => {
                    if ent.file.as_ref().unwrap().name != "-" {
                        ent.file.as_mut().unwrap().fp = None;
                    }
                    name_list_advance();
                    return true;
                }
            }
        }
    }
