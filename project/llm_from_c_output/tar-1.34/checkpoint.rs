use std::collections::LinkedList;
use std::ffi::{CString, OsString};
use std::fs::File;
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use nix::sys::signal::{self, SigSet};
use nix::sys::termios;
use libc::{c_int, winsize, TIOCGWINSZ};
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum CheckpointOpcode {
    Dot,
    Bell,
    Echo,
    TtyOut,
    Sleep,
    Exec,
    Totals,
    Wait,
}

#[derive(Debug)]
enum CheckpointValue {
    Time(u64),
    Command(String),
    Signal(i32),
}

#[derive(Debug)]
struct CheckpointAction {
    opcode: CheckpointOpcode,
    value: CheckpointValue,
}

struct CheckpointSystem {
    checkpoint: AtomicUsize,
    actions: LinkedList<CheckpointAction>,
    state: CheckpointState,
    blocked_signals: SigSet,
    tty: Option<File>,
    tty_cleanup: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CheckpointState {
    Init,
    Compile,
    Run,
}

impl CheckpointSystem {
    fn new() -> Self {
        Self {
            checkpoint: AtomicUsize::new(0),
            actions: LinkedList::new(),
            state: CheckpointState::Init,
            blocked_signals: SigSet::empty(),
            tty: None,
            tty_cleanup: false,
        }
    }

    fn compile_action(&mut self, str: &str) -> Result<(), String> {
        if self.state == CheckpointState::Init {
            self.blocked_signals = SigSet::empty();
            self.state = CheckpointState::Compile;
        }

        let action = match str {
            "." | "dot" => CheckpointAction {
                opcode: CheckpointOpcode::Dot,
                value: CheckpointValue::Time(0),
            },
            "bell" => CheckpointAction {
                opcode: CheckpointOpcode::Bell,
                value: CheckpointValue::Time(0),
            },
            "echo" => CheckpointAction {
                opcode: CheckpointOpcode::Echo,
                value: CheckpointValue::Command(String::new()),
            },
            _ if str.starts_with("echo=") => CheckpointAction {
                opcode: CheckpointOpcode::Echo,
                value: CheckpointValue::Command(unquote_string(&str[5..])),
            },
            _ if str.starts_with("exec=") => CheckpointAction {
                opcode: CheckpointOpcode::Exec,
                value: CheckpointValue::Command(unquote_string(&str[5..])),
            },
            _ if str.starts_with("ttyout=") => CheckpointAction {
                opcode: CheckpointOpcode::TtyOut,
                value: CheckpointValue::Command(unquote_string(&str[7..])),
            },
            _ if str.starts_with("sleep=") => {
                let n = str[6..].parse::<u64>().map_err(|_| format!("{}: not a valid timeout", str))?;
                CheckpointAction {
                    opcode: CheckpointOpcode::Sleep,
                    value: CheckpointValue::Time(n),
                }
            }
            "totals" => CheckpointAction {
                opcode: CheckpointOpcode::Totals,
                value: CheckpointValue::Time(0),
            },
            _ if str.starts_with("wait=") => {
                let sig = decode_signal(&str[5..])?;
                self.blocked_signals.add(signal::Signal::from_c_int(sig).unwrap());
                CheckpointAction {
                    opcode: CheckpointOpcode::Wait,
                    value: CheckpointValue::Signal(sig),
                }
            }
            _ => return Err(format!("{}: unknown checkpoint action", str)),
        };

        self.actions.push_back(action);
        Ok(())
    }

    fn finish_compile(&mut self) -> Result<(), String> {
        if self.state == CheckpointState::Init && self.actions.is_empty() {
            self.compile_action("echo")?;
        }

        if self.state == CheckpointState::Compile {
            signal::sigprocmask(signal::SigmaskHow::SIG_BLOCK, Some(&self.blocked_signals), None)
                .map_err(|e| e.to_string())?;
            self.state = CheckpointState::Run;
        }
        Ok(())
    }

