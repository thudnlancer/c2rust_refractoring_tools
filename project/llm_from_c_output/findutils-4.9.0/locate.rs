use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{File, metadata};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;
use std::process;
use std::str;
use std::collections::HashMap;
use regex::Regex;
use glob::Pattern;
use nix::unistd::{getuid, getgid, setuid, setgid};
use nix::sys::stat::Mode;
use libc::{uid_t, gid_t};
use getopts::Options;
use memmap2::Mmap;
use chrono::{DateTime, Local};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const LOCATE_DB: &str = "/var/lib/mlocate/mlocate.db";
const SECONDS_PER_DAY: u64 = 86400;
const WARN_DAYS: u64 = 8;

struct LocateOptions {
    ignore_case: bool,
    print: bool,
    count: bool,
    basename_only: bool,
    limit: Option<usize>,
    regex: bool,
    stats: bool,
    op_and: bool,
    follow_symlinks: bool,
    check_existence: ExistenceCheckType,
    separator: u8,
    quote: bool,
}

#[derive(PartialEq)]
enum ExistenceCheckType {
    Either,
    Existing,
    NonExisting,
}

struct LocateStats {
    compressed_bytes: usize,
    total_files: usize,
    total_length: usize,
    whitespace_files: usize,
    newline_files: usize,
    highbit_files: usize,
}

struct ProcessData<'a> {
    original_path: String,
    munged_path: String,
    basename: String,
    db_path: &'a str,
    file: BufReader<File>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    
    opts.optflag("i", "ignore-case", "ignore case distinctions");
    opts.optflag("p", "print", "print matching entries");
    opts.optflag("c", "count", "only print count of matches");
    opts.optflag("b", "basename", "match only basename");
    opts.optopt("l", "limit", "limit output to N entries", "N");
    opts.optflag("r", "regex", "patterns are regex");
    opts.optflag("S", "stats", "print statistics");
    opts.optflag("A", "all", "all patterns must match");
    opts.optflag("e", "existing", "only print existing files");
    opts.optflag("E", "non-existing", "only print non-existing files");
    opts.optflag("L", "follow", "follow symlinks");
    opts.optflag("P", "nofollow", "don't follow symlinks");
    opts.optflag("0", "null", "separate output with null");
    opts.optflag("h", "help", "print help");
    opts.optflag("v", "version", "print version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0]);
        return Ok(());
    }

    if matches.opt_present("v") {
        println!("locate {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let options = LocateOptions {
        ignore_case: matches.opt_present("i"),
        print: matches.opt_present("p") || !matches.opt_present("c") && !matches.opt_present("S"),
        count: matches.opt_present("c"),
        basename_only: matches.opt_present("b"),
        limit: matches.opt_str("l").and_then(|s| s.parse().ok()),
        regex: matches.opt_present("r"),
        stats: matches.opt_present("S"),
        op_and: matches.opt_present("A"),
        follow_symlinks: !matches.opt_present("P"),
        check_existence: if matches.opt_present("e") {
            ExistenceCheckType::Existing
        } else if matches.opt_present("E") {
            ExistenceCheckType::NonExisting
        } else {
            ExistenceCheckType::Either
        },
        separator: if matches.opt_present("0") { 0 } else { b'\n' },
        quote: !matches.opt_present("0"),
    };

    let patterns = matches.free;
    if patterns.is_empty() && !options.stats {
        eprintln!("Error: no pattern specified");
        print_usage(&args[0]);
        process::exit(1);
    }

    drop_privileges()?;

    let db_path = env::var("LOCATE_PATH").unwrap_or_else(|_| LOCATE_DB.to_string());
    let mut found = 0;
    let mut stats = LocateStats {
        compressed_bytes: 0,
        total_files: 0,
        total_length: 0,
        whitespace_files: 0,
        newline_files: 0,
        highbit_files: 0,
    };

    for path in db_path.split(':') {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut reader = BufReader::new(file);

        let mut proc_data = ProcessData {
            original_path: String::new(),
            munged_path: String::new(),
            basename: String::new(),
            db_path: path,
            file: reader,
        };

        let count = search_database(&mut proc_data, &patterns, &options, &mut stats)?;
        found += count;

        if let Some(limit) = options.limit {
            if found >= limit {
                break;
            }
        }
    }

    if options.count {
        println!("{}", found);
    }

    if options.stats {
        print_statistics(&stats, &options);
    }

    Ok(())
}

