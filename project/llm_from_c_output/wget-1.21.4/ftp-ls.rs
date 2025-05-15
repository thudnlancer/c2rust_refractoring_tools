use std::{
    ffi::CStr,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    mem,
    os::unix::ffi::OsStrExt,
    path::Path,
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

use libc::{c_char, c_int, c_void, size_t, strcmp, strerror, strtok, time, time_t, tm};

const FT_PLAINFILE: c_int = 0;
const FT_DIRECTORY: c_int = 1;
const FT_SYMLINK: c_int = 2;
const FT_UNKNOWN: c_int = 3;

const TT_DAY: c_int = 0;
const TT_HOUR_MIN: c_int = 1;

const ST_UNIX: c_int = 0;
const ST_WINNT: c_int = 1;
const ST_VMS: c_int = 2;
const ST_MACOS: c_int = 3;

const LOG_NOTQUIET: c_int = 0;
const FTPOK: c_int = 0;
const FOPENERR: c_int = 1;

struct FileInfo {
    name: *mut c_char,
    linkto: *mut c_char,
    size: i64,
    perms: c_int,
    tstamp: time_t,
    ptype: c_int,
    type_: c_int,
    prev: *mut FileInfo,
    next: *mut FileInfo,
}

struct Url {
    host: *mut c_char,
    port: c_int,
    dir: *mut c_char,
    user: *mut c_char,
    passwd: *mut c_char,
}

extern "C" {
    fn xmalloc(size: size_t) -> *mut c_void;
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn xfree(ptr: *mut c_void);
    fn logprintf(level: c_int, fmt: *const c_char, ...);
    fn html_quote_string(s: *const c_char) -> *mut c_char;
    fn url_escape(s: *const c_char) -> *mut c_char;
    fn url_escape_unsafe_and_reserved(s: *const c_char) -> *mut c_char;
    fn concat_strings(s1: *const c_char, ...) -> *mut c_char;
    fn number_to_static_string(n: i64) -> *mut c_char;
    fn str_to_wgint(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn c_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn c_strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn c_isdigit(c: c_int) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn localtime(timer: *const time_t) -> *mut tm;
    fn mktime(tm: *mut tm) -> time_t;
    fn strptime(
        s: *const c_char,
        format: *const c_char,
        tm: *mut tm,
    ) -> *mut c_char;
}

static mut output_stream: *mut FILE = ptr::null_mut();

#[no_mangle]
pub extern "C" fn symperms(s: *const c_char) -> c_int {
    let mut perms = 0;
    let mut i = 0;

    if unsafe { strlen(s) } < 9 {
        return 0;
    }

    let mut s_ptr = s;
    for _ in 0..3 {
        perms <<= 3;
        unsafe {
            perms += (((*s_ptr == b'r' as c_int) << 2)
                + ((*s_ptr.add(1) == b'w' as c_int) << 1)
                + (*s_ptr.add(2) == b'x' as c_int || *s_ptr.add(2) == b's' as c_int);
            s_ptr = s_ptr.add(3);
        }
    }

    perms
}

#[no_mangle]
pub extern "C" fn clean_line(line: *mut c_char, mut len: c_int) -> c_int {
    if len <= 0 {
        return 0;
    }

    unsafe {
        while len > 0 && (*line.add((len - 1) as usize) == b'\n' as c_char
            || *line.add((len - 1) as usize) == b'\r' as c_char)
        {
            *line.add((len - 1) as usize) = b'\0' as c_char;
            len -= 1;
        }
    }

    if len == 0 {
        return 0;
    }

    unsafe {
        let mut line_ptr = line;
        while *line_ptr != b'\0' as c_char {
            if *line_ptr == b'\t' as c_char {
                *line_ptr = b' ' as c_char;
            }
            line_ptr = line_ptr.add(1);
        }
    }

    len
}

#[no_mangle]
pub extern "C" fn ftp_parse_unix_ls(fp: *mut FILE, ignore_perms: c_int) -> *mut FileInfo {
    static MONTHS: [&[u8]; 12] = [
        b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun",
        b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec",
    ];

    let mut dir: *mut FileInfo = ptr::null_mut();
    let mut l: *mut FileInfo = ptr::null_mut();
    let mut cur = FileInfo {
        name: ptr::null_mut(),
        linkto: ptr::null_mut(),
        size: 0,
        perms: 0,
        tstamp: 0,
        ptype: TT_DAY,
        type_: FT_UNKNOWN,
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    };

    let mut line: *mut c_char = ptr::null_mut();
    let mut bufsize: size_t = 0;

    unsafe {
        while getline(&mut line, &mut bufsize, fp) > 0 {
            let mut len = clean_line(line, len);
            if c_strncasecmp(line, b"total\0".as_ptr() as *const c_char, 5) == 0 {
                continue;
            }

            let mut tok = strtok(line, b" \0".as_ptr() as *const c_char);
            if tok.is_null() {
                continue;
            }

            cur.name = ptr::null_mut();
            cur.linkto = ptr::null_mut();

            match *tok {
                b'-' => cur.type_ = FT_PLAINFILE,
                b'd' => cur.type_ = FT_DIRECTORY,
                b'l' => cur.type_ = FT_SYMLINK,
                _ => cur.type_ = FT_UNKNOWN,
            }

            if ignore_perms != 0 {
                cur.perms = match cur.type_ {
                    FT_PLAINFILE => 0o644,
                    FT_DIRECTORY => 0o755,
                    _ => 0o644,
                };
            } else {
                cur.perms = symperms(tok.add(1));
            }

            let mut error = 0;
            let mut ignore = 0;
            let mut year = 0;
            let mut month = 0;
            let mut day = 0;
            let mut hour = 0;
            let mut min = 0;
            let mut sec = 0;
            let mut ptype = TT_DAY;
            let mut next = -1;

            let mut tok = line;
            let mut ptok = ptr::null_mut();

            loop {
                ptok = tok;
                tok = strtok(ptr::null_mut(), b" \0".as_ptr() as *const c_char);
                if tok.is_null() {
                    break;
                }

                next -= 1;
                if next < 0 {
                    let mut i = 0;
                    while i < 12 {
                        if c_strcasecmp(tok, MONTHS[i].as_ptr() as *const c_char) == 0 {
                            break;
                        }
                        i += 1;
                    }

                    if i != 12 {
                        let mut size: i64;
                        if ptok == line {
                            error = 1;
                            break;
                        }

                        errno = 0;
                        size = str_to_wgint(ptok, ptr::null_mut(), 10);
                        if size == WGINT_MAX && errno == ERANGE {
                            cur.size = 0;
                        } else {
                            cur.size = size;
                        }

                        month = i;
                        next = 5;
                    }
                } else if next == 4 {
                    if *tok.add(1) != b'\0' as c_char {
                        day = 10 * (*tok - b'0' as c_int) + *tok.add(1) - b'0' as c_int;
                    } else {
                        day = *tok - b'0' as c_int;
                    }
                } else if next == 3 {
                    year = 0;
                    min = 0;
                    hour = 0;
                    sec = 0;

                    if c_isdigit(*tok as c_int) != 0 {
                        let mut tok_ptr = tok;
                        while c_isdigit(*tok_ptr as c_int) != 0 && year <= 99999 {
                            year = (*tok_ptr - b'0' as c_int) + 10 * year;
                            tok_ptr = tok_ptr.add(1);
                        }

                        if *tok_ptr == b':' as c_char {
                            let mut n = 0;
                            hour = year;
                            year = 0;
                            ptype = TT_HOUR_MIN;
                            tok_ptr = tok_ptr.add(1);

                            while c_isdigit(*tok_ptr as c_int) != 0 && n < 2 {
                                min = (*tok_ptr - b'0' as c_int) + 10 * min;
                                tok_ptr = tok_ptr.add(1);
                                n += 1;
                            }

                            if *tok_ptr == b':' as c_char {
                                tok_ptr = tok_ptr.add(1);
                                n = 0;
                                while c_isdigit(*tok_ptr as c_int) != 0 && n < 2 {
                                    sec = (*tok_ptr - b'0' as c_int) + 10 * sec;
                                    tok_ptr = tok_ptr.add(1);
                                    n += 1;
                                }
                            }
                        }
                    }
                } else if next == 2 {
                    let mut fnlen = strlen(tok);
                    let mut p = ptr::null_mut();

                    if fnlen < len - (tok - line) {
                        *tok.add(fnlen) = b' ' as c_char;
                        if cur.type_ == FT_SYMLINK {
                            p = strstr(tok, b" -> \0".as_ptr() as *const c_char);
                            if p.is_null() {
                                error = 1;
                                break;
                            }
                            cur.linkto = xstrdup(p.add(4));
                            *p = b'\0' as c_char;
                        }
                    }

                    if strcmp(tok, b".\0".as_ptr() as *const c_char) == 0
                        || strcmp(tok, b"..\0".as_ptr() as *const c_char) == 0
                    {
                        ignore = 1;
                        break;
                    }

                    fnlen = strlen(tok);
                    cur.name = xmalloc(fnlen + 1);
                    memcpy(cur.name as *mut c_void, tok as *const c_void, fnlen + 1);

                    if fnlen != 0 {
                        if cur.type_ == FT_DIRECTORY && *cur.name.add(fnlen - 1) == b'/' as c_char {
                            *cur.name.add(fnlen - 1) = b'\0' as c_char;
                        } else if cur.type_ == FT_SYMLINK && *cur.name.add(fnlen - 1) == b'@' as c_char {
                            *cur.name.add(fnlen - 1) = b'\0' as c_char;
                        } else if cur.type_ == FT_PLAINFILE
                            && (cur.perms & 0o111) != 0
                            && *cur.name.add(fnlen - 1) == b'*' as c_char
                        {
                            *cur.name.add(fnlen - 1) = b'\0' as c_char;
                        }
                    } else {
                        error = 1;
                    }
                    break;
                } else {
                    abort();
                }
            }

            if cur.name.is_null() || (cur.type_ == FT_SYMLINK && cur.linkto.is_null()) {
                error = 1;
            }

            if error != 0 || ignore != 0 {
                xfree(cur.name as *mut c_void);
                xfree(cur.linkto as *mut c_void);
                continue;
            }

            let timenow = time(ptr::null_mut());
            let tnow = localtime(&timenow);

            let mut timestruct = tm {
                tm_sec: sec,
                tm_min: min,
                tm_hour: hour,
                tm_mday: day,
                tm_mon: month,
                tm_year: if year == 0 {
                    if month > (*tnow).tm_mon {
                        (*tnow).tm_year - 1
                    } else {
                        (*tnow).tm_year
                    }
                } else {
                    if year >= 1900 {
                        year - 1900
                    } else {
                        year
                    }
                },
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: -1,
                tm_gmtoff: 0,
                tm_zone: ptr::null_mut(),
            };

            if dir.is_null() {
                l = dir = xmalloc(mem::size_of::<FileInfo>()) as *mut FileInfo;
                memcpy(dir as *mut c_void, &cur as *const _ as *const c_void, mem::size_of::<FileInfo>());
                (*dir).prev = ptr::null_mut();
                (*dir).next = ptr::null_mut();
            } else {
                cur.prev = l;
                (*l).next = xmalloc(mem::size_of::<FileInfo>()) as *mut FileInfo;
                l = (*l).next;
                memcpy(l as *mut c_void, &cur as *const _ as *const c_void, mem::size_of::<FileInfo>());
                (*l).next = ptr::null_mut();
            }

            (*l).tstamp = mktime(&mut timestruct);
            (*l).ptype = ptype;
        }

        xfree(line as *mut c_void);
        dir
    }
}

#[no_mangle]
pub extern "C" fn ftp_parse_winnt_ls(fp: *mut FILE) -> *mut FileInfo {
    let mut dir: *mut FileInfo = ptr::null_mut();
    let mut l: *mut FileInfo = ptr::null_mut();
    let mut cur = FileInfo {
        name: ptr::null_mut(),
        linkto: ptr::null_mut(),
        size: 0,
        perms: 0,
        tstamp: 0,
        ptype: TT_HOUR_MIN,
        type_: FT_UNKNOWN,
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    };

    let mut line: *mut c_char = ptr::null_mut();
    let mut bufsize: size_t = 0;

    unsafe {
        while getline(&mut line, &mut bufsize, fp) > 0 {
            let len = clean_line(line, len);
            if len < 40 {
                continue;
            }

            let mut filename = line.add(39);

            let mut tok = strtok(line, b"-\0".as_ptr() as *const c_char);
            if tok.is_null() {
                continue;
            }

            let mut month = atoi(tok) - 1;
            if month < 0 {
                month = 0;
            }

            tok = strtok(ptr::null_mut(), b"-\0".as_ptr() as *const c_char);
            if tok.is_null() {
                continue;
            }

            let mut day = atoi(tok);

            tok = strtok(ptr::null_mut(), b" \0".as_ptr() as *const c_char);
            if tok.is_null() {
                continue;
            }

            let mut year = atoi(tok);
            if year <= 70 {
                year += 100;
            } else if year >= 1900 {
                year -= 1900;
                if len < 42 {
                    continue;
                }
                filename = filename.add(2);
            }

            xfree(cur.name as *mut c_void);
            mem::zero(&mut cur, mem::size_of::<FileInfo>());
            cur.name = xstrdup(filename);

            tok = strtok(ptr::null_mut(), b":\0".as_ptr() as *const c_char);
            if tok.is_null() {
                continue;
            }

            let mut hour = atoi(tok);

            tok = strtok(ptr::null_mut(), b"M\0".as_ptr() as *const c_char);
            if tok.is_null() {
                continue;
            }

            let mut min = atoi(tok);
            if tok[0] != 0 && tok[1] != 0 {
                tok = tok.add(2);
            }

            if *tok == b'P' as c_char {
                hour += 12;
            }

            let mut timestruct = tm {
                tm_sec: 0,
                tm_min: min,
                tm_hour: hour,
                tm_mday: day,
                tm_mon: month,
                tm_year: year,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: -1,
                tm_gmtoff: 0,
                tm_zone: ptr::null_mut(),
            };

            cur.tstamp = mktime(&mut timestruct);
            cur.ptype = TT_HOUR_MIN;

            tok = str