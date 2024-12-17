#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum uerr_t {
    METALINK_SIZE_ERROR = 62,
    RETR_WITH_METALINK = 61,
    METALINK_MISSING_RESOURCE = 60,
    METALINK_SIG_ERROR = 59,
    METALINK_CHKSUM_ERROR = 58,
    METALINK_RETR_ERROR = 57,
    METALINK_PARSE_ERROR = 56,
    TIMECONV_ERR = 55,
    WARC_TMP_FWRITEERR = 54,
    WARC_TMP_FOPENERR = 53,
    WARC_ERR = 52,
    UNKNOWNATTR = 51,
    ATTRMISSING = 50,
    CLOSEFAILED = 49,
    NEWLOCATION_KEEP_POST = 48,
    UNLINKERR = 47,
    VERIFCERTERR = 46,
    SSLINITFAILED = 45,
    WRITEFAILED = 44,
    QUOTEXC = 43,
    AUTHFAILED = 42,
    PROXERR = 41,
    RETRBADPATTERN = 40,
    RANGEERR = 39,
    FILEBADFILE = 38,
    TRYLIMEXC = 37,
    READERR = 36,
    RETRFINISHED = 35,
    RETRUNNEEDED = 34,
    CONTNOTSUPPORTED = 33,
    FTPNOAUTH = 32,
    FTPNOPROT = 31,
    FTPNOPBSZ = 30,
    FTPNOPASV = 29,
    FTPINVPASV = 28,
    WRONGCODE = 27,
    RECLEVELEXC = 26,
    RETROK = 25,
    HERR = 24,
    GATEWAYTIMEOUT = 23,
    HEOF = 22,
    FWRITEERR = 21,
    FOPEN_EXCL_ERR = 20,
    FOPENERR = 19,
    URLERROR = 18,
    FTPRESTFAIL = 17,
    FTPRETRINT = 16,
    FTPSRVERR = 15,
    FTPRERR = 14,
    FTPUNKNOWNTYPE = 13,
    FTPNSFOD = 12,
    FTPSYSERR = 11,
    FTPPORTERR = 10,
    FTPLOGREFUSED = 9,
    FTPLOGINC = 8,
    FTPOK = 7,
    NEWLOCATION = 6,
    CONIMPOSSIBLE = 5,
    CONSSLERR = 4,
    CONERROR = 3,
    CONSOCKERR = 2,
    HOSTERR = 1,
    NOCONERROR = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    WGET_EXIT_UNKNOWN = 9,
    WGET_EXIT_SERVER_ERROR = 8,
    WGET_EXIT_PROTOCOL_ERROR = 7,
    WGET_EXIT_SERVER_AUTH_FAIL = 6,
    WGET_EXIT_SSL_AUTH_FAIL = 5,
    WGET_EXIT_NETWORK_FAIL = 4,
    WGET_EXIT_IO_FAIL = 3,
    WGET_EXIT_PARSE_ERROR = 2,
    WGET_EXIT_GENERIC_ERROR = 1,
    WGET_EXIT_SUCCESS = 0,
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
