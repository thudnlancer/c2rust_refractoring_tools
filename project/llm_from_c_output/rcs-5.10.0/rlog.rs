/*!
Print log messages and other information about RCS files.

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

use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{self, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
    ptr,
    str,
};

const KS_DELIMS: &str = ", \t\n;";

struct RevRange {
    beg: *const libc::c_char,
    end: *const libc::c_char,
    nfield: libc::c_int,
}

struct DateRange {
    beg: [libc::c_char; DATESIZE],
    end: [libc::c_char; DATESIZE],
    open_end: bool,
}

struct DateSelection {
    in_: *mut Link,
    by: *mut Link,
}

struct Criteria {
    revs: *mut Link,
    actual: *mut Link,
    authors: *mut Link,
    lockers: *mut Link,
    states: *mut Link,
}

struct Link {
    entry: *mut libc::c_void,
    next: *mut Link,
}

struct Delta {
    num: *const libc::c_char,
    date: *const libc::c_char,
    author: *const libc::c_char,
    state: *const libc::c_char,
    lockedby: *const libc::c_char,
    text: *mut AtAt,
    branches: *mut WLink,
    ilk: *mut Delta,
    selector: bool,
    pretty_log: Cbuf,
    commitid: *const libc::c_char,
    log: *mut AtAt,
}

struct WLink {
    entry: *mut Delta,
    next: *mut WLink,
}

struct AtAt {
    beg: *const libc::c_char,
    end: *const libc::c_char,
}

struct Cbuf {
    string: *const libc::c_char,
    size: libc::size_t,
}

const DATESIZE: usize = 24;
const FULLDATESIZE: usize = 32;
const PLEXUS: i32 = 0;
const SINGLE: i32 = 1;
const EMPTYLOG: &str = "\n";

fn push<T>(x: *mut T, ls: *mut *mut Link, plexus: i32) -> *mut Link {
    unsafe {
        let new_link = libc::malloc(std::mem::size_of::<Link>()) as *mut Link;
        (*new_link).entry = x as *mut libc::c_void;
        (*new_link).next = *ls;
        *ls = new_link;
        new_link
    }
}

fn tokenize(argv: *mut libc::c_char, chain: *mut *mut Link) -> bool {
    unsafe {
        let before = *chain;
        let mut save = ptr::null_mut();
        let mut s = argv;

        while !(*s).is_null() {
            let token = libc::strtok_r(s, KS_DELIMS.as_ptr() as *const libc::c_char, &mut save);
            if token.is_null() {
                break;
            }
            push(token as *mut libc::c_char, chain, PLEXUS);
            s = ptr::null_mut();
        }

        *chain != before
    }
}

fn cleanup(exitstatus: &mut libc::c_int) {
    unsafe {
        if FLOW.erroneous {
            *exitstatus = exit_failure;
        }
        fro_zclose(&mut FLOW.from);
    }
}

fn getlocker(argv: *mut libc::c_char, criteria: &mut Criteria) {
    tokenize(argv, &mut criteria.lockers);
}

fn read_positive_integer(p: &mut *const libc::c_char) -> libc::c_long {
    unsafe {
        let mut end = ptr::null_mut();
        libc::errno = 0;
        let rv = libc::strtol(*p, &mut end, 10);
        if rv < 1 {
            RFATAL(b"non-positive integer\0".as_ptr() as *const libc::c_char);
        }
        if libc::errno == libc::ERANGE {
            RFATAL(b"bad integer\0".as_ptr() as *const libc::c_char);
        }
        *p = end;
        rv
    }
}

fn count_a_d(a: &mut libc::c_long, d: &mut libc::c_long, edits: *mut AtAt) {
    unsafe {
        let s = string_from_atat(SINGLE, edits);
        let end = s.string.offset(s.size as isize);
        let totals = zlloc(SINGLE, 2 * std::mem::size_of::<libc::c_long>()) as *mut libc::c_long;

        let mut p = s.string;
        while p < end {
            let add = *p == b'a' as libc::c_char;
            p = p.offset(1);

            p = libc::strchr(p, b' ' as libc::c_int);
            if p.is_null() {
                break;
            }
            p = p.offset(1);

            let mut count = read_positive_integer(&mut p);

            *totals.offset(add as isize) += count;
            if add {
                while count > 0 {
                    let remaining = end.offset_from(p) as usize;
                    p = libc::memchr(p as *const libc::c_void, b'\n' as libc::c_int, remaining) as *const libc::c_char;
                    if p.is_null() {
                        break;
                    }
                    p = p.offset(1);
                    count -= 1;
                }
            }
        }

        *a = *totals.offset(1);
        *d = *totals.offset(0);
        brush_off(SINGLE, totals as *mut libc::c_void);
    }
}

fn putadelta(node: *const Delta, editscript: *const Delta, ins_del_format: *const libc::c_char) {
    unsafe {
        let out = libc::stdout;
        let mut datebuf = [0; FULLDATESIZE];
        let pre5 = BE.version < VERSION(5);
        let log = (*node).log;

        aprintf(
            out,
            b"----------------------------\nrevision %s%s\0".as_ptr() as *const libc::c_char,
            (*node).num,
            if pre5 {
                b"        \0".as_ptr() as *const libc::c_char
            } else {
                b"\0".as_ptr() as *const libc::c_char
            },
        );
        if !(*node).lockedby.is_null() {
            aprintf(
                out,
                if pre5 {
                    b"\tlocked by: %s;\0".as_ptr() as *const libc::c_char
                } else {
                    b"locked by: %s;\0".as_ptr() as *const libc::c_char
                },
                (*node).lockedby,
            );
        }

        aprintf(
            out,
            b"\ndate: %s;  author: %s;  state: %s;\0".as_ptr() as *const libc::c_char,
            date2str((*node).date, datebuf.as_mut_ptr()),
            (*node).author,
            (*node).state,
        );

        if !editscript.is_null() && editscript != REPO.tip {
            let trunk = node != editscript;
            let mut a = 0;
            let mut d = 0;

            count_a_d(
                if trunk { &mut d } else { &mut a },
                if trunk { &mut a } else { &mut d },
                (*editscript).text,
            );
            aprintf(out, ins_del_format, a, d);
        }

        if !(*node).branches.is_null() {
            aputs(b"\nbranches:\0".as_ptr() as *const libc::c_char, out);
            let mut ls = (*node).branches;
            while !ls.is_null() {
                let delta = (*ls).entry;
                aprintf(
                    out,
                    b"  %s;\0".as_ptr() as *const libc::c_char,
                    BRANCHNO((*delta).num),
                );
                ls = (*ls).next;
            }
        }

        if !(*node).commitid.is_null() {
            aprintf(
                out,
                if !editscript.is_null() {
                    b"; commitid: %s\0".as_ptr() as *const libc::c_char
                } else {
                    b" commitid: %s\0".as_ptr() as *const libc::c_char
                },
                (*node).commitid,
            );
        }

        newline(out);
        if !log.is_null() && (*log).beg.offset(1) < ATAT_END(log) {
            atat_display(out, log, true);
        } else {
            awrite(
                EMPTYLOG.as_ptr() as *const libc::c_char,
                EMPTYLOG.len(),
                out,
            );
        }
    }
}

fn putrunk(ins_del_format: *const libc::c_char) {
    unsafe {
        let mut ptr = REPO.tip;
        while !ptr.is_null() {
            if (*ptr).selector {
                putadelta(ptr, (*ptr).ilk, ins_del_format);
            }
            ptr = (*ptr).ilk;
        }
    }
}

fn putforest(branchroot: *const WLink, ins_del_format: *const libc::c_char) -> *const Delta {
    unsafe {
        if !(*branchroot).next.is_null() {
            putforest((*branchroot).next, ins_del_format);
        }

        putabranch((*branchroot).entry, ins_del_format);
        (*branchroot).entry
    }
}

fn putabranch(root: *const Delta, ins_del_format: *const libc::c_char) {
    unsafe {
        let mut current = root;
        while !current.is_null() {
            if (*current).selector {
                break;
            }
            current = (*current).ilk;
        }
        if current.is_null() {
            return;
        }

        if !(*current).ilk.is_null() {
            putabranch((*current).ilk, ins_del_format);
        }

        putadelta(current, current, ins_del_format);
    }
}

fn putree(root: *const Delta, ins_del_format: *const libc::c_char) {
    unsafe {
        let mut current = root;
        while !current.is_null() {
            if (*current).branches.is_null() {
                current = (*current).ilk;
            } else {
                putree((*current).ilk, ins_del_format);
                current = putforest((*current).branches, ins_del_format);
            }
        }
    }
}

fn extractdelta(
    pdelta: *const Delta,
    lockflag: bool,
    criteria: *const Criteria,
) -> bool {
    unsafe {
        let mut pauthor = (*criteria).authors;
        while !pauthor.is_null() {
            if STR_DIFF((*pauthor).entry, (*pdelta).author) {
                pauthor = (*pauthor).next;
                if pauthor.is_null() {
                    return false;
                }
            } else {
                break;
            }
        }

        let mut pstate = (*criteria).states;
        while !pstate.is_null() {
            if STR_DIFF((*pstate).entry, (*pdelta).state) {
                pstate = (*pstate).next;
                if pstate.is_null() {
                    return false;
                }
            } else {
                break;
            }
        }

        if lockflag && lock_on(pdelta) == 0 {
            return false;
        }

        let mut ls = (*criteria).actual;
        while !ls.is_null() {
            let rr = (*ls).entry as *const RevRange;
            let length = (*rr).nfield;
            if countnumflds((*pdelta).num) == length + ODDP(length)
                && compartial((*pdelta).num, (*rr).beg, length) >= 0
                && compartial((*rr).end, (*pdelta).num, length) >= 0
            {
                break;
            }
            ls = (*ls).next;
            if ls.is_null() {
                return false;
            }
        }

        true
    }
}

fn exttree(root: *mut Delta, lockflag: bool, criteria: *mut Criteria) {
    unsafe {
        let mut current = root;
        while !current.is_null() {
            (*current).selector = extractdelta(current, lockflag, criteria);
            (*current).pretty_log.string = ptr::null();

            if (*current).branches.is_null() {
                current = (*current).ilk;
            } else {
                exttree((*current).ilk, lockflag, criteria);
                let mut ls = (*current).branches;
                while !(*ls).next.is_null() {
                    exttree((*ls).entry, lockflag, criteria);
                    ls = (*ls).next;
                }
                current = (*ls).entry;
            }
        }
    }
}

fn getauthor(argv: *mut libc::c_char, criteria: *mut Criteria) {
    unsafe {
        if !tokenize(argv, &mut (*criteria).authors) {
            push(
                getusername(false) as *mut libc::c_char,
                &mut (*criteria).authors,
                PLEXUS,
            );
        }
    }
}

fn getstate(argv: *mut libc::c_char, criteria: *mut Criteria) {
    unsafe {
        if !tokenize(argv, &mut (*criteria).states) {
            PERR(b"missing state attributes after -s option\0".as_ptr() as *const libc::c_char);
        }
    }
}

fn trunclocks(criteria: *mut Criteria) {
    unsafe {
        if (*criteria).lockers.is_null() {
            return;
        }

        let mut box_ = Link {
            entry: ptr::null_mut(),
            next: GROK.locks,
        };
        let mut tp = &mut box_;

        while !(*tp).next.is_null() {
            let rl = (*(*tp).next).entry as *const RcsLock;
            let mut plocker = (*criteria).lockers;

            loop {
                if STR_SAME((*plocker).entry, (*rl).login) {
                    tp = &mut *(*tp).next;
                    break;
                } else if (*plocker).next.is_null() {
                    (*tp).next = (*(*tp).next).next;
                    GROK.locks = box_.next;
                    break;
                } else {
                    plocker = (*plocker).next;
                }
            }
        }
    }
}

fn recentdate(root: *const Delta, r: *mut DateRange) {
    unsafe {
        let mut current = root;
        while !current.is_null() {
            if (*current).selector
                && !DATE_LT((*current).date, (*r).beg.as_ptr())
                && !DATE_GT((*current).date, (*r).end.as_ptr())
            {
                libc::strncpy(
                    (*r).beg.as_mut_ptr(),
                    (*current).date,
                    DATESIZE - 1,
                );
                (*r).beg[DATESIZE - 1] = 0;
            }

            let mut ls = (*current).branches;
            if ls.is_null() {
                current = (*current).ilk;
            } else {
                while !(*ls).next.is_null() {
                    recentdate((*ls).entry, r);
                    ls = (*ls).next;
                }
                current = (*ls).entry;
            }
        }
    }
}

fn extdate(root: *mut Delta, datesel: *mut DateSelection) -> libc::size_t {
    unsafe {
        let mut revno = 0;
        let mut current = root;

        while !current.is_null() {
            if !(*datesel).in_.is_null() || !(*datesel).by.is_null() {
                let mut sel = false;
                let mut ls = (*datesel).in_;

                while !ls.is_null() {
                    let r = (*ls).entry as *const DateRange;
                    let open_end = (*r).open_end;
                    if ((*r).beg[0] == 0
                        || (open_end
                            ? DATE_LT((*r).beg.as_ptr(), (*current).date)
                            : DATE_GT((*r).beg.as_ptr(), (*current).date) == 0))
                        && ((*r).end[0] == 0
                            || (open_end
                                ? DATE_LT((*current).date, (*r).end.as_ptr())
                                : DATE_GT((*current).date, (*r).end.as_ptr()) == 0))
                    {
                        sel = true;
                        break;
                    }
                    ls = (*ls).next;
                }

                if !sel {
                    ls = (*datesel).by;
                    while !ls.is_null() {
                        let r = (*ls).entry as *const DateRange;
                        if DATE_EQ((*current).date, (*r).beg.as_ptr()) {
                            sel = true;
                            break;
                        }
                        ls = (*ls).next;
                    }
                    if !sel {
                        (*current).selector = false;
                    }
                }
            }

            revno += (*current).selector as libc