use std::{
    ffi::CString,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    mem,
    os::unix::prelude::*,
    path::{Path, PathBuf},
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

const UNKNOWN_ATTEMPTED_SIZE: i64 = -3;
const MAX_PINNED_PUBKEY_SIZE: usize = 1048576; // 1MB

#[derive(Debug)]
struct FileMemory {
    content: Vec<u8>,
    mmap_p: bool,
}

#[derive(Debug)]
struct FileStats {
    access_err: i32,
    st_ino: u64,
    st_dev: u64,
}

fn xmalloc(size: usize) -> *mut u8 {
    unsafe {
        let ptr = libc::malloc(size);
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }
        ptr as *mut u8
    }
}

fn xrealloc(ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        let new_ptr = libc::realloc(ptr as *mut libc::c_void, size);
        if new_ptr.is_null() {
            panic!("Memory reallocation failed");
        }
        new_ptr as *mut u8
    }
}

fn xfree(ptr: *mut u8) {
    unsafe {
        libc::free(ptr as *mut libc::c_void);
    }
}

fn xstrdup(s: &str) -> *mut u8 {
    let len = s.len() + 1;
    let ptr = xmalloc(len);
    unsafe {
        ptr::copy_nonoverlapping(s.as_ptr(), ptr, len);
    }
    ptr
}

fn memfatal(context: &str, attempted_size: i64) -> ! {
    if attempted_size == UNKNOWN_ATTEMPTED_SIZE {
        eprintln!("{}: {}: Failed to allocate enough memory; memory exhausted.", "wget", context);
    } else {
        eprintln!(
            "{}: {}: Failed to allocate {} bytes; memory exhausted.",
            "wget", context, attempted_size
        );
    }
    std::process::exit(1);
}

fn xstrdup_lower(s: &str) -> *mut u8 {
    let copy = xstrdup(s);
    unsafe {
        let mut p = copy;
        while *p != 0 {
            *p = (*p).to_ascii_lowercase();
            p = p.add(1);
        }
    }
    copy
}

fn strdupdelim(beg: *const u8, end: *const u8) -> *mut u8 {
    if !beg.is_null() && beg <= end {
        let len = unsafe { end.offset_from(beg) } as usize;
        let res = xmalloc(len + 1);
        unsafe {
            ptr::copy_nonoverlapping(beg, res, len);
            *res.add(len) = 0;
        }
        res
    } else {
        xstrdup("")
    }
}

fn sepstring(s: &str) -> *mut *mut u8 {
    if s.is_empty() {
        return ptr::null_mut();
    }

    let mut res: Vec<*mut u8> = Vec::new();
    let mut p = s.as_ptr();
    let mut start = p;

    for c in s.bytes() {
        if c == b',' {
            let item = strdupdelim(start, p);
            res.push(item);
            p = p.add(1);
            // Skip whitespace after comma
            while unsafe { *p }.is_ascii_whitespace() {
                p = p.add(1);
            }
            start = p;
        } else {
            p = p.add(1);
        }
    }

    // Add last item
    let item = strdupdelim(start, p);
    res.push(item);

    // Convert to null-terminated array
    let mut arr = xmalloc((res.len() + 1) * mem::size_of::<*mut u8>()) as *mut *mut u8;
    unsafe {
        for (i, &item) in res.iter().enumerate() {
            *arr.add(i) = item;
        }
        *arr.add(res.len()) = ptr::null_mut();
    }

    arr
}

fn aprintf(fmt: &str, args: std::fmt::Arguments) -> *mut u8 {
    let s = format!("{}", args);
    xstrdup(&s)
}

fn concat_strings(strings: &[&str]) -> *mut u8 {
    let total_len = strings.iter().map(|s| s.len()).sum::<usize>();
    let mut buf = Vec::with_capacity(total_len + 1);
    for s in strings {
        buf.extend_from_slice(s.as_bytes());
    }
    buf.push(0);
    let ptr = xmalloc(buf.len());
    unsafe {
        ptr::copy_nonoverlapping(buf.as_ptr(), ptr, buf.len());
    }
    ptr
}

fn time_str(t: SystemTime) -> *mut u8 {
    let duration = t.duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    let tm = unsafe { libc::localtime(&secs as *const _) };
    if tm.is_null() {
        panic!("localtime failed");
    }
    let mut buf = [0u8; 32];
    unsafe {
        libc::strftime(
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len(),
            b"%H:%M:%S\0".as_ptr() as *const libc::c_char,
            tm,
        );
    }
    xstrdup(unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }.to_str().unwrap())
}

fn datetime_str(t: SystemTime) -> *mut u8 {
    let duration = t.duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    let tm = unsafe { libc::localtime(&secs as *const _) };
    if tm.is_null() {
        panic!("localtime failed");
    }
    let mut buf = [0u8; 32];
    unsafe {
        libc::strftime(
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len(),
            b"%Y-%m-%d %H:%M:%S\0".as_ptr() as *const libc::c_char,
            tm,
        );
    }
    xstrdup(unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }.to_str().unwrap())
}