fn drop_privileges() -> io::Result<()> {
    let uid = getuid();
    let gid = getgid();

    unsafe {
        if libc::setgroups(1, &gid.as_raw()) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    setgid(gid).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
    setuid(uid).map_err(|e| io::Error::from_raw_os_error(e as i32))?;

    Ok(())
}

fn search_database(
    proc_data: &mut ProcessData,
    patterns: &[String],
    options: &LocateOptions,
    stats: &mut LocateStats,
) -> io::Result<usize> {
    let mut count = 0;
    let mut buffer = Vec::new();

    while proc_data.file.read_until(b'\0', &mut buffer)? > 0 {
        if buffer.last() == Some(&b'\0') {
            buffer.pop();
        }

        proc_data.original_path = String::from_utf8_lossy(&buffer).into_owned();
        buffer.clear();

        if options.basename_only {
            proc_data.basename = Path::new(&proc_data.original_path)
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            proc_data.munged_path = proc_data.basename.clone();
        } else {
            proc_data.munged_path = proc_data.original_path.clone();
        }

        if !check_existence(&proc_data.original_path, options)? {
            continue;
        }

        let mut matches_all = true;
        let mut matches_any = false;

        for pattern in patterns {
            let matches = if options.regex {
                let re = if options.ignore_case {
                    Regex::new(&format!("(?i){}", pattern))
                } else {
                    Regex::new(pattern)
                }.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

                re.is_match(&proc_data.munged_path)
            } else {
                let pat = if options.ignore_case {
                    Pattern::new(&pattern.to_lowercase())
                } else {
                    Pattern::new(pattern)
                }.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

                let path = if options.ignore_case {
                    proc_data.munged_path.to_lowercase()
                } else {
                    proc_data.munged_path.clone()
                };

                pat.matches(&path)
            };

            if options.op_and {
                matches_all &= matches;
                if !matches_all {
                    break;
                }
            } else {
                matches_any |= matches;
                if matches_any {
                    break;
                }
            }
        }

        let matched = if options.op_and { matches_all } else { matches_any };
        if !matched {
            continue;
        }

        count += 1;
        update_stats(stats, &proc_data.original_path);

        if options.print {
            print_path(&proc_data.original_path, options)?;
        }

        if let Some(limit) = options.limit {
            if count >= limit {
                break;
            }
        }
    }

    Ok(count)
}

fn check_existence(path: &str, options: &LocateOptions) -> io::Result<bool> {
    match options.check_existence {
        ExistenceCheckType::Either => Ok(true),
        ExistenceCheckType::Existing => {
            if options.follow_symlinks {
                Path::new(path).exists()
            } else {
                Path::new(path).symlink_metadata().is_ok()
            }
        }
        ExistenceCheckType::NonExisting => {
            if options.follow_symlinks {
                !Path::new(path).exists()
            } else {
                Path::new(path).symlink_metadata().is_err()
            }
        }
    }
}

fn update_stats(stats: &mut LocateStats, path: &str) {
    stats.total_files += 1;
    stats.total_length += path.len();

    if path.contains(|c: char| c.is_whitespace()) {
        stats.whitespace_files += 1;
    }
    if path.contains('\n') {
        stats.newline_files += 1;
    }
    if path.chars().any(|c| c as u32 > 127) {
        stats.highbit_files += 1;
    }
}

fn print_path(path: &str, options: &LocateOptions) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if options.quote {
        write!(handle, "\"{}\"", path)?;
    } else {
        handle.write_all(path.as_bytes())?;
    }

    handle.write_all(&[options.separator])?;
    Ok(())
}

fn print_statistics(stats: &LocateStats, options: &LocateOptions) {
    println!("Database statistics:");
    println!("  Total files: {}", stats.total_files);
    println!("  Total length: {} bytes", stats.total_length);
    println!("  Files with whitespace: {}", stats.whitespace_files);
    println!("  Files with newlines: {}", stats.newline_files);
    println!("  Files with high-bit chars: {}", stats.highbit_files);
}

fn print_usage(program: &str) {
    println!("Usage: {} [OPTIONS] PATTERN", program);
    println!("Search for files in locate database");
    println!();
    println!("Options:");
    println!("  -i, --ignore-case   ignore case distinctions");
    println!("  -p, --print         print matching entries");
    println!("  -c, --count         only print count of matches");
    println!("  -b, --basename      match only basename");
    println!("  -l, --limit=N       limit output to N entries");
    println!("  -r, --regex         patterns are regex");
    println!("  -S, --stats         print statistics");
    println!("  -A, --all           all patterns must match");
    println!("  -e, --existing      only print existing files");
    println!("  -E, --non-existing  only print non-existing files");
    println!("  -L, --follow        follow symlinks");
    println!("  -P, --nofollow      don't follow symlinks");
    println!("  -0, --null          separate output with null");
    println!("  -h, --help          print this help");
    println!("  -v, --version       print version");
}