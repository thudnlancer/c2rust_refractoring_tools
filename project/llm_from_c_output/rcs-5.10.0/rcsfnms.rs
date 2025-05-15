use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs::{self, File, Metadata};
use std::io::{self, Read, Write};
use std::os::unix::fs::MetadataExt;
use std::ptr;
use std::str;
use std::mem;
use std::collections::HashMap;
use libc::{c_char, c_int, c_void, size_t};
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{fstat, FileStat};
use nix::unistd;
use same_file::is_same_file;

const RCSDIR: &str = "RCS";
const RCSDIRLEN: usize = RCSDIR.len();

struct Compair {
    suffix: &'static str,
    comlead: &'static str,
}

static COMTABLE: &[Compair] = &[
    Compair { suffix: "a",    comlead: "-- " },    // Ada
    Compair { suffix: "ada",  comlead: "-- " },
    Compair { suffix: "adb",  comlead: "-- " },
    Compair { suffix: "ads",  comlead: "-- " },
    Compair { suffix: "asm",  comlead: ";; " },    // assembler (MS-DOS)
    Compair { suffix: "bat",  comlead: ":: " },    // batch (MS-DOS)
    Compair { suffix: "body", comlead: "-- " },    // Ada
    Compair { suffix: "c",    comlead: " * " },    // C
    Compair { suffix: "c++",  comlead: "// " },    // C++ in all its infinite guises
    Compair { suffix: "cc",   comlead: "// " },
    Compair { suffix: "cpp",  comlead: "// " },
    Compair { suffix: "cxx",  comlead: "// " },
    Compair { suffix: "cl",   comlead: ";;; " },   // Common Lisp
    Compair { suffix: "cmd",  comlead: ":: " },    // command (OS/2)
    Compair { suffix: "cmf",  comlead: "c " },     // CM Fortran
    Compair { suffix: "cs",   comlead: " * " },    // C*
    Compair { suffix: "el",   comlead: "; " },     // Emacs Lisp
    Compair { suffix: "f",    comlead: "c " },     // Fortran
    Compair { suffix: "for",  comlead: "c " },
    Compair { suffix: "h",    comlead: " * " },    // C-header
    Compair { suffix: "hpp",  comlead: "// " },    // C++ header
    Compair { suffix: "hxx",  comlead: "// " },
    Compair { suffix: "l",    comlead: " * " },    // lex (NOTE: franzlisp disagrees)
    Compair { suffix: "lisp", comlead: ";;; " },   // Lucid Lisp
    Compair { suffix: "lsp",  comlead: ";; " },    // Microsoft Lisp
    Compair { suffix: "m",    comlead: "// " },    // Objective C
    Compair { suffix: "mac",  comlead: ";; " },    // macro (DEC-10, MS-DOS, PDP-11, VMS, etc)
    Compair { suffix: "me",   comlead: ".\\\" " }, // troff -me
    Compair { suffix: "ml",   comlead: "; " },     // mocklisp
    Compair { suffix: "mm",   comlead: ".\\\" " }, // troff -mm
    Compair { suffix: "ms",   comlead: ".\\\" " }, // troff -ms
    Compair { suffix: "p",    comlead: " * " },    // Pascal
    Compair { suffix: "pas",  comlead: " * " },
    Compair { suffix: "ps",   comlead: "% " },     // PostScript
    Compair { suffix: "spec", comlead: "-- " },    // Ada
    Compair { suffix: "sty",  comlead: "% " },     // LaTeX style
    Compair { suffix: "tex",  comlead: "% " },     // TeX
    Compair { suffix: "y",    comlead: " * " },    // yacc
    Compair { suffix: "",     comlead: "# " },     // default for unknown suffix; must be last
];

struct Repo {
    tip: Option<String>,
    strictly_locking: bool,
    r: Option<String>,
    log_lead: String,
    kws: HashMap<String, String>,
    filename: Option<String>,
    stat: Option<Metadata>,
    fd_lock: Option<File>,
    cwd: Option<String>,
}

struct Mani {
    filename: Option<String>,
    standard_output: bool,
}

struct Flow {
    from: Option<File>,
    to: Option<File>,
    erroneous: bool,
}