fn fork_to_background() -> bool {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("fork: {}", io::Error::last_os_error());
            std::process::exit(1);
        } else if pid != 0 {
            println!("Continuing in background, pid {}", pid);
            std::process::exit(0);
        }

        if libc::setsid() < 0 {
            eprintln!("setsid failed");
            std::process::exit(1);
        }

        let null = CString::new("/dev/null").unwrap();
        let stdin = libc::freopen(null.as_ptr(), b"r\0".as_ptr() as *const libc::c_char, libc::stdin);
        let stdout = libc::freopen(null.as_ptr(), b"w\0".as_ptr() as *const libc::c_char, libc::stdout);
        let stderr = libc::freopen(null.as_ptr(), b"w\0".as_ptr() as *const libc::c_char, libc::stderr);

        if stdin.is_null() || stdout.is_null() || stderr.is_null() {
            eprintln!("Failed to redirect stdio to /dev/null");
            std::process::exit(1);
        }

        false
    }
}

fn touch(file: &str, tm: SystemTime) {
    let path = CString::new(file).unwrap();
    let duration = tm.duration_since(UNIX_EPOCH).unwrap();
    let times = [
        libc::timeval {
            tv_sec: duration.as_secs() as i64,
            tv_usec: duration.subsec_micros() as i64,
        },
        libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    ];
    unsafe {
        if libc::utimes(path.as_ptr(), times.as_ptr()) < 0 {
            eprintln!("utime({}): {}", file, io::Error::last_os_error());
        }
    }
}

fn remove_link(file: &str) -> i32 {
    let path = CString::new(file).unwrap();
    let mut st: libc::stat = unsafe { mem::zeroed() };
    unsafe {
        if libc::lstat(path.as_ptr(), &mut st) == 0 && (st.st_mode & libc::S_IFMT) == libc::S_IFLNK {
            if libc::unlink(path.as_ptr()) != 0 {
                eprintln!("Failed to unlink symlink {}: {}", file, io::Error::last_os_error());
                return -1;
            }
            return 0;
        }
    }
    0
}

fn file_exists_p(filename: &str, fstats: Option<&mut FileStats>) -> bool {
    if filename.is_empty() {
        return false;
    }

    let path = Path::new(filename);
    match fs::metadata(path) {
        Ok(metadata) => {
            if let Some(stats) = fstats {
                stats.access_err = 0;
                stats.st_ino = metadata.ino();
                stats.st_dev = metadata.dev();
            }
            true
        }
        Err(e) => {
            if let Some(stats) = fstats {
                stats.access_err = e.raw_os_error().unwrap_or(0);
            }
            false
        }
    }
}

fn file_non_directory_p(path: &str) -> bool {
    let path = Path::new(path);
    match fs::symlink_metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}

fn file_size(filename: &str) -> i64 {
    match fs::metadata(filename) {
        Ok(metadata) => metadata.len() as i64,
        Err(_) => -1,
    }
}

fn make_directory(directory: &str) -> i32 {
    let path = Path::new(directory);
    if path.exists() {
        return 0;
    }
    match fs::create_dir_all(path) {
        Ok(_) => 0,
        Err(e) => e.raw_os_error().unwrap_or(-1),
    }
}

fn file_merge(base: &str, file: &str) -> *mut u8 {
    let base_path = Path::new(base);
    let file_path = Path::new(file);
    let merged = if let Some(parent) = base_path.parent() {
        parent.join(file_path)
    } else {
        file_path.to_path_buf()
    };
    xstrdup(merged.to_str().unwrap())
}

fn fnmatch_nocase(pattern: &str, string: &str, flags: i32) -> i32 {
    // FNM_CASEFOLD is not standard, but commonly available
    let flags = flags | 0x10; // FNM_CASEFOLD
    let pattern_c = CString::new(pattern).unwrap();
    let string_c = CString::new(string).unwrap();
    unsafe { libc::fnmatch(pattern_c.as_ptr(), string_c.as_ptr(), flags) }
}

fn acceptable(s: &str) -> bool {
    // Implementation depends on global options
    true
}

fn accept_url(s: &str) -> bool {
    // Implementation depends on global options
    true
}

fn subdir_p(d1: &str, d2: &str) -> bool {
    let d1 = d1.trim_start_matches('/');
    let d2 = d2.trim_start_matches('/');
    d2.starts_with(d1) && (d2.len() == d1.len() || d2.as_bytes()[d1.len()] == b'/')
}

