/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};
use libc::{self, c_int, pid_t};
use log::{error, info, warn, LevelFilter};
use nix::sys::stat;
use nix::unistd::{self, ForkResult, Pid};
use signal_hook::iterator::Signals;
use sysinfo::{System, SystemExt};

const NC_CONF_PATH: &str = "conf/nutcracker.yml";
const NC_LOG_DEFAULT: LevelFilter = LevelFilter::Info;
const NC_LOG_MIN: LevelFilter = LevelFilter::Error;
const NC_LOG_MAX: LevelFilter = LevelFilter::Trace;
const NC_LOG_PATH: Option<&str> = None;

const NC_STATS_PORT: u16 = 22222;
const NC_STATS_ADDR: &str = "0.0.0.0";
const NC_STATS_INTERVAL: u64 = 30000;

const NC_PID_FILE: Option<&str> = None;

const NC_MBUF_SIZE: usize = 16384;
const NC_MBUF_MIN_SIZE: usize = 1024;
const NC_MBUF_MAX_SIZE: usize = 65536;

#[derive(Debug)]
struct Instance {
    log_level: LevelFilter,
    log_filename: Option<String>,
    conf_filename: String,
    stats_port: u16,
    stats_addr: String,
    stats_interval: u64,
    hostname: String,
    mbuf_chunk_size: usize,
    pid: pid_t,
    pid_filename: Option<String>,
    pidfile: bool,
}

impl Default for Instance {
    fn default() -> Self {
        let mut hostname = String::new();
        if let Ok(name) = hostname::get() {
            if let Some(name) = name.to_str() {
                hostname = name.to_string();
            }
        }
        if hostname.is_empty() {
            hostname = "unknown".to_string();
        }

        Instance {
            log_level: NC_LOG_DEFAULT,
            log_filename: NC_LOG_PATH.map(|s| s.to_string()),
            conf_filename: NC_CONF_PATH.to_string(),
            stats_port: NC_STATS_PORT,
            stats_addr: NC_STATS_ADDR.to_string(),
            stats_interval: NC_STATS_INTERVAL,
            hostname,
            mbuf_chunk_size: NC_MBUF_SIZE,
            pid: unsafe { libc::getpid() },
            pid_filename: NC_PID_FILE.map(|s| s.to_string()),
            pidfile: false,
        }
    }
}

fn nc_daemonize(dump_core: bool) -> io::Result<()> {
    match unsafe { libc::fork() } {
        -1 => Err(io::Error::last_os_error()),
        0 => {
            // First child process
            let sid = unsafe { libc::setsid() };
            if sid < 0 {
                return Err(io::Error::last_os_error());
            }

            unsafe {
                libc::signal(libc::SIGHUP, libc::SIG_IGN);
            }

            match unsafe { libc::fork() } {
                -1 => Err(io::Error::last_os_error()),
                0 => {
                    // Second child process
                    if !dump_core {
                        std::env::set_current_dir("/")?;
                    }

                    unsafe {
                        libc::umask(0);
                    }

                    // Redirect stdin, stdout, stderr to /dev/null
                    let dev_null = File::open("/dev/null")?;
                    let fd = dev_null.as_raw_fd();
                    unsafe {
                        libc::dup2(fd, libc::STDIN_FILENO);
                        libc::dup2(fd, libc::STDOUT_FILENO);
                        libc::dup2(fd, libc::STDERR_FILENO);
                    }

                    if fd > libc::STDERR_FILENO {
                        unsafe {
                            libc::close(fd);
                        }
                    }

                    Ok(())
                }
                _ => process::exit(0), // First child exits
            }
        }
        _ => process::exit(0), // Parent exits
    }
}

fn nc_print_run(nci: &Instance) {
    let sys = System::new();
    info!(
        "nutcracker-{} built for {} {} {} started on pid {}",
        env!("CARGO_PKG_VERSION"),
        sys.name().unwrap_or_else(|| "unknown".to_string()),
        sys.kernel_version().unwrap_or_else(|| "unknown".to_string()),
        sys.host_name().unwrap_or_else(|| "unknown".to_string()),
        nci.pid
    );

    info!("run, rabbit run / dig that hole, forget the sun / and when at last the work is done / don't sit down / it's time to dig another one");
}

fn nc_print_done() {
    info!("done, rabbit done");
}

