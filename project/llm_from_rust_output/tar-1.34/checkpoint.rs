use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Write};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

type TimeT = c_long;
type SizeT = c_ulong;
type UIntMaxT = c_ulong;

#[derive(Debug)]
struct TimeVal {
    tv_sec: TimeT,
    tv_usec: c_long,
}

#[derive(Debug)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

#[derive(Debug)]
struct WinSize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

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
    Time(TimeT),
    Command(CString),
    Signal(c_int),
}

#[derive(Debug)]
struct CheckpointAction {
    next: Option<Box<CheckpointAction>>,
    opcode: CheckpointOpcode,
    value: CheckpointValue,
}

static mut CHECKPOINT: c_uint = 0;
static mut CHECKPOINT_ACTION: Option<Box<CheckpointAction>> = None;
static mut CHECKPOINT_ACTION_TAIL: Option<*mut CheckpointAction> = None;
static mut CHECKPOINT_STATE: c_int = 0;
static mut SIGS: sigset_t = sigset_t { __val: [0; 16] };

fn alloc_action(opcode: CheckpointOpcode) -> Box<CheckpointAction> {
    let action = Box::new(CheckpointAction {
        next: None,
        opcode,
        value: CheckpointValue::Time(0),
    });

    unsafe {
        if let Some(tail) = CHECKPOINT_ACTION_TAIL {
            (*tail).next = Some(action.clone());
        } else {
            CHECKPOINT_ACTION = Some(action.clone());
        }
        CHECKPOINT_ACTION_TAIL = Some(Box::into_raw(action));
    }

    action
}

fn copy_string_unquote(str: &CStr) -> CString {
    let mut output = str.to_owned();
    let len = output.to_bytes().len();

    if (output.to_bytes()[0] == b'"' || output.to_bytes()[0] == b'\'') 
        && output.to_bytes()[len - 1] == output.to_bytes()[0] 
    {
        output = CString::new(&output.to_bytes()[1..len - 1]).unwrap();
    }

    output
}

fn checkpoint_compile_action(str: &CStr) {
    unsafe {
        if CHECKPOINT_STATE == CHKP_INIT {
            sigemptyset(&mut SIGS);
            CHECKPOINT_STATE = CHKP_COMPILE;
        }

        if str == CStr::from_bytes_with_nul(b".\0").unwrap() 
            || str == CStr::from_bytes_with_nul(b"dot\0").unwrap() 
        {
            alloc_action(CheckpointOpcode::Dot);
        } else if str == CStr::from_bytes_with_nul(b"bell\0").unwrap() {
            alloc_action(CheckpointOpcode::Bell);
        } else if str == CStr::from_bytes_with_nul(b"echo\0").unwrap() {
            alloc_action(CheckpointOpcode::Echo);
        } else if str.to_bytes().starts_with(b"echo=") {
            let act = alloc_action(CheckpointOpcode::Echo);
            act.value = CheckpointValue::Command(copy_string_unquote(
                CStr::from_bytes_with_nul(&str.to_bytes()[5..]).unwrap()
            ));
        } else if str.to_bytes().starts_with(b"exec=") {
            let act = alloc_action(CheckpointOpcode::Exec);
            act.value = CheckpointValue::Command(copy_string_unquote(
                CStr::from_bytes_with_nul(&str.to_bytes()[5..]).unwrap()
            ));
        } else if str.to_bytes().starts_with(b"ttyout=") {
            let act = alloc_action(CheckpointOpcode::TtyOut);
            act.value = CheckpointValue::Command(copy_string_unquote(
                CStr::from_bytes_with_nul(&str.to_bytes()[7..]).unwrap()
            ));
        } else if str.to_bytes().starts_with(b"sleep=") {
            let n = str.to_bytes()[6..].iter()
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, c| acc * 10 + (c - b'0') as TimeT);
            
            let act = alloc_action(CheckpointOpcode::Sleep);
            act.value = CheckpointValue::Time(n);
        } else if str == CStr::from_bytes_with_nul(b"totals\0").unwrap() {
            alloc_action(CheckpointOpcode::Totals);
        } else if str.to_bytes().starts_with(b"wait=") {
            let signal = decode_signal(CStr::from_bytes_with_nul(&str.to_bytes()[5..]).unwrap();
            let act = alloc_action(CheckpointOpcode::Wait);
            act.value = CheckpointValue::Signal(signal);
            sigaddset(&mut SIGS, signal);
        } else {
            error(0, 0, "unknown checkpoint action");
            fatal_exit();
        }
    }
}

