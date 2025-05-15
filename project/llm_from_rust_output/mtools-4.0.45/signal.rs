use nix::sys::signal::{self, SigHandler, Signal};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigSet};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};

static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);

fn signal_handler(_: Signal) {
    GOT_SIGNAL.store(true, Ordering::SeqCst);
}

pub fn setup_signal() {
    let handler = SigHandler::Handler(signal_handler);
    let flags = SaFlags::empty();
    let mask = SigSet::empty();
    let action = SigAction::new(handler, flags, mask);

    for sig in &[Signal::SIGHUP, Signal::SIGINT, Signal::SIGTERM, Signal::SIGQUIT] {
        unsafe {
            sigaction(*sig, &action).unwrap_or_else(|e| {
                eprintln!("sigaction failed: {}", e);
                exit(1);
            });
        }
    }
}

pub struct SavedSigState {
    sa: [SigAction; 4],
}

impl SavedSigState {
    pub fn new() -> Self {
        SavedSigState {
            sa: [
                SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty()),
                SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty()),
                SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty()),
                SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty()),
            ],
        }
    }

    pub fn allow_interrupts(&mut self) {
        let handler = SigHandler::Handler(signal_handler);
        let flags = SaFlags::empty();
        let mask = SigSet::empty();
        let action = SigAction::new(handler, flags, mask);

        for i in 0..4 {
            unsafe {
                self.sa[i] = sigaction(Signal::SIGINT, &action).unwrap_or_else(|e| {
                    eprintln!("sigaction failed: {}", e);
                    exit(1);
                });
            }
        }
    }

    pub fn restore_interrupts(&self) {
        for i in 0..4 {
            unsafe {
                sigaction(Signal::SIGINT, &self.sa[i]).unwrap_or_else(|e| {
                    eprintln!("restore sigaction failed: {}", e);
                    exit(1);
                });
            }
        }
    }
}