    fn run_actions(&mut self, do_write: bool) -> io::Result<()> {
        for action in &self.actions {
            match action.opcode {
                CheckpointOpcode::Dot => {
                    std::io::stdout().write_all(b".")?;
                    std::io::stdout().flush()?;
                }
                CheckpointOpcode::Bell => {
                    if self.tty.is_none() {
                        self.tty = Some(File::create("/dev/tty")?);
                    }
                    if let Some(tty) = &mut self.tty {
                        tty.write_all(b"\x07")?;
                        tty.flush()?;
                    }
                }
                CheckpointOpcode::Echo => {
                    let n = write!(io::stderr(), "{}: ", program_name())?;
                    format_checkpoint_string(
                        &mut io::stderr(),
                        n,
                        match &action.value {
                            CheckpointValue::Command(cmd) => Some(cmd),
                            _ => None,
                        },
                        do_write,
                        self.checkpoint.load(Ordering::Relaxed),
                    )?;
                    writeln!(io::stderr())?;
                }
                CheckpointOpcode::TtyOut => {
                    if self.tty.is_none() {
                        self.tty = Some(File::create("/dev/tty")?);
                    }
                    if let Some(tty) = &mut self.tty {
                        format_checkpoint_string(
                            tty,
                            0,
                            match &action.value {
                                CheckpointValue::Command(cmd) => Some(cmd),
                                _ => None,
                            },
                            do_write,
                            self.checkpoint.load(Ordering::Relaxed),
                        )?;
                    }
                }
                CheckpointOpcode::Sleep => {
                    if let CheckpointValue::Time(secs) = action.value {
                        std::thread::sleep(std::time::Duration::from_secs(secs));
                    }
                }
                CheckpointOpcode::Exec => {
                    if let CheckpointValue::Command(cmd) = &action.value {
                        sys_exec_checkpoint_script(cmd, archive_name_cursor(), self.checkpoint.load(Ordering::Relaxed))?;
                    }
                }
                CheckpointOpcode::Totals => {
                    print_total_stats();
                }
                CheckpointOpcode::Wait => {
                    if let CheckpointValue::Signal(sig) = action.value {
                        let mut sig = 0;
                        signal::sigwait(&self.blocked_signals, &mut sig)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn flush_actions(&mut self) -> io::Result<()> {
        if self.tty_cleanup {
            if let Some(tty) = &mut self.tty {
                let width = get_width(tty)?;
                for _ in 0..width {
                    tty.write_all(b" ")?;
                }
                tty.write_all(b"\r")?;
                tty.flush()?;
            }
        }
        Ok(())
    }

    fn run(&mut self, do_write: bool) -> io::Result<()> {
        let checkpoint = self.checkpoint.fetch_add(1, Ordering::Relaxed) + 1;
        if checkpoint % checkpoint_option() == 0 {
            self.run_actions(do_write)?;
        }
        Ok(())
    }

    fn finish(&mut self) -> io::Result<()> {
        if checkpoint_option() > 0 {
            self.flush_actions()?;
            if let Some(tty) = self.tty.take() {
                drop(tty);
            }
        }
        Ok(())
    }
}

fn unquote_string(s: &str) -> String {
    let mut s = s.to_string();
    if s.len() >= 2 && (s.starts_with('"') || s.starts_with('\'')) && s.ends_with(s.chars().next().unwrap()) {
        s.pop();
        s.remove(0);
    }
    s
}

fn decode_signal(s: &str) -> Result<i32, String> {
    // Signal decoding logic would go here
    Ok(0)
}

fn get_width(file: &File) -> io::Result<usize> {
    unsafe {
        let mut winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if libc::ioctl(file.as_raw_fd(), TIOCGWINSZ, &mut winsize) == 0 && winsize.ws_col > 0 {
            return Ok(winsize.ws_col as usize);
        }
    }

    if let Ok(cols) = env::var("COLUMNS") {
        if let Ok(col) = usize::from_str(&cols) {
            if col > 0 {
                return Ok(col);
            }
        }
    }

    Ok(80)
}

fn format_checkpoint_string(
    file: &mut dyn Write,
    mut len: usize,
    input: Option<&str>,
    do_write: bool,
    checkpoint: usize,
) -> io::Result<usize> {
    let opstr = if do_write {
        "write"
    } else {
        "read"
    };
    let input = match input {
        Some(s) => s,
        None => {
            if do_write {
                "Write checkpoint {}"
            } else {
                "Read checkpoint {}"
            }
        }
    };

    let mut arg = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some('{') = chars.peek() {
                chars.next();
                arg.clear();
                while let Some(c) = chars.next() {
                    if c == '}' {
                        break;
                    }
                    arg.push(c);
                }
            }
            match chars.next() {
                Some('c') => {
                    len = format_checkpoint_string(file, len, Some(DEFAULT_FORMAT), do_write, checkpoint)?;
                }
                Some('u') => {
                    let s = checkpoint.to_string();
                    file.write_all(s.as_bytes())?;
                    len += s.len();
                }
                Some('s') => {
                    file.write_all(opstr.as_bytes())?;
                    len += opstr.len();
                }
                Some('d') => {
                    let duration = compute_duration();
                    write!(file, "{:.0}", duration)?;
                    len += format!("{:.0}", duration).len();
                }
                Some('T') => {
                    compute_duration();
                    len += format_total_stats(file, &CHECKPOINT_TOTAL_FORMAT, ',', 0)?;
                }
                Some('t') => {
                    let fmt = if arg.is_empty() { "%c" } else { &arg };
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap();
                    let tm = time::now();
                    len += fprintftime(file, fmt, &tm, now.as_secs(), now.subsec_nanos() / 1000)?;
                }
                Some('*') => {
                    let width = if arg.is_empty() {
                        get_width(&file)?
                    } else {
                        usize::from_str(&arg).unwrap_or(80)
                    };
                    for _ in len..width {
                        file.write_all(b" ")?;
                        len += 1;
                    }
                }
                Some(c) => {
                    file.write_all(&[b'%', c as u8])?;
                    len += 2;
                }
                None => {
                    file.write_all(b"%")?;
                    len += 1;
                }
            }
            arg.clear();
        } else {
            file.write_all(&[c as u8])?;
            if c == '\r' {
                len = 0;
                // tty_cleanup = true;
            } else {
                len += 1;
            }
        }
    }
    file.flush()?;
    Ok(len)
}

const DEFAULT_FORMAT: &str = "%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r";
const CHECKPOINT_TOTAL_FORMAT: [&str; 3] = ["R", "W", "D"];

fn program_name() -> &'static str {
    "tar"
}

fn checkpoint_option() -> usize {
    // Would return the checkpoint option value
    0
}

fn archive_name_cursor() -> &'static str {
    ""
}

fn compute_duration() -> f64 {
    // Duration computation logic
    0.0
}

fn print_total_stats() {
    // Print total stats logic
}

fn format_total_stats(
    _file: &mut dyn Write,
    _formats: &[&str],
    _delim: char,
    _flags: u32,
) -> io::Result<usize> {
    // Format total stats logic
    Ok(0)
}

fn fprintftime(
    _file: &mut dyn Write,
    _fmt: &str,
    _tm: &time::Tm,
    _sec: u64,
    _usec: u32,
) -> io::Result<usize> {
    // Time formatting logic
    Ok(0)
}

fn sys_exec_checkpoint_script(_cmd: &str, _archive: &str, _checkpoint: usize) -> io::Result<()> {
    // Script execution logic
    Ok(())
}