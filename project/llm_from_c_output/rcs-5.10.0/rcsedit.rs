/*
 * RCS stream editor - Rust translation
 *
 * Original C code copyright holders:
 * Copyright (C) 2010-2020 Thien-Thi Nguyen
 * Copyright (C) 1990, 1991, 1992, 1993, 1994, 1995 Paul Eggert
 * Copyright (C) 1982, 1988, 1989 Walter Tichy
 *
 * This file is part of GNU RCS.
 *
 * GNU RCS is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * GNU RCS is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty
 * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 */

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write, Seek, SeekFrom},
    path::{Path, PathBuf},
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStrExt,
    time::{SystemTime, UNIX_EPOCH},
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
};

const SDELIM: char = '@';
const _POSIX_SYMLOOP_MAX: usize = 8;

struct EditStuff {
    fedit: Option<File>,
    filename: PathBuf,
    script_lno: usize,
    lcount: i64,
    corr: i64,
    lines: Vec<String>,
    gap: usize,
    gapsize: usize,
    lim: usize,
    sff: Sff,
}

struct Sff {
    lockdir: PathBuf,
    newdir: PathBuf,
}

impl EditStuff {
    fn new() -> Self {
        EditStuff {
            fedit: None,
            filename: PathBuf::new(),
            script_lno: 0,
            lcount: 0,
            corr: 0,
            lines: Vec::new(),
            gap: 0,
            gapsize: 0,
            lim: 0,
            sff: Sff::new(),
        }
    }

    fn unmake(&mut self) {
        self.lines.clear();
        *self = EditStuff::new();
    }
}

impl Sff {
    fn new() -> Self {
        Sff {
            lockdir: PathBuf::new(),
            newdir: PathBuf::new(),
        }
    }
}

struct DiffCmd {
    adprev: i64,
    dafter: i64,
    line1: i64,
    nlines: i64,
}

impl DiffCmd {
    fn new() -> Self {
        DiffCmd {
            adprev: 0,
            dafter: 0,
            line1: 0,
            nlines: 0,
        }
    }
}

fn nfs_noent_p() -> bool {
    // TODO: Implement NFS check
    false
}

fn unlink_file(path: &Path) -> io::Result<()> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) if nfs_noent_p() => Ok(()),
        Err(e) => Err(e),
    }
}

fn insert_line(es: &mut EditStuff, n: usize, line: String) -> io::Result<()> {
    if es.lim - es.gapsize < n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "edit script refers to line past end of file",
        ));
    }

    if es.gapsize == 0 {
        if es.lim == 0 {
            es.lines = Vec::with_capacity(1024);
            es.lim = 1024;
            es.gapsize = 1024;
        } else {
            es.gap = es.gapsize = es.lim;
            es.lim *= 2;
            es.lines.reserve(es.lim);
        }
    }

    if n < es.gap {
        let to_move = es.gap - n;
        es.lines.copy_within(n..es.gap, n + es.gapsize);
    } else if es.gap < n {
        let to_move = n - es.gap;
        es.lines.copy_within(es.gap + es.gapsize..n, es.gap);
    }

    es.lines[n] = line;
    es.gap = n + 1;
    es.gapsize -= 1;

    Ok(())
}

fn delete_lines(es: &mut EditStuff, n: usize, nlines: usize) -> io::Result<()> {
    let l = n + nlines;
    if es.lim - es.gapsize < l || l < n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "edit script refers to line past end of file",
        ));
    }

    if l < es.gap {
        let to_move = es.gap - l;
        es.lines.copy_within(l..es.gap, l + es.gapsize);
    } else if es.gap < n {
        let to_move = n - es.gap;
        es.lines.copy_within(es.gap + es.gapsize..n, es.gap);
    }

    es.gap = n;
    es.gapsize += nlines;

    Ok(())
}

fn snapshot_line(f: &mut File, line: &str) -> io::Result<()> {
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == SDELIM {
            if chars.next() != Some(SDELIM) {
                return Ok(());
            }
        }
        write!(f, "{}", c)?;
    }
    Ok(())
}

