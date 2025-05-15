/*  Copyright 1997,2001,2002,2007-2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fs::File;
use std::io::{self, Read, Write, stdin, Stdin};
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use libc::{c_int, c_void, signal, SIGALRM, SIG_IGN};
use nix::sys::termios;
use nix::sys::termios::{Termios, tcgetattr, tcsetattr, SetArg, LocalFlags};
use std::ffi::CString;
use std::ptr;
use std::mem;

static mut TTY: Option<File> = None;
static NOTTY: AtomicBool = AtomicBool::new(false);
static mut TTYFD: RawFd = -1;
static MTLS_RAW_TTY: AtomicBool = AtomicBool::new(false);

static mut TTY_MODE: i32 = -1; // 1 for raw, 0 for cooked, -1 for initial
static NEED_TTY_RESET: AtomicBool = AtomicBool::new(false);
static HANDLER_IS_SET: AtomicBool = AtomicBool::new(false);

static mut IN_ORIG: Option<Termios> = None;

extern "C" fn tty_time_out(_dummy: c_int) -> ! {
    unsafe {
        signal(SIGALRM, SIG_IGN);
        if TTY.is_some() && NEED_TTY_RESET.load(Ordering::SeqCst) {
            if let Some(orig) = &IN_ORIG {
                tcsetattr(TTYFD, SetArg::TCSANOW, orig).unwrap();
            }
        }
        std::process::exit(0);
    }
}

fn cleanup_tty() {
    unsafe {
        if TTY.is_some() && NEED_TTY_RESET.load(Ordering::SeqCst) {
            if let Some(orig) = &IN_ORIG {
                tcsetattr(TTYFD, SetArg::TCSANOW, orig).unwrap();
            }
        }
    }
}

fn set_raw_tty(mode: i32) -> io::Result<()> {
    unsafe {
        if mode != TTY_MODE && mode != -1 {
            if !HANDLER_IS_SET.load(Ordering::SeqCst) {
                let orig = tcgetattr(TTYFD)?;
                IN_ORIG = Some(orig);
                NEED_TTY_RESET.store(true, Ordering::SeqCst);
                
                libc::atexit(cleanup_tty);
                HANDLER_IS_SET.store(true, Ordering::SeqCst);
            }

            let mut raw = tcgetattr(TTYFD)?;
            if mode != 0 {
                raw.local_flags &= !LocalFlags::ICANON;
                raw.control_chars[nix::sys::termios::SpecialCharacterIndices::VMIN as usize] = 1;
                raw.control_chars[nix::sys::termios::SpecialCharacterIndices::VTIME as usize] = 0;
                tcsetattr(TTYFD, SetArg::TCSANOW, &raw)?;
            } else {
                raw.local_flags |= LocalFlags::ICANON;
                tcsetattr(TTYFD, SetArg::TCSANOW, &raw)?;
            }
            TTY_MODE = mode;
        }
        Ok(())
    }
}

fn opentty(mode: i32) -> io::Result<Option<&'static File>> {
    unsafe {
        if NOTTY.load(Ordering::SeqCst) {
            return Ok(None);
        }
        
        if TTY.is_none() {
            let tty_file = File::open("/dev/tty")?;
            TTYFD = tty_file.as_raw_fd();
            TTY = Some(tty_file);
        }
        
        if TTY.is_none() {
            if nix::unistd::isatty(0).unwrap_or(false) {
                TTYFD = 0;
                TTY = Some(stdin().try_clone()?);
            } else {
                NOTTY.store(true, Ordering::SeqCst);
                return Ok(None);
            }
        }
        
        if MTLS_RAW_TTY.load(Ordering::SeqCst) {
            set_raw_tty(mode)?;
        }
        
        Ok(TTY.as_ref())
    }
}

fn ask_confirmation(format: &str, args: std::fmt::Arguments) -> io::Result<bool> {
    let mut ans = [0u8; 10];
    
    let tty = match opentty(-1)? {
        Some(t) => t,
        None => return Ok(false),
    };
    
    loop {
        eprint!("{}", format);
        io::stderr().flush()?;
        tty.flush()?;
        
        if MTLS_RAW_TTY.load(Ordering::SeqCst) {
            let mut buf = [0u8; 1];
            tty.read_exact(&mut buf)?;
            ans[0] = buf[0];
            eprintln!();
        } else {
            let mut buf = String::new();
            tty.read_line(&mut buf)?;
            if buf.len() > 0 {
                ans[0..1].copy_from_slice(&buf.as_bytes()[0..1]);
            } else {
                ans[0] = b'n';
            }
        }
        
        match ans[0] {
            b'y' | b'Y' => return Ok(true),
            b'n' | b'N' => return Ok(false),
            _ => continue,
        }
    }
}