#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum uerr_t {
    NOCONERROR,
    HOSTERR,
    CONSOCKERR,
    CONERROR,
    CONSSLERR,
    CONIMPOSSIBLE,
    NEWLOCATION,
    FTPOK,
    FTPLOGINC,
    FTPLOGREFUSED,
    FTPPORTERR,
    FTPSYSERR,
    FTPNSFOD,
    FTPUNKNOWNTYPE,
    FTPRERR,
    FTPSRVERR,
    FTPRETRINT,
    FTPRESTFAIL,
    URLERROR,
    FOPENERR,
    FOPEN_EXCL_ERR,
    FWRITEERR,
    HEOF,
    GATEWAYTIMEOUT,
    HERR,
    RETROK,
    RECLEVELEXC,
    WRONGCODE,
    FTPINVPASV,
    FTPNOPASV,
    FTPNOPBSZ,
    FTPNOPROT,
    FTPNOAUTH,
    CONTNOTSUPPORTED,
    RETRUNNEEDED,
    RETRFINISHED,
    READERR,
    TRYLIMEXC,
    FILEBADFILE,
    RANGEERR,
    RETRBADPATTERN,
    PROXERR,
    AUTHFAILED,
    QUOTEXC,
    WRITEFAILED,
    SSLINITFAILED,
    VERIFCERTERR,
    UNLINKERR,
    NEWLOCATION_KEEP_POST,
    CLOSEFAILED,
    ATTRMISSING,
    UNKNOWNATTR,
    WARC_ERR,
    WARC_TMP_FOPENERR,
    WARC_TMP_FWRITEERR,
    TIMECONV_ERR,
    METALINK_PARSE_ERROR,
    METALINK_RETR_ERROR,
    METALINK_CHKSUM_ERROR,
    METALINK_SIG_ERROR,
    METALINK_MISSING_RESOURCE,
    RETR_WITH_METALINK,
    METALINK_SIZE_ERROR,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    WGET_EXIT_SUCCESS = 0,
    WGET_EXIT_GENERIC_ERROR = 1,
    WGET_EXIT_PARSE_ERROR = 2,
    WGET_EXIT_IO_FAIL = 3,
    WGET_EXIT_NETWORK_FAIL = 4,
    WGET_EXIT_SSL_AUTH_FAIL = 5,
    WGET_EXIT_SERVER_AUTH_FAIL = 6,
    WGET_EXIT_PROTOCOL_ERROR = 7,
    WGET_EXIT_SERVER_ERROR = 8,
    WGET_EXIT_UNKNOWN,
}  // end of enum

static mut final_exit_status: libc::c_int = WGET_EXIT_SUCCESS as libc::c_int;
unsafe extern "C" fn get_status_for_err(mut err: uerr_t) -> libc::c_int {
    match err as libc::c_uint {
        25 => return WGET_EXIT_SUCCESS as libc::c_int,
        19 | 20 | 21 | 44 | 47 | 49 | 38 => return WGET_EXIT_IO_FAIL as libc::c_int,
        0 | 1 | 2 | 3 | 4 | 5 | 14 | 28 | 36 | 37 => {
            return WGET_EXIT_NETWORK_FAIL as libc::c_int;
        }
        46 => return WGET_EXIT_SSL_AUTH_FAIL as libc::c_int,
        8 | 9 | 42 => return WGET_EXIT_SERVER_AUTH_FAIL as libc::c_int,
        22 | 24 | 50 => return WGET_EXIT_PROTOCOL_ERROR as libc::c_int,
        27 | 10 | 11 | 12 | 13 | 15 | 16 | 17 | 29 | 33 | 39 | 40 | 41 | 23 => {
            return WGET_EXIT_SERVER_ERROR as libc::c_int;
        }
        18 | 43 | 45 | 51 | _ => return WGET_EXIT_UNKNOWN as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn inform_exit_status(mut err: uerr_t) {
    let mut new_status: libc::c_int = get_status_for_err(err);
    if new_status != WGET_EXIT_SUCCESS as libc::c_int
        && (final_exit_status == WGET_EXIT_SUCCESS as libc::c_int
            || new_status < final_exit_status)
    {
        final_exit_status = new_status;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_exit_status() -> libc::c_int {
    return if final_exit_status == WGET_EXIT_UNKNOWN as libc::c_int {
        1 as libc::c_int
    } else {
        final_exit_status
    };
}
