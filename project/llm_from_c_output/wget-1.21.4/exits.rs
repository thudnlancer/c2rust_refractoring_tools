//! Exit status handling for GNU Wget.
//!
//! This module provides functionality for managing exit status codes in a way
//! compatible with GNU Wget's behavior.

use std::sync::atomic::{AtomicI32, Ordering};

/// Exit code possibilities.
/// Exit codes 1 and 2 are reserved for situations that lead to direct exits,
/// not using the value of the final exit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitStatus {
    Success = 0,
    GenericError = 1,
    ParseError = 2,
    IoFail = 3,
    NetworkFail = 4,
    SslAuthFail = 5,
    ServerAuthFail = 6,
    ProtocolError = 7,
    ServerError = 8,
    Unknown,
}

impl From<ExitStatus> for i32 {
    fn from(status: ExitStatus) -> Self {
        status as i32
    }
}

/// Type alias for error codes (uerr_t in original C code)
pub type UError = i32;

// Original C error codes (simplified for Rust)
pub const RETROK: UError = 0;
pub const FOPENERR: UError = 1;
pub const FOPEN_EXCL_ERR: UError = 2;
pub const FWRITEERR: UError = 3;
pub const WRITEFAILED: UError = 4;
pub const UNLINKERR: UError = 5;
pub const CLOSEFAILED: UError = 6;
pub const FILEBADFILE: UError = 7;
pub const NOCONERROR: UError = 8;
pub const HOSTERR: UError = 9;
pub const CONSOCKERR: UError = 10;
pub const CONERROR: UError = 11;
pub const CONSSLERR: UError = 12;
pub const CONIMPOSSIBLE: UError = 13;
pub const FTPRERR: UError = 14;
pub const FTPINVPASV: UError = 15;
pub const READERR: UError = 16;
pub const TRYLIMEXC: UError = 17;
pub const VERIFCERTERR: UError = 18;
pub const FTPLOGINC: UError = 19;
pub const FTPLOGREFUSED: UError = 20;
pub const AUTHFAILED: UError = 21;
pub const HEOF: UError = 22;
pub const HERR: UError = 23;
pub const ATTRMISSING: UError = 24;
pub const WRONGCODE: UError = 25;
pub const FTPPORTERR: UError = 26;
pub const FTPSYSERR: UError = 27;
pub const FTPNSFOD: UError = 28;
pub const FTPUNKNOWNTYPE: UError = 29;
pub const FTPSRVERR: UError = 30;
pub const FTPRETRINT: UError = 31;
pub const FTPRESTFAIL: UError = 32;
pub const FTPNOPASV: UError = 33;
pub const CONTNOTSUPPORTED: UError = 34;
pub const RANGEERR: UError = 35;
pub const RETRBADPATTERN: UError = 36;
pub const PROXERR: UError = 37;
pub const GATEWAYTIMEOUT: UError = 38;
pub const URLERROR: UError = 39;
pub const QUOTEXC: UError = 40;
pub const SSLINITFAILED: UError = 41;
pub const UNKNOWNATTR: UError = 42;

static FINAL_EXIT_STATUS: AtomicI32 = AtomicI32::new(ExitStatus::Success as i32);

/// Maps error codes to exit statuses
fn get_status_for_err(err: UError) -> ExitStatus {
    match err {
        RETROK => ExitStatus::Success,
        FOPENERR | FOPEN_EXCL_ERR | FWRITEERR | WRITEFAILED | UNLINKERR | CLOSEFAILED | FILEBADFILE => {
            ExitStatus::IoFail
        }
        NOCONERROR | HOSTERR | CONSOCKERR | CONERROR | CONSSLERR | CONIMPOSSIBLE | FTPRERR
        | FTPINVPASV | READERR | TRYLIMEXC => ExitStatus::NetworkFail,
        VERIFCERTERR => ExitStatus::SslAuthFail,
        FTPLOGINC | FTPLOGREFUSED | AUTHFAILED => ExitStatus::ServerAuthFail,
        HEOF | HERR | ATTRMISSING => ExitStatus::ProtocolError,
        WRONGCODE | FTPPORTERR | FTPSYSERR | FTPNSFOD | FTPUNKNOWNTYPE | FTPSRVERR | FTPRETRINT
        | FTPRESTFAIL | FTPNOPASV | CONTNOTSUPPORTED | RANGEERR | RETRBADPATTERN | PROXERR
        | GATEWAYTIMEOUT => ExitStatus::ServerError,
        URLERROR | QUOTEXC | SSLINITFAILED | UNKNOWNATTR | _ => ExitStatus::Unknown,
    }
}

/// Updates the exit status to reflect the given error, unless a more severe
/// error has already been recorded.
pub fn inform_exit_status(err: UError) {
    let new_status = get_status_for_err(err);
    
    FINAL_EXIT_STATUS.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
        let current_status = ExitStatus::from(current);
        if new_status != ExitStatus::Success
            && (current_status == ExitStatus::Success || (new_status as i32) < current)
        {
            Some(new_status as i32)
        } else {
            None
        }
    }).ok();
}

/// Retrieves the current exit status.
/// Returns 1 (GenericError) if the status is Unknown.
pub fn get_exit_status() -> i32 {
    let status = FINAL_EXIT_STATUS.load(Ordering::SeqCst);
    if ExitStatus::from(status) == ExitStatus::Unknown {
        1
    } else {
        status
    }
}

impl From<i32> for ExitStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => ExitStatus::Success,
            1 => ExitStatus::GenericError,
            2 => ExitStatus::ParseError,
            3 => ExitStatus::IoFail,
            4 => ExitStatus::NetworkFail,
            5 => ExitStatus::SslAuthFail,
            6 => ExitStatus::ServerAuthFail,
            7 => ExitStatus::ProtocolError,
            8 => ExitStatus::ServerError,
            _ => ExitStatus::Unknown,
        }
    }
}