fn checkpoint_finish_compile() {
    unsafe {
        if CHECKPOINT_STATE == CHKP_INIT && CHECKPOINT_OPTION != 0 && CHECKPOINT_ACTION.is_none() {
            checkpoint_compile_action(CStr::from_bytes_with_nul(b"echo\0").unwrap());
        }

        if CHECKPOINT_STATE == CHKP_COMPILE {
            sigprocmask(0, &SIGS, ptr::null_mut());
            if CHECKPOINT_OPTION == 0 {
                CHECKPOINT_OPTION = 10;
            }
            CHECKPOINT_STATE = CHKP_RUN;
        }
    }
}

fn run_checkpoint_actions(do_write: bool) {
    unsafe {
        let mut p = CHECKPOINT_ACTION.as_ref();
        while let Some(action) = p {
            match action.opcode {
                CheckpointOpcode::Dot => {
                    stdlis.write_all(b".").unwrap();
                    stdlis.flush().unwrap();
                }
                CheckpointOpcode::Bell => {
                    if TTY.is_null() {
                        TTY = fopen(b"/dev/tty\0".as_ptr(), b"w\0".as_ptr());
                    }
                    if !TTY.is_null() {
                        TTY.write_all(&[7]).unwrap();
                        TTY.flush().unwrap();
                    }
                }
                CheckpointOpcode::Echo => {
                    let n = fprintf(stderr, b"%s: \0".as_ptr(), PROGRAM_NAME.as_ptr());
                    format_checkpoint_string(
                        stderr,
                        n as SizeT,
                        match &action.value {
                            CheckpointValue::Command(cmd) => cmd.as_ptr(),
                            _ => ptr::null(),
                        },
                        do_write,
                        CHECKPOINT,
                    );
                    stderr.write_all(b"\n").unwrap();
                }
                CheckpointOpcode::TtyOut => {
                    if TTY.is_null() {
                        TTY = fopen(b"/dev/tty\0".as_ptr(), b"w\0".as_ptr());
                    }
                    if !TTY.is_null() {
                        format_checkpoint_string(
                            TTY,
                            0,
                            match &action.value {
                                CheckpointValue::Command(cmd) => cmd.as_ptr(),
                                _ => ptr::null(),
                            },
                            do_write,
                            CHECKPOINT,
                        );
                    }
                }
                CheckpointOpcode::Sleep => {
                    if let CheckpointValue::Time(secs) = action.value {
                        std::thread::sleep(std::time::Duration::from_secs(secs as u64));
                    }
                }
                CheckpointOpcode::Exec => {
                    if let CheckpointValue::Command(cmd) = &action.value {
                        sys_exec_checkpoint_script(
                            cmd.as_ptr(),
                            ARCHIVE_NAME_CURSOR[0],
                            CHECKPOINT as c_int,
                        );
                    }
                }
                CheckpointOpcode::Totals => {
                    compute_duration();
                    print_total_stats();
                }
                CheckpointOpcode::Wait => {
                    if let CheckpointValue::Signal(sig) = action.value {
                        let mut n = 0;
                        sigwait(&SIGS, &mut n);
                    }
                }
            }
            p = action.next.as_ref();
        }
    }
}

fn checkpoint_run(do_write: bool) {
    unsafe {
        if CHECKPOINT_OPTION != 0 {
            CHECKPOINT += 1;
            if CHECKPOINT % CHECKPOINT_OPTION == 0 {
                run_checkpoint_actions(do_write);
            }
        }
    }
}

fn checkpoint_finish() {
    unsafe {
        if CHECKPOINT_OPTION != 0 {
            checkpoint_flush_actions();
            if !TTY.is_null() {
                fclose(TTY);
            }
        }
    }
}