fn dir_matches_p(dirlist: &[&str], dir: &str) -> bool {
    for pattern in dirlist {
        if pattern.contains(&['*', '?', '[', ']'][..]) {
            if fnmatch_nocase(pattern, dir, 0) == 0 {
                return true;
            }
        } else if subdir_p(pattern, dir) {
            return true;
        }
    }
    false
}

fn accdir(directory: &str) -> bool {
    let directory = directory.trim_start_matches('/');
    // Implementation depends on global options
    true
}

fn match_tail(string: &str, tail: &str, fold_case: bool) -> bool {
    if string.len() < tail.len() {
        return false;
    }
    let start = string.len() - tail.len();
    if fold_case {
        string[start..].eq_ignore_ascii_case(tail)
    } else {
        &string[start..] == tail
    }
}

fn in_acclist(accepts: &[&str], s: &str, backward: bool) -> bool {
    for pattern in accepts {
        if pattern.contains(&['*', '?', '[', ']'][..]) {
            if fnmatch_nocase(pattern, s, 0) == 0 {
                return true;
            }
        } else if backward {
            if match_tail(s, pattern, false) {
                return true;
            }
        } else if s == *pattern {
            return true;
        }
    }
    false
}

fn suffix(s: &str) -> Option<&str> {
    if let Some(dot_pos) = s.rfind('.') {
        if !s[dot_pos + 1..].contains('/') {
            return Some(&s[dot_pos + 1..]);
        }
    }
    None
}

fn has_wildcards_p(s: &str) -> bool {
    s.contains(&['*', '?', '[', ']'][..])
}

fn has_html_suffix_p(fname: &str) -> bool {
    if let Some(suf) = suffix(fname) {
        if suf.eq_ignore_ascii_case("html") || suf.eq_ignore_ascii_case("htm") {
            return true;
        }
        if suf.len() > 1 && suf[1..].eq_ignore_ascii_case("html") {
            return true;
        }
    }
    false
}

fn wget_read_file(file: &str) -> Option<Box<FileMemory>> {
    if file == "-" {
        let mut content = Vec::new();
        if let Ok(_) = io::stdin().read_to_end(&mut content) {
            return Some(Box::new(FileMemory {
                content,
                mmap_p: false,
            }));
        }
        return None;
    }

    match fs::read(file) {
        Ok(content) => Some(Box::new(FileMemory {
            content,
            mmap_p: false,
        })),
        Err(_) => None,
    }
}

fn wget_read_file_free(fm: Box<FileMemory>) {
    // Memory is automatically freed when Box goes out of scope
}

fn free_vec(vec: *mut *mut u8) {
    if !vec.is_null() {
        unsafe {
            let mut p = vec;
            while !(*p).is_null() {
                xfree(*p);
                p = p.add(1);
            }
            xfree(vec as *mut u8);
        }
    }
}

fn merge_vecs(v1: *mut *mut u8, v2: *mut *mut u8) -> *mut *mut u8 {
    if v1.is_null() {
        return v2;
    }
    if v2.is_null() {
        return v1;
    }

    unsafe {
        // Count elements in v1
        let mut i = 0;
        while !(*v1.add(i)).is_null() {
            i += 1;
        }

        // Count elements in v2
        let mut j = 0;
        while !(*v2.add(j)).is_null() {
            j += 1;
        }

        if j == 0 {
            xfree(v2 as *mut u8);
            return v1;
        }

        // Reallocate v1
        let new_size = (i + j + 1) * mem::size_of::<*mut u8>();
        let new_v1 = xrealloc(v1 as *mut u8, new_size) as *mut *mut u8;

        // Copy v2 after v1
        ptr::copy_nonoverlapping(v2, new_v1.add(i), j + 1);

        xfree(v2 as *mut u8);
        new_v1
    }
}

fn vec_append(vec: *mut *mut u8, str: &str) -> *mut *mut u8 {
    let mut cnt = 0;
    unsafe {
        if !vec.is_null() {
            while !(*vec.add(cnt)).is_null() {
                cnt += 1;
            }
            cnt += 1;
        }

        let new_size = (cnt + 1) * mem::size_of::<*mut u8>();
        let new_vec = if vec.is_null() {
            xmalloc(new_size) as *mut *mut u8
        } else {
            xrealloc(vec as *mut u8, new_size) as *mut *mut u8
        };

        *new_vec.add(cnt - 1) = xstrdup(str);
        *new_vec.add(cnt) = ptr::null_mut();
        new_vec
    }
}

fn string_set_add(ht: *mut libc::c_void, s: &str) {
    // Implementation depends on hash table implementation
}

fn string_set_contains(ht: *mut libc::c_void, s: &str) -> bool {
    // Implementation depends on hash table implementation
    false
}

fn string_set_to_array(ht: *mut libc::c_void, array: *mut *mut u8) {
    // Implementation depends on hash table implementation
}

fn string_set_free(ht: *mut libc::c_void) {
    //