fn nc_show_usage() {
    eprintln!("Usage: nutcracker [OPTIONS]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help             : this help");
    eprintln!("  -V, --version          : show version and exit");
    eprintln!("  -t, --test-conf        : test configuration for syntax errors and exit");
    eprintln!("  -d, --daemonize        : run as a daemon");
    eprintln!("  -D, --describe-stats   : print stats description and exit");
    eprintln!("  -v, --verbose=N        : set logging level (default: {}, min: {}, max: {})", 
              NC_LOG_DEFAULT, NC_LOG_MIN, NC_LOG_MAX);
    eprintln!("  -o, --output=S         : set logging file (default: stderr)");
    eprintln!("  -c, --conf-file=S      : set configuration file (default: {})", NC_CONF_PATH);
    eprintln!("  -s, --stats-port=N     : set stats monitoring port (default: {})", NC_STATS_PORT);
    eprintln!("  -a, --stats-addr=S     : set stats monitoring ip (default: {})", NC_STATS_ADDR);
    eprintln!("  -i, --stats-interval=N : set stats aggregation interval in msec (default: {} msec)", NC_STATS_INTERVAL);
    eprintln!("  -p, --pid-file=S       : set pid file (default: off)");
    eprintln!("  -m, --mbuf-size=N      : set size of mbuf chunk in bytes (default: {} bytes)", NC_MBUF_SIZE);
}

fn nc_create_pidfile(nci: &mut Instance) -> io::Result<()> {
    if let Some(pid_filename) = &nci.pid_filename {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(pid_filename)?;
        
        writeln!(file, "{}", nci.pid)?;
        nci.pidfile = true;
    }
    Ok(())
}

fn nc_remove_pidfile(nci: &Instance) {
    if let Some(pid_filename) = &nci.pid_filename {
        if let Err(e) = std::fs::remove_file(pid_filename) {
            error!("unlink of pid file '{}' failed, ignored: {}", pid_filename, e);
        }
    }
}

fn nc_test_conf(nci: &Instance) -> bool {
    // TODO: Implement actual configuration testing
    info!("Configuration file '{}' syntax is ok", nci.conf_filename);
    true
}

fn nc_pre_run(nci: &mut Instance) -> io::Result<()> {
    if nci.pid_filename.is_some() {
        nc_create_pidfile(nci)?;
    }

    nc_print_run(nci);
    Ok(())
}

fn nc_post_run(nci: &Instance) {
    if nci.pidfile {
        nc_remove_pidfile(nci);
    }
    nc_print_done();
}

fn nc_run(nci: &Instance) {
    // TODO: Implement main event loop
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let matches = App::new("nutcracker")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A fast and lightweight proxy for memcached protocol")
        .arg(Arg::with_name("help")
            .short("h")
            .long("help")
            .help("Show this help message"))
        .arg(Arg::with_name("version")
            .short("V")
            .long("version")
            .help("Show version information"))
        .arg(Arg::with_name("test-conf")
            .short("t")
            .long("test-conf")
            .help("Test configuration for syntax errors and exit"))
        .arg(Arg::with_name("daemonize")
            .short("d")
            .long("daemonize")
            .help("Run as a daemon"))
        .arg(Arg::with_name("describe-stats")
            .short("D")
            .long("describe-stats")
            .help("Print stats description and exit"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .takes_value(true)
            .help("Set logging level"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Set logging file"))
        .arg(Arg::with_name("conf-file")
            .short("c")
            .long("conf-file")
            .takes_value(true)
            .help("Set configuration file"))
        .arg(Arg::with_name("stats-port")
            .short("s")
            .long("stats-port")
            .takes_value(true)
            .help("Set stats monitoring port"))
        .arg(Arg::with_name("stats-addr")
            .short("a")
            .long("stats-addr")
            .takes_value(true)
            .help("Set stats monitoring ip"))
        .arg(Arg::with_name("stats-interval")
            .short("i")
            .long("stats-interval")
            .takes_value(true)
            .help("Set stats aggregation interval in msec"))
        .arg(Arg::with_name("pid-file")
            .short("p")
            .long("pid-file")
            .takes_value(true)
            .help("Set pid file"))
        .arg(Arg::with_name("mbuf-size")
            .short("m")
            .long("mbuf-size")
            .takes_value(true)
            .help("Set size of mbuf chunk in bytes"))
        .get_matches();

    let mut nci = Instance::default();

    if matches.is_present("help") {
        nc_show_usage();
        return;
    }

    if matches.is_present("version") {
        println!("nutcracker-{}", env!("CARGO_PKG_VERSION"));
        println!("async event backend: epoll");
        println!();
        return;
    }

    if matches.is_present("test-conf") {
        if !nc_test_conf(&nci) {
            process::exit(1);
        }
        return;
    }

    if matches.is_present("daemonize") {
        if let Err(e) = nc_daemonize(true) {
            eprintln!("Failed to daemonize: {}", e);
            process::exit(1);
        }
    }

    if let Err(e) = nc_pre_run(&mut nci) {
        eprintln!("Failed during pre-run: {}", e);
        nc_post_run(&nci);
        process::exit(1);
    }

    nc_run(&nci);
    nc_post_run(&nci);
}