struct Maybe {
    tentative: String,
    open: fn(&Maybe) -> Option<File>,
    mustread: bool,
    status: Option<Metadata>,
    bestfit: String,
    eno: Option<io::Error>,
    space: Vec<u8>,
}

fn init_admin(repo: &mut Repo, mani: &Mani) {
    repo.tip = None;
    repo.strictly_locking = true;
    repo.r = Some(String::new());
    
    let ext = Path::new(mani.filename.as_ref().unwrap())
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    for ent in COMTABLE {
        if ent.suffix.is_empty() || ent.suffix.eq_ignore_ascii_case(ext) {
            repo.log_lead = ent.comlead.to_string();
            break;
        }
    }
    
    repo.kws = HashMap::new();
}

fn basefilename(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

fn suffixlen(x: &str) -> usize {
    x.find(|c| c == '/' || c == '\0').unwrap_or(x.len())
}

fn rcssuffix(name: &str, pe: &str) -> Option<&str> {
    let nl = name.len();
    let nz = name.as_ptr() as usize + nl;
    
    let mut x = pe;
    loop {
        let xl = suffixlen(x);
        if xl > 0 {
            if xl <= nl {
                let p = nz - xl;
                if name[p..].starts_with(x) {
                    return Some(&name[p..]);
                }
            }
        } else {
            for p in 0..nl - RCSDIRLEN {
                if name[p + RCSDIRLEN..].starts_with("/")
                    && (p == 0 || name[p - 1..].starts_with("/"))
                    && name[p..p + RCSDIRLEN] == *RCSDIR
                {
                    return Some(&name[nl..]);
                }
            }
        }
        
        x = &x[xl..];
        if x.is_empty() {
            break;
        }
        x = &x[1..];
    }
    
    None
}

fn rcsreadopen(m: &Maybe) -> Option<File> {
    File::open(&m.tentative).ok()
}

fn finopen(m: &mut Maybe, repo: &mut Repo, flow: &mut Flow) -> bool {
    let preferold = !m.bestfit.is_empty() && (m.mustread || repo.fd_lock.is_some());
    
    flow.from = (m.open)(m);
    let interesting = flow.from.is_some() || m.eno.as_ref().map(|e| e.kind()) != Some(io::ErrorKind::NotFound);
    
    if interesting || !preferold {
        m.eno = flow.from.as_ref().map(|_| io::Error::last_os_error());
        m.bestfit = m.tentative.clone();
    }
    
    interesting
}

fn fin2open(
    d: &str,
    dlen: usize,
    base: &str,
    baselen: usize,
    x: &str,
    xlen: usize,
    m: &mut Maybe,
    repo: &mut Repo,
    flow: &mut Flow,
) -> bool {
    m.tentative = format!("{}{}/{}", &d[..dlen], RCSDIR, base);
    if finopen(m, repo, flow) {
        return true;
    }
    
    if xlen > 0 {
        m.tentative = format!("{}{}", &d[..dlen], base);
        finopen(m, repo, flow)
    } else {
        false
    }
}

fn pairnames(
    argc: usize,
    argv: &[String],
    rcsopen: fn(&Maybe) -> Option<File>,
    mustread: bool,
    quiet: bool,
    repo: &mut Repo,
    mani: &mut Mani,
    flow: &mut Flow,
) -> i32 {
    repo.fd_lock = None;
    
    let arg = &argv[0];
    if arg.starts_with('-') {
        eprintln!("{} option is ignored after filenames", arg);
        return 0;
    }
    
    let base = basefilename(arg);
    let mut paired = false;
    
    let x = rcssuffix(arg, "");
    let (rcs1, rcsbase, baselen, mani_filename) = if let Some(x) = x {
        let baselen = x.as_ptr() as usize - base.as_ptr() as usize;
        if argc > 1 {
            let p = &argv[1];
            if rcssuffix(p, "").is_none() {
                let arglen = p.len();
                if baselen <= arglen {
                    let p_ptr = p.as_ptr() as usize + arglen - baselen;
                    if p_ptr == p.as_ptr() as usize || p.chars().nth(p_ptr as usize - 1).map(|c| c == '/').unwrap_or(false) {
                        if base == &p[p_ptr as usize..p_ptr as usize + baselen] {
                            paired = true;
                            mani.filename = Some(argv[1].clone());
                        }
                    }
                }
            }
        }
        
        (arg.to_string(), base.to_string(), baselen, base[..baselen].to_string())
    } else {
        let baselen = base.len();
        if argc > 1 {
            let p = &argv[1];
            if let Some(x) = rcssuffix(p, "") {
                if p.len() >= baselen {
                    let rcsbase_ptr = x.as_ptr() as usize - baselen;
                    if rcsbase_ptr == p.as_ptr() as usize || p.chars().nth(rcsbase_ptr as usize - 1).map(|c| c == '/').unwrap_or(false) {
                        if base == &p[rcsbase_ptr as usize..rcsbase_ptr as usize + baselen] {
                            paired = true;
                            repo.filename = Some(argv[1].clone());
                        }
                    }
                }
            }
        }
        
        (String::new(), String::new(), baselen, arg.to_string())
    };
    
    mani.filename = Some(mani_filename);
    
    let mut maybe = Maybe {
        tentative: String::new(),
        open: rcsopen,
        mustread,
        status: None,
        bestfit: String::new(),
        eno: None,
        space: Vec::new(),
    };
    
    if rcsbase != rcs1 {
        maybe.bestfit = rcs1;
        maybe.tentative = rcs1;
        flow.from = (maybe.open)(&maybe);
        maybe.eno = flow.from.as_ref().map(|_| io::Error::last_os_error());
    } else {
        maybe.bestfit = String::new();
        if !rcs1.is_empty() {
            fin2open(arg, 0, &rcsbase, baselen, x, x.len(), &mut maybe, repo, flow);
        } else {
            let dlen = base.as_ptr() as usize - arg.as_ptr() as usize;
            let mut x_ptr = "";
            loop {
                let xlen = suffixlen(x_ptr);
                if !fin2open(arg, dlen, base, baselen, x_ptr, xlen, &mut maybe, repo, flow) {
                    x_ptr = &x_ptr[xlen..];
                    if x_ptr.is_empty() {
                        break;
                    }
                    x_ptr = &x_ptr[1..];
                } else {
                    break;
                }
            }
        }
    }
    
    repo.filename = Some(maybe.bestfit.clone());
    flow.erroneous = false;
    
    if let Some(from) = flow.from.take() {
        if let Some(metadata) = maybe.status {
            if !metadata.is_file() {
                eprintln!("{} isn't a regular file -- ignored", maybe.bestfit);
                return 0;
            }
        }
        // TODO: Implement grok_all
        // repo.r = grok_all(from);
        flow.to = None;
        1
    } else {
        if maybe.eno.as_ref().map(|e| e.kind()) != Some(io::ErrorKind::NotFound) || mustread || repo.fd_lock.is_none() {
            if let Some(e) = maybe.eno {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    eprintln!("RCS file {} is in use", maybe.bestfit);
                } else if !quiet || e.kind() != io::ErrorKind::NotFound {
                    eprintln!("{}: {}", maybe.bestfit, e);
                }
            }
            0
        } else {
            init_admin(repo, mani);
            -1
        }
    }
}

fn dir_useful_len(d: &str) -> usize {
    let mut dlen = d.len();
    
    if dlen == 2 && d.starts_with("//") {
        dlen -= 1;
    } else {
        while dlen > 0 && d.chars().nth(dlen - 1).map(|c| c == '/').unwrap_or(false) {
            dlen -= 1;
        }
    }
    
    dlen
}

fn getfull_rcsname(repo: &Repo) -> String {
    let r = repo.filename.as_ref().unwrap();
    
    if Path::new(r).is_absolute() {
        r.clone()
    } else {
        let cwd = repo.cwd.as_ref().unwrap();
        let mut r = r.clone();
        
        while r.starts_with("./") {
            r = r[2..].to_string();
            while r.starts_with('/') {
                r = r[1..].to_string();
            }
        }
        
        format!("{}/{}", cwd, r)
    }
}

fn is_slash(c: char) -> bool {
    c == '/'
}