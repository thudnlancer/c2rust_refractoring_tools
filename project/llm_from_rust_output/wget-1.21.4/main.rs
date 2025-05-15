use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::process;

// Define necessary types and constants
type Wgint = i64;
type UerrT = u32;
type UrlScheme = u32;

const SCHEME_HTTP: UrlScheme = 0;
const SCHEME_HTTPS: UrlScheme = 1;
const SCHEME_FTP: UrlScheme = 2;
const SCHEME_FTPS: UrlScheme = 3;
const SCHEME_INVALID: UrlScheme = 4;

const WGET_EXIT_SUCCESS: c_int = 0;
const WGET_EXIT_GENERIC_ERROR: c_int = 1;
