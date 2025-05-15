use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;
use std::sync::Arc;
use fnmatch::fnmatch;
use lazy_static::lazy_static;
use libc::{O_RDONLY, F_OK};
use nix::fcntl::{openat, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::close;
use regex::Regex;

const EXCL_NON_RECURSIVE: i32 = 1;
const EXCL_DEFAULT: i32 = 0;
const EXCLUDE_WILDCARDS: i32 = 1 << 0;
const EXCLUDE_REGEX: i32 = 1 << 1;
const EXCLUDE_INCLUDE: i32 = 1 << 2;
const FNM_FILE_NAME: i32 = 1 << 0;
const FNM_LEADING_DIR: i32 = 1 << 1;
const EXCLUDE_ANCHORED: i32 = 1 << 3;
const EXCLUDE_ALLOC: i32 = 1 << 4;

type AddFn = fn(exclude: &mut Exclude, pattern: &str, options: i32, data: Option<&mut dyn std::any::Any>) -> io::Result<()>;

struct VcsIgnoreFile {
    filename: &'static str,
    flags: i32,
    addfn: AddFn,
    initfn: Option<fn(data: Option<&mut dyn std::any::Any>) -> Option<Box<dyn std::any::Any>>>,
    data: Option<Box<dyn std::any::Any>>,
}

struct ExcFile {
    next: Option<Box<ExcFile>>,
    flags: i32,
    name: String,
}

struct ExcludeList {
    next: Option<Box<ExcludeList>>,
    prev: Option<*mut ExcludeList>,
    flags: i32,
    excluded: Box<Exclude>,
}

struct Exclude {
    patterns: Vec<String>,
    options: i32,
}

impl Exclude {
    fn new() -> Self {
        Exclude {
            patterns: Vec::new(),
            options: 0,
        }
    }

    fn add_pattern(&mut self, pattern: &str, options: i32) {
        self.patterns.push(pattern.to_string());
        self.options = options;
    }

    fn add_pattern_buffer(&mut self, pattern: String, options: i32) {
        self.patterns.push(pattern);
        self.options = options;
    }
}

struct TarStatInfo {
    fd: i32,
    exclude_list: Option<Box<ExcludeList>>,
    parent: Option<Box<TarStatInfo>>,
}

fn excfile_add(name: &str, flags: i32, excfile_head: &mut Option<Box<ExcFile>>, excfile_tail: &mut Option<Box<ExcFile>>) {
    let new_file = Box::new(ExcFile {
        next: None,
        flags,
        name: name.to_string(),
    });

    if let Some(tail) = excfile_tail {
        tail.next = Some(new_file);
    } else {
        *excfile_head = Some(new_file);
    }
    *excfile_tail = Some(new_file);
}

fn info_attach_exclist(dir: &mut TarStatInfo, excfile_head: &Option<Box<ExcFile>>) -> io::Result<()> {
    if dir.exclude_list.is_some() {
        return Ok(());
    }

    let mut head: Option<Box<ExcludeList>> = None;
    let mut tail: Option<Box<ExcludeList>> = None;

    let mut current = excfile_head;
    while let Some(file) = current {
        let fd = openat(
            Some(dir.fd),
            file.name.as_str(),
            OFlag::from_bits(O_RDONLY).unwrap(),
            Mode::empty(),
        )?;

        if fd == -1 {
            continue;
        }

        let fp = File::from_raw_fd(fd);
        let reader = BufReader::new(fp);

        let mut ex = Exclude::new();
        let vcsfile = get_vcs_ignore_file(&file.name);

        if let Some(initfn) = vcsfile.initfn {
            vcsfile.data = initfn(vcsfile.data.as_deref_mut());
        }

        for line in reader.lines() {
            let line = line?;
            (vcsfile.addfn)(&mut ex, &line, vcsfile.flags, vcsfile.data.as_deref_mut())?;
        }

        let ent = Box::new(ExcludeList {
            excluded: Box::new(ex),
            flags: if file.flags == EXCL_DEFAULT {
                file.flags
            } else {
                vcsfile.flags
            },
            prev: tail.as_deref_mut().map(|t| t as *mut _),
            next: None,
        });

        if let Some(t) = &mut tail {
            t.next = Some(ent);
        } else {
            head = Some(ent);
        }
        tail = head.as_mut();

        current = &file.next;
    }

    dir.exclude_list = head;
    Ok(())
}

fn info_free_exclist(dir: &mut TarStatInfo) {
    let mut ep = dir.exclude_list.take();
    while let Some(mut e) = ep {
        ep = e.next.take();
    }
}

fn excluded_name(name: &str, st: Option<&TarStatInfo>) -> bool {
    let name = &name[FILE_SYSTEM_PREFIX_LEN(name)..];

    if excluded_file_name(&EXCLUDED, name) {
        return true;
    }

    let st = match st {
        Some(s) => s,
        None => return false,
    };

    let mut result = false;
    let mut st = Some(st);
    let mut nr = 0;

    while let Some(s) = st {
        let mut ep = &s.exclude_list;
        while let Some(e) = ep {
            if e.flags & nr != 0 {
                ep = &e.next;
                continue;
            }

            if excluded_file_name(&e.excluded, name) {
                return true;
            }

            let rname = {
                let mut rname = name;
                while rname.starts_with("./") {
                    rname = &rname[2..];
                }
                rname
            };

            if excluded_file_name(&e.excluded, rname) {
                return true;
            }

            let bname = base_name(name);
            if excluded_file_name(&e.excluded, &bname) {
                return true;
            }

            ep = &e.next;
        }

        st = s.parent.as_deref();
        nr = EXCL_NON_RECURSIVE;
    }

    result
}

lazy_static! {
    static ref EXCLUDED: Exclude = Exclude::new();
}

fn cvs_addfn(ex: &mut Exclude, pattern: &str, options: i32, _data: Option<&mut dyn std::any::Any>) -> io::Result<()> {
    let words: Vec<&str> = pattern.split_whitespace().collect();
    for word in words {
        ex.add_pattern(word, options);
    }
    Ok(())
}

fn git_addfn(ex: &mut Exclude, pattern: &str, options: i32, _data: Option<&mut dyn std::any::Any>) -> io::Result<()> {
    let pattern = pattern.trim();
    if pattern.is_empty() || pattern.starts_with('#') {
        return Ok(());
    }
    let pattern = pattern.strip_prefix("\\#").unwrap_or(pattern);
    ex.add_pattern(pattern, options);
    Ok(())
}

fn bzr_addfn(ex: &mut Exclude, pattern: &str, options: i32, _data: Option<&mut dyn std::any::Any>) -> io::Result<()> {
    let mut pattern = pattern.trim();
    if pattern.is_empty() || pattern.starts_with('#') {
        return Ok(());
    }

    let mut options = options;
    if pattern.starts_with('!') {
        pattern = &pattern[1..];
        if pattern.starts_with('!') {
            pattern = &pattern[1..];
        } else {
            options |= EXCLUDE_INCLUDE;
        }
    }

    if pattern.starts_with("RE:") {
        pattern = &pattern[3..];
        ex.add_pattern(pattern, (options & !EXCLUDE_WILDCARDS) | EXCLUDE_REGEX);
    } else {
        ex.add_pattern(pattern, options);
    }
    Ok(())
}

fn hg_initfn(data: Option<&mut dyn std::any::Any>) -> Option<Box<dyn std::any::Any>> {
    Some(Box::new(EXCLUDE_REGEX) as Box<dyn std::any::Any>)
}

fn hg_addfn(ex: &mut Exclude, pattern: &str, options: i32, data: Option<&mut dyn std::any::Any>) -> io::Result<()> {
    let pattern = pattern.trim();
    if pattern.is_empty() || pattern.starts_with('#') {
        return Ok(());
    }

    let hgopt = data.and_then(|d| d.downcast_mut::<i32>()).unwrap_or(&mut (EXCLUDE_REGEX as i32));

    if pattern.starts_with("syntax:") {
        let syntax = pattern[7..].trim();
        *hgopt = if syntax == "regexp" {
            EXCLUDE_REGEX
        } else if syntax == "glob" {
            EXCLUDE_WILDCARDS
        } else {
            *hgopt
        };
        return Ok(());
    }

    let mut pattern = pattern.to_string();
    if pattern.ends_with('/') {
        pattern.pop();
        ex.add_pattern_buffer(pattern, options | FNM_LEADING_DIR | EXCLUDE_ALLOC);
    } else {
        let opts = if *hgopt == EXCLUDE_REGEX {
            (options & !EXCLUDE_WILDCARDS) | EXCLUDE_REGEX
        } else {
            (options & !EXCLUDE_REGEX) | EXCLUDE_WILDCARDS
        };
        ex.add_pattern(&pattern, opts);
    }
    Ok(())
}

lazy_static! {
    static ref VCS_IGNORE_FILES: [VcsIgnoreFile; 5] = [
        VcsIgnoreFile {
            filename: ".cvsignore",
            flags: EXCL_NON_RECURSIVE,
            addfn: cvs_addfn,
            initfn: None,
            data: None,
        },
        VcsIgnoreFile {
            filename: ".gitignore",
            flags: 0,
            addfn: git_addfn,
            initfn: None,
            data: None,
        },
        VcsIgnoreFile {
            filename: ".bzrignore",
            flags: 0,
            addfn: bzr_addfn,
            initfn: None,
            data: None,
        },
        VcsIgnoreFile {
            filename: ".hgignore",
            flags: 0,
            addfn: hg_addfn,
            initfn: Some(hg_initfn),
            data: None,
        },
        VcsIgnoreFile {
            filename: "",
            flags: 0,
            addfn: git_addfn,
            initfn: None,
            data: None,
        },
    ];
}

fn get_vcs_ignore_file(name: &str) -> &'static VcsIgnoreFile {
    VCS_IGNORE_FILES.iter().find(|f| f.filename == name).unwrap_or(&VCS_IGNORE_FILES[4])
}

fn exclude_vcs_ignores(excfile_head: &mut Option<Box<ExcFile>>, excfile_tail: &mut Option<Box<ExcFile>>) {
    for file in VCS_IGNORE_FILES.iter().take(4) {
        excfile_add(file.filename, EXCL_DEFAULT, excfile_head, excfile_tail);
    }
}

fn excluded_file_name(ex: &Exclude, name: &str) -> bool {
    ex.patterns.iter().any(|p| fnmatch(p, name, ex.options).unwrap_or(false))
}

fn base_name(name: &str) -> String {
    Path::new(name).file_name().unwrap().to_string_lossy().into_owned()
}

fn FILE_SYSTEM_PREFIX_LEN(name: &str) -> usize {
    if name.starts_with("//") && name.len() > 2 && !name[2..].starts_with('/') {
        2
    } else if name.starts_with('/') {
        1
    } else {
        0
    }
}