fn snapshot_edit_fast(es: &EditStuff, f: &mut File) -> io::Result<()> {
    for line in es.lines[..es.gap].iter() {
        snapshot_line(f, line)?;
    }
    for line in es.lines[es.gap + es.gapsize..].iter() {
        snapshot_line(f, line)?;
    }
    Ok(())
}

struct FinCtx {
    // TODO: Implement expansion context
    script_lno: usize,
}

fn finish_edit_line(_finctx: &FinCtx, _line: &str) -> io::Result<()> {
    // TODO: Implement keyword expansion
    Ok(())
}

fn finish_edit_fast(
    es: &EditStuff,
    _delta: Option<()>, // TODO: Replace with actual delta type
    outfile: &mut File,
    done: bool,
) -> io::Result<()> {
    if done {
        if _delta.is_none() {
            snapshot_edit_fast(es, outfile)?;
        } else {
            // TODO: Implement delta processing
            for line in es.lines[..es.gap].iter() {
                finish_edit_line(&FinCtx { script_lno: es.script_lno }, line)?;
            }
            for line in es.lines[es.gap + es.gapsize..].iter() {
                finish_edit_line(&FinCtx { script_lno: es.script_lno }, line)?;
            }
        }
    }
    Ok(())
}

fn fopen_update_truncate(name: &Path) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
}

fn swap_edit_files(es: &mut EditStuff, outfile: Option<File>) -> io::Result<()> {
    es.lcount = 0;
    es.corr = 0;

    if es.fedit.is_none() {
        es.fedit = Some(File::open(&es.filename)?);
    }

    if let Some(mut f) = es.fedit.take() {
        f.seek(SeekFrom::Start(0))?;
        es.fedit = Some(f);
    }

    // TODO: Implement temp file swapping logic
    Ok(())
}

fn finish_edit_slow(
    es: &mut EditStuff,
    _delta: Option<()>, // TODO: Replace with actual delta type
    outfile: Option<File>,
    done: bool,
) -> io::Result<()> {
    if let Some(mut fe) = es.fedit.take() {
        if let Some(mut fc) = outfile {
            // TODO: Implement delta processing
            let mut buf = String::new();
            fe.read_to_string(&mut buf)?;
            fc.write_all(buf.as_bytes())?;
        }
    }

    if !done {
        swap_edit_files(es, outfile)?;
    }

    Ok(())
}

fn snapshot_edit_slow(es: &mut EditStuff, f: &mut File) -> io::Result<()> {
    finish_edit_slow(es, None, None, false)?;
    if let Some(mut fe) = es.fedit.as_mut() {
        let mut buf = String::new();
        fe.read_to_string(&mut buf)?;
        f.write_all(buf.as_bytes())?;
        fe.seek(SeekFrom::Start(0))?;
    }
    Ok(())
}

fn finish_edit(
    es: &mut EditStuff,
    delta: Option<()>, // TODO: Replace with actual delta type
    outfile: Option<File>,
    done: bool,
) -> io::Result<()> {
    // TODO: Implement STDIO_P check
    finish_edit_fast(es, delta, &mut outfile.unwrap(), done)
}

fn snapshot_edit(es: &mut EditStuff, f: &mut File) -> io::Result<()> {
    // TODO: Implement STDIO_P check
    snapshot_edit_fast(es, f)
}

fn copy_lines(
    es: &mut EditStuff,
    upto: i64,
    _delta: Option<()>, // TODO: Replace with actual delta type
) -> io::Result<()> {
    // TODO: Implement full copy_lines functionality
    es.lcount = upto;
    Ok(())
}

struct AtAt {
    line_count: i64,
    // TODO: Add other necessary fields
}

fn copy_string(es: &mut EditStuff, atat: &AtAt) -> io::Result<()> {
    // TODO: Implement string copying
    es.lcount += atat.line_count;
    Ok(())
}

fn enter_string(es: &mut EditStuff, atat: &AtAt) -> io::Result<()> {
    // TODO: Implement STDIO_P check
    copy_string(es, atat)
}

fn edit_string(
    es: &mut EditStuff,
    _script: &AtAt, // TODO: Replace with actual script type
    _delta: Option<()>, // TODO: Replace with actual delta type
) -> io::Result<()> {
    // TODO: Implement full edit_string functionality
    Ok(())
}

fn naturalize(_m: &mut ()) -> io::Result<(bool, bool)> {
    // TODO: Implement symlink resolution
    Ok((false, false))
}

fn rcs_write_open(_m: &mut ()) -> io::Result<File> {
    // TODO: Implement lock file creation and opening
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}

fn chname_mod(
    from: &Path,
    to: &Path,
    set_mode: bool,
    mode: u32,
    mtime: SystemTime,
) -> io::Result<()> {
    // TODO: Implement full file renaming with mode and time changes
    std::fs::rename(from, to)
}

fn set_mtime(file: &Path, mtime: SystemTime) -> io::Result<()> {
    if mtime == UNIX_EPOCH {
        return Ok(());
    }
    // TODO: Implement proper mtime setting
    Ok(())
}

fn find_lock(delete: bool) -> io::Result<Option<()>> {
    // TODO: Implement lock finding
    Ok(None)
}

fn add_symbol(num: &str, name: &str, rebind: bool) -> io::Result<bool> {
    // TODO: Implement symbol table management
    Ok(false)
}

fn check_access_list() -> bool {
    // TODO: Implement access list checking
    true
}

fn do_rewrite(lockflag: bool, changed: i32) -> io::Result<()> {
    // TODO: Implement rewrite preparation
    Ok(())
}

fn done_rewrite(changed: i32, mtime: SystemTime) -> io::Result<()> {
    // TODO: Implement rewrite completion
    Ok(())
}

fn orcs_close() -> io::Result<()> {
    // TODO: Implement cleanup
    Ok(())
}

fn orcs_error() {
    // TODO: Implement error cleanup
}

fn unexpected_eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF in diff output")
}

fn init_diff_cmd(dc: &mut DiffCmd) {
    *dc = DiffCmd::new();
}

fn bad_diff_output(buf: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("bad diff output line: {}", buf))
}

fn diff_line_number_too_large(buf: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("diff line number too large: {}", buf),
    )
}

fn get_diff_cmd(
    fin: &mut File,
    delimiter: bool,
    fout: Option<&mut File>,
    dc: &mut DiffCmd,
) -> io::Result<i32> {
    let mut buf = String::new();
    fin.read_line(&mut buf)?;

    if delimiter && buf.starts_with(SDELIM) {
        if buf.len() > 1 && buf.chars().nth(1) == Some(SDELIM) {
            return Err(bad_diff_output(&buf));
        }
        if let Some(f) = fout {
            write!(f, "{}", buf)?;
        }
        return Ok(-1);
    }

    let mut parts = buf.split_whitespace();
    let cmd = parts.next().ok_or_else(|| bad_diff_output(&buf))?;
    let line1 = parts.next().and_then(|s| s.parse().ok()).ok_or_else(|| bad_diff_output(&buf))?;
    let nlines = parts.next().and_then(|s| s.parse().ok()).ok_or_else(|| bad_diff_output(&buf))?;

    if line1 + nlines < line1 {
        return Err(diff_line_number_too_large(&buf));
    }

    match cmd {
        "a" => {
            if line1 < dc.adprev {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("backward insertion in diff output: {}", buf),
                ));
            }
            dc.adprev = line1 + 1;
        }
        "d" => {
            if line1 < dc.adprev || line1 < dc.dafter {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("backward deletion in diff output: {}", buf),
                ));
            }
            dc.adprev = line1;
            dc.dafter = line1 + nlines;
        }
        _ => return Err(bad_diff_output(&buf)),
    }

    if let Some(f) = fout {
        write!(f, "{}", buf)?;
    }

    dc.line1 = line1;
    dc.nlines = nlines;

    Ok(if cmd == "a" { 1 } else { 0 })
}