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
impl uerr_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            uerr_t::NOCONERROR => 0,
            uerr_t::HOSTERR => 1,
            uerr_t::CONSOCKERR => 2,
            uerr_t::CONERROR => 3,
            uerr_t::CONSSLERR => 4,
            uerr_t::CONIMPOSSIBLE => 5,
            uerr_t::NEWLOCATION => 6,
            uerr_t::FTPOK => 7,
            uerr_t::FTPLOGINC => 8,
            uerr_t::FTPLOGREFUSED => 9,
            uerr_t::FTPPORTERR => 10,
            uerr_t::FTPSYSERR => 11,
            uerr_t::FTPNSFOD => 12,
            uerr_t::FTPUNKNOWNTYPE => 13,
            uerr_t::FTPRERR => 14,
            uerr_t::FTPSRVERR => 15,
            uerr_t::FTPRETRINT => 16,
            uerr_t::FTPRESTFAIL => 17,
            uerr_t::URLERROR => 18,
            uerr_t::FOPENERR => 19,
            uerr_t::FOPEN_EXCL_ERR => 20,
            uerr_t::FWRITEERR => 21,
            uerr_t::HEOF => 22,
            uerr_t::GATEWAYTIMEOUT => 23,
            uerr_t::HERR => 24,
            uerr_t::RETROK => 25,
            uerr_t::RECLEVELEXC => 26,
            uerr_t::WRONGCODE => 27,
            uerr_t::FTPINVPASV => 28,
            uerr_t::FTPNOPASV => 29,
            uerr_t::FTPNOPBSZ => 30,
            uerr_t::FTPNOPROT => 31,
            uerr_t::FTPNOAUTH => 32,
            uerr_t::CONTNOTSUPPORTED => 33,
            uerr_t::RETRUNNEEDED => 34,
            uerr_t::RETRFINISHED => 35,
            uerr_t::READERR => 36,
            uerr_t::TRYLIMEXC => 37,
            uerr_t::FILEBADFILE => 38,
            uerr_t::RANGEERR => 39,
            uerr_t::RETRBADPATTERN => 40,
            uerr_t::PROXERR => 41,
            uerr_t::AUTHFAILED => 42,
            uerr_t::QUOTEXC => 43,
            uerr_t::WRITEFAILED => 44,
            uerr_t::SSLINITFAILED => 45,
            uerr_t::VERIFCERTERR => 46,
            uerr_t::UNLINKERR => 47,
            uerr_t::NEWLOCATION_KEEP_POST => 48,
            uerr_t::CLOSEFAILED => 49,
            uerr_t::ATTRMISSING => 50,
            uerr_t::UNKNOWNATTR => 51,
            uerr_t::WARC_ERR => 52,
            uerr_t::WARC_TMP_FOPENERR => 53,
            uerr_t::WARC_TMP_FWRITEERR => 54,
            uerr_t::TIMECONV_ERR => 55,
            uerr_t::METALINK_PARSE_ERROR => 56,
            uerr_t::METALINK_RETR_ERROR => 57,
            uerr_t::METALINK_CHKSUM_ERROR => 58,
            uerr_t::METALINK_SIG_ERROR => 59,
            uerr_t::METALINK_MISSING_RESOURCE => 60,
            uerr_t::RETR_WITH_METALINK => 61,
            uerr_t::METALINK_SIZE_ERROR => 62,
        }
    }
}

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
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::WGET_EXIT_SUCCESS => 0,
            C2RustUnnamed::WGET_EXIT_GENERIC_ERROR => 1,
            C2RustUnnamed::WGET_EXIT_PARSE_ERROR => 2,
            C2RustUnnamed::WGET_EXIT_IO_FAIL => 3,
            C2RustUnnamed::WGET_EXIT_NETWORK_FAIL => 4,
            C2RustUnnamed::WGET_EXIT_SSL_AUTH_FAIL => 5,
            C2RustUnnamed::WGET_EXIT_SERVER_AUTH_FAIL => 6,
            C2RustUnnamed::WGET_EXIT_PROTOCOL_ERROR => 7,
            C2RustUnnamed::WGET_EXIT_SERVER_ERROR => 8,
            C2RustUnnamed::WGET_EXIT_UNKNOWN => 9,
        }
    }
}

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
