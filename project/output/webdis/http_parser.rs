#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type size_t = u64;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uchar", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uchar", bits = "2..=7")]
    pub type_0_flags: [u8; 1],
    pub state: u8,
    pub header_state: u8,
    pub index: u8,
    pub nread: uint32_t,
    pub content_length: int64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    pub status_code: libc::c_ushort,
    pub method: u8,
    pub upgrade: i8,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_path: http_data_cb,
    pub on_query_string: http_data_cb,
    pub on_url: http_data_cb,
    pub on_fragment: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
}
pub type http_cb = Option<unsafe extern "C" fn(*mut http_parser) -> i32>;
pub type http_data_cb = Option<
    unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum http_method {
    HTTP_DELETE = 0,
    HTTP_GET,
    HTTP_HEAD,
    HTTP_POST,
    HTTP_PUT,
    HTTP_CONNECT,
    HTTP_OPTIONS,
    HTTP_TRACE,
    HTTP_COPY,
    HTTP_LOCK,
    HTTP_MKCOL,
    HTTP_MOVE,
    HTTP_PROPFIND,
    HTTP_PROPPATCH,
    HTTP_UNLOCK,
    HTTP_REPORT,
    HTTP_MKACTIVITY,
    HTTP_CHECKOUT,
    HTTP_MERGE,
    HTTP_MSEARCH,
    HTTP_NOTIFY,
    HTTP_SUBSCRIBE,
    HTTP_UNSUBSCRIBE,
}
impl http_method {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            http_method::HTTP_DELETE => 0,
            http_method::HTTP_GET => 1,
            http_method::HTTP_HEAD => 2,
            http_method::HTTP_POST => 3,
            http_method::HTTP_PUT => 4,
            http_method::HTTP_CONNECT => 5,
            http_method::HTTP_OPTIONS => 6,
            http_method::HTTP_TRACE => 7,
            http_method::HTTP_COPY => 8,
            http_method::HTTP_LOCK => 9,
            http_method::HTTP_MKCOL => 10,
            http_method::HTTP_MOVE => 11,
            http_method::HTTP_PROPFIND => 12,
            http_method::HTTP_PROPPATCH => 13,
            http_method::HTTP_UNLOCK => 14,
            http_method::HTTP_REPORT => 15,
            http_method::HTTP_MKACTIVITY => 16,
            http_method::HTTP_CHECKOUT => 17,
            http_method::HTTP_MERGE => 18,
            http_method::HTTP_MSEARCH => 19,
            http_method::HTTP_NOTIFY => 20,
            http_method::HTTP_SUBSCRIBE => 21,
            http_method::HTTP_UNSUBSCRIBE => 22,
        }
    }
    fn from_libc_c_uint(value: u32) -> http_method {
        match value {
            0 => http_method::HTTP_DELETE,
            1 => http_method::HTTP_GET,
            2 => http_method::HTTP_HEAD,
            3 => http_method::HTTP_POST,
            4 => http_method::HTTP_PUT,
            5 => http_method::HTTP_CONNECT,
            6 => http_method::HTTP_OPTIONS,
            7 => http_method::HTTP_TRACE,
            8 => http_method::HTTP_COPY,
            9 => http_method::HTTP_LOCK,
            10 => http_method::HTTP_MKCOL,
            11 => http_method::HTTP_MOVE,
            12 => http_method::HTTP_PROPFIND,
            13 => http_method::HTTP_PROPPATCH,
            14 => http_method::HTTP_UNLOCK,
            15 => http_method::HTTP_REPORT,
            16 => http_method::HTTP_MKACTIVITY,
            17 => http_method::HTTP_CHECKOUT,
            18 => http_method::HTTP_MERGE,
            19 => http_method::HTTP_MSEARCH,
            20 => http_method::HTTP_NOTIFY,
            21 => http_method::HTTP_SUBSCRIBE,
            22 => http_method::HTTP_UNSUBSCRIBE,
            _ => panic!("Invalid value for http_method: {}", value),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum http_parser_type {
    HTTP_REQUEST,
    HTTP_RESPONSE,
    HTTP_BOTH,
}
impl http_parser_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            http_parser_type::HTTP_REQUEST => 0,
            http_parser_type::HTTP_RESPONSE => 1,
            http_parser_type::HTTP_BOTH => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> http_parser_type {
        match value {
            0 => http_parser_type::HTTP_REQUEST,
            1 => http_parser_type::HTTP_RESPONSE,
            2 => http_parser_type::HTTP_BOTH,
            _ => panic!("Invalid value for http_parser_type: {}", value),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flags {
    F_CHUNKED = 1,
    F_CONNECTION_KEEP_ALIVE = 2,
    F_CONNECTION_CLOSE = 4,
    F_TRAILING = 8,
    F_UPGRADE = 16,
    F_SKIPBODY = 32,
}
impl flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            flags::F_CHUNKED => 1,
            flags::F_CONNECTION_KEEP_ALIVE => 2,
            flags::F_CONNECTION_CLOSE => 4,
            flags::F_TRAILING => 8,
            flags::F_UPGRADE => 16,
            flags::F_SKIPBODY => 32,
        }
    }
    fn from_libc_c_uint(value: u32) -> flags {
        match value {
            1 => flags::F_CHUNKED,
            2 => flags::F_CONNECTION_KEEP_ALIVE,
            4 => flags::F_CONNECTION_CLOSE,
            8 => flags::F_TRAILING,
            16 => flags::F_UPGRADE,
            32 => flags::F_SKIPBODY,
            _ => panic!("Invalid value for flags: {}", value),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum state {
    s_dead = 1,
    s_start_req_or_res,
    s_res_or_resp_H,
    s_start_res,
    s_res_H,
    s_res_HT,
    s_res_HTT,
    s_res_HTTP,
    s_res_first_http_major,
    s_res_http_major,
    s_res_first_http_minor,
    s_res_http_minor,
    s_res_first_status_code,
    s_res_status_code,
    s_res_status,
    s_res_line_almost_done,
    s_start_req,
    s_req_method,
    s_req_spaces_before_url,
    s_req_schema,
    s_req_schema_slash,
    s_req_schema_slash_slash,
    s_req_host,
    s_req_port,
    s_req_path,
    s_req_query_string_start,
    s_req_query_string,
    s_req_fragment_start,
    s_req_fragment,
    s_req_http_start,
    s_req_http_H,
    s_req_http_HT,
    s_req_http_HTT,
    s_req_http_HTTP,
    s_req_first_http_major,
    s_req_http_major,
    s_req_first_http_minor,
    s_req_http_minor,
    s_req_line_almost_done,
    s_header_field_start,
    s_header_field,
    s_header_value_start,
    s_header_value,
    s_header_almost_done,
    s_headers_almost_done,
    s_chunk_size_start,
    s_chunk_size,
    s_chunk_size_almost_done,
    s_chunk_parameters,
    s_chunk_data,
    s_chunk_data_almost_done,
    s_chunk_data_done,
    s_body_identity,
    s_body_identity_eof,
}
impl state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            state::s_dead => 1,
            state::s_start_req_or_res => 2,
            state::s_res_or_resp_H => 3,
            state::s_start_res => 4,
            state::s_res_H => 5,
            state::s_res_HT => 6,
            state::s_res_HTT => 7,
            state::s_res_HTTP => 8,
            state::s_res_first_http_major => 9,
            state::s_res_http_major => 10,
            state::s_res_first_http_minor => 11,
            state::s_res_http_minor => 12,
            state::s_res_first_status_code => 13,
            state::s_res_status_code => 14,
            state::s_res_status => 15,
            state::s_res_line_almost_done => 16,
            state::s_start_req => 17,
            state::s_req_method => 18,
            state::s_req_spaces_before_url => 19,
            state::s_req_schema => 20,
            state::s_req_schema_slash => 21,
            state::s_req_schema_slash_slash => 22,
            state::s_req_host => 23,
            state::s_req_port => 24,
            state::s_req_path => 25,
            state::s_req_query_string_start => 26,
            state::s_req_query_string => 27,
            state::s_req_fragment_start => 28,
            state::s_req_fragment => 29,
            state::s_req_http_start => 30,
            state::s_req_http_H => 31,
            state::s_req_http_HT => 32,
            state::s_req_http_HTT => 33,
            state::s_req_http_HTTP => 34,
            state::s_req_first_http_major => 35,
            state::s_req_http_major => 36,
            state::s_req_first_http_minor => 37,
            state::s_req_http_minor => 38,
            state::s_req_line_almost_done => 39,
            state::s_header_field_start => 40,
            state::s_header_field => 41,
            state::s_header_value_start => 42,
            state::s_header_value => 43,
            state::s_header_almost_done => 44,
            state::s_headers_almost_done => 45,
            state::s_chunk_size_start => 46,
            state::s_chunk_size => 47,
            state::s_chunk_size_almost_done => 48,
            state::s_chunk_parameters => 49,
            state::s_chunk_data => 50,
            state::s_chunk_data_almost_done => 51,
            state::s_chunk_data_done => 52,
            state::s_body_identity => 53,
            state::s_body_identity_eof => 54,
        }
    }
    fn from_libc_c_uint(value: u32) -> state {
        match value {
            1 => state::s_dead,
            2 => state::s_start_req_or_res,
            3 => state::s_res_or_resp_H,
            4 => state::s_start_res,
            5 => state::s_res_H,
            6 => state::s_res_HT,
            7 => state::s_res_HTT,
            8 => state::s_res_HTTP,
            9 => state::s_res_first_http_major,
            10 => state::s_res_http_major,
            11 => state::s_res_first_http_minor,
            12 => state::s_res_http_minor,
            13 => state::s_res_first_status_code,
            14 => state::s_res_status_code,
            15 => state::s_res_status,
            16 => state::s_res_line_almost_done,
            17 => state::s_start_req,
            18 => state::s_req_method,
            19 => state::s_req_spaces_before_url,
            20 => state::s_req_schema,
            21 => state::s_req_schema_slash,
            22 => state::s_req_schema_slash_slash,
            23 => state::s_req_host,
            24 => state::s_req_port,
            25 => state::s_req_path,
            26 => state::s_req_query_string_start,
            27 => state::s_req_query_string,
            28 => state::s_req_fragment_start,
            29 => state::s_req_fragment,
            30 => state::s_req_http_start,
            31 => state::s_req_http_H,
            32 => state::s_req_http_HT,
            33 => state::s_req_http_HTT,
            34 => state::s_req_http_HTTP,
            35 => state::s_req_first_http_major,
            36 => state::s_req_http_major,
            37 => state::s_req_first_http_minor,
            38 => state::s_req_http_minor,
            39 => state::s_req_line_almost_done,
            40 => state::s_header_field_start,
            41 => state::s_header_field,
            42 => state::s_header_value_start,
            43 => state::s_header_value,
            44 => state::s_header_almost_done,
            45 => state::s_headers_almost_done,
            46 => state::s_chunk_size_start,
            47 => state::s_chunk_size,
            48 => state::s_chunk_size_almost_done,
            49 => state::s_chunk_parameters,
            50 => state::s_chunk_data,
            51 => state::s_chunk_data_almost_done,
            52 => state::s_chunk_data_done,
            53 => state::s_body_identity,
            54 => state::s_body_identity_eof,
            _ => panic!("Invalid value for state: {}", value),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum header_states {
    h_general = 0,
    h_C,
    h_CO,
    h_CON,
    h_matching_connection,
    h_matching_proxy_connection,
    h_matching_content_length,
    h_matching_transfer_encoding,
    h_matching_upgrade,
    h_connection,
    h_content_length,
    h_transfer_encoding,
    h_upgrade,
    h_matching_transfer_encoding_chunked,
    h_matching_connection_keep_alive,
    h_matching_connection_close,
    h_transfer_encoding_chunked,
    h_connection_keep_alive,
    h_connection_close,
}
impl header_states {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            header_states::h_general => 0,
            header_states::h_C => 1,
            header_states::h_CO => 2,
            header_states::h_CON => 3,
            header_states::h_matching_connection => 4,
            header_states::h_matching_proxy_connection => 5,
            header_states::h_matching_content_length => 6,
            header_states::h_matching_transfer_encoding => 7,
            header_states::h_matching_upgrade => 8,
            header_states::h_connection => 9,
            header_states::h_content_length => 10,
            header_states::h_transfer_encoding => 11,
            header_states::h_upgrade => 12,
            header_states::h_matching_transfer_encoding_chunked => 13,
            header_states::h_matching_connection_keep_alive => 14,
            header_states::h_matching_connection_close => 15,
            header_states::h_transfer_encoding_chunked => 16,
            header_states::h_connection_keep_alive => 17,
            header_states::h_connection_close => 18,
        }
    }
    fn from_libc_c_uint(value: u32) -> header_states {
        match value {
            0 => header_states::h_general,
            1 => header_states::h_C,
            2 => header_states::h_CO,
            3 => header_states::h_CON,
            4 => header_states::h_matching_connection,
            5 => header_states::h_matching_proxy_connection,
            6 => header_states::h_matching_content_length,
            7 => header_states::h_matching_transfer_encoding,
            8 => header_states::h_matching_upgrade,
            9 => header_states::h_connection,
            10 => header_states::h_content_length,
            11 => header_states::h_transfer_encoding,
            12 => header_states::h_upgrade,
            13 => header_states::h_matching_transfer_encoding_chunked,
            14 => header_states::h_matching_connection_keep_alive,
            15 => header_states::h_matching_connection_close,
            16 => header_states::h_transfer_encoding_chunked,
            17 => header_states::h_connection_keep_alive,
            18 => header_states::h_connection_close,
            _ => panic!("Invalid value for header_states: {}", value),
        }
    }
}
static mut method_strings: [*const i8; 23] = [
    b"DELETE\0" as *const u8 as *const i8,
    b"GET\0" as *const u8 as *const i8,
    b"HEAD\0" as *const u8 as *const i8,
    b"POST\0" as *const u8 as *const i8,
    b"PUT\0" as *const u8 as *const i8,
    b"CONNECT\0" as *const u8 as *const i8,
    b"OPTIONS\0" as *const u8 as *const i8,
    b"TRACE\0" as *const u8 as *const i8,
    b"COPY\0" as *const u8 as *const i8,
    b"LOCK\0" as *const u8 as *const i8,
    b"MKCOL\0" as *const u8 as *const i8,
    b"MOVE\0" as *const u8 as *const i8,
    b"PROPFIND\0" as *const u8 as *const i8,
    b"PROPPATCH\0" as *const u8 as *const i8,
    b"UNLOCK\0" as *const u8 as *const i8,
    b"REPORT\0" as *const u8 as *const i8,
    b"MKACTIVITY\0" as *const u8 as *const i8,
    b"CHECKOUT\0" as *const u8 as *const i8,
    b"MERGE\0" as *const u8 as *const i8,
    b"M-SEARCH\0" as *const u8 as *const i8,
    b"NOTIFY\0" as *const u8 as *const i8,
    b"SUBSCRIBE\0" as *const u8 as *const i8,
    b"UNSUBSCRIBE\0" as *const u8 as *const i8,
];
static mut tokens: [i8; 256] = [
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    ' ' as i32 as i8,
    '!' as i32 as i8,
    '"' as i32 as i8,
    '#' as i32 as i8,
    '$' as i32 as i8,
    '%' as i32 as i8,
    '&' as i32 as i8,
    '\'' as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    '*' as i32 as i8,
    '+' as i32 as i8,
    0 as i32 as i8,
    '-' as i32 as i8,
    '.' as i32 as i8,
    '/' as i32 as i8,
    '0' as i32 as i8,
    '1' as i32 as i8,
    '2' as i32 as i8,
    '3' as i32 as i8,
    '4' as i32 as i8,
    '5' as i32 as i8,
    '6' as i32 as i8,
    '7' as i32 as i8,
    '8' as i32 as i8,
    '9' as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    'a' as i32 as i8,
    'b' as i32 as i8,
    'c' as i32 as i8,
    'd' as i32 as i8,
    'e' as i32 as i8,
    'f' as i32 as i8,
    'g' as i32 as i8,
    'h' as i32 as i8,
    'i' as i32 as i8,
    'j' as i32 as i8,
    'k' as i32 as i8,
    'l' as i32 as i8,
    'm' as i32 as i8,
    'n' as i32 as i8,
    'o' as i32 as i8,
    'p' as i32 as i8,
    'q' as i32 as i8,
    'r' as i32 as i8,
    's' as i32 as i8,
    't' as i32 as i8,
    'u' as i32 as i8,
    'v' as i32 as i8,
    'w' as i32 as i8,
    'x' as i32 as i8,
    'y' as i32 as i8,
    'z' as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    '^' as i32 as i8,
    '_' as i32 as i8,
    '`' as i32 as i8,
    'a' as i32 as i8,
    'b' as i32 as i8,
    'c' as i32 as i8,
    'd' as i32 as i8,
    'e' as i32 as i8,
    'f' as i32 as i8,
    'g' as i32 as i8,
    'h' as i32 as i8,
    'i' as i32 as i8,
    'j' as i32 as i8,
    'k' as i32 as i8,
    'l' as i32 as i8,
    'm' as i32 as i8,
    'n' as i32 as i8,
    'o' as i32 as i8,
    'p' as i32 as i8,
    'q' as i32 as i8,
    'r' as i32 as i8,
    's' as i32 as i8,
    't' as i32 as i8,
    'u' as i32 as i8,
    'v' as i32 as i8,
    'w' as i32 as i8,
    'x' as i32 as i8,
    'y' as i32 as i8,
    'z' as i32 as i8,
    0 as i32 as i8,
    '|' as i32 as i8,
    '}' as i32 as i8,
    '~' as i32 as i8,
    0 as i32 as i8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut unhex: [int8_t; 256] = [
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    0 as i32 as int8_t,
    1 as i32 as int8_t,
    2 as i32 as int8_t,
    3 as i32 as int8_t,
    4 as i32 as int8_t,
    5 as i32 as int8_t,
    6 as i32 as int8_t,
    7 as i32 as int8_t,
    8 as i32 as int8_t,
    9 as i32 as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    10 as i32 as int8_t,
    11 as i32 as int8_t,
    12 as i32 as int8_t,
    13 as i32 as int8_t,
    14 as i32 as int8_t,
    15 as i32 as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    10 as i32 as int8_t,
    11 as i32 as int8_t,
    12 as i32 as int8_t,
    13 as i32 as int8_t,
    14 as i32 as int8_t,
    15 as i32 as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    -(1 as i32) as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut normal_url_char: [uint8_t; 256] = [
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    0 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    0 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn http_parser_execute(
    mut parser: *mut http_parser,
    mut settings: *const http_parser_settings,
    mut data: *const i8,
    mut len: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut c: i8 = 0;
    let mut ch: i8 = 0;
    let mut p: *const i8 = data;
    let mut pe: *const i8 = 0 as *const i8;
    let mut to_read: int64_t = 0;
    let mut state: state = (*parser).state as state;
    let mut header_state: header_states = (*parser).header_state as header_states;
    let mut index: uint64_t = (*parser).index as uint64_t;
    let mut nread: uint64_t = (*parser).nread as uint64_t;
    if len == 0 as i32 as u64 {
        if state as u32 == state::s_body_identity_eof as i32 as u32 {
            if ((*settings).on_message_complete).is_some() {
                if 0 as i32
                    != ((*settings).on_message_complete)
                        .expect("non-null function pointer")(parser)
                {
                    return p.offset_from(data) as i64 as size_t;
                }
            }
        }
        return 0 as i32 as size_t;
    }
    let mut header_field_mark: *const i8 = 0 as *const i8;
    let mut header_value_mark: *const i8 = 0 as *const i8;
    let mut fragment_mark: *const i8 = 0 as *const i8;
    let mut query_string_mark: *const i8 = 0 as *const i8;
    let mut path_mark: *const i8 = 0 as *const i8;
    let mut url_mark: *const i8 = 0 as *const i8;
    if state as u32 == state::s_header_field as i32 as u32 {
        header_field_mark = data;
    }
    if state as u32 == state::s_header_value as i32 as u32 {
        header_value_mark = data;
    }
    if state as u32 == state::s_req_fragment as i32 as u32 {
        fragment_mark = data;
    }
    if state as u32 == state::s_req_query_string as i32 as u32 {
        query_string_mark = data;
    }
    if state as u32 == state::s_req_path as i32 as u32 {
        path_mark = data;
    }
    if state as u32 == state::s_req_path as i32 as u32
        || state as u32 == state::s_req_schema as i32 as u32
        || state as u32 == state::s_req_schema_slash as i32 as u32
        || state as u32 == state::s_req_schema_slash_slash as i32 as u32
        || state as u32 == state::s_req_port as i32 as u32
        || state as u32 == state::s_req_query_string_start as i32 as u32
        || state as u32 == state::s_req_query_string as i32 as u32
        || state as u32 == state::s_req_host as i32 as u32
        || state as u32 == state::s_req_fragment_start as i32 as u32
        || state as u32 == state::s_req_fragment as i32 as u32
    {
        url_mark = data;
    }
    p = data;
    pe = data.offset(len as isize);
    loop {
        if !(p != pe) {
            current_block = 10962706839408844679;
            break;
        }
        ch = *p;
        if state as u32 <= state::s_headers_almost_done as i32 as u32
            && 0 as i32 == (*parser).flags() as i32 & flags::F_TRAILING as i32
        {
            nread = nread.wrapping_add(1);
            nread;
            if nread > (80 as i32 * 1024 as i32) as u64 {
                current_block = 5716811153459378242;
                break;
            }
        }
        match state as u32 {
            1 => {
                current_block = 5716811153459378242;
                break;
            }
            2 => {
                if ch as i32 == '\r' as i32 || ch as i32 == '\n' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    (*parser).set_flags(0 as i32 as u8);
                    (*parser).content_length = -(1 as i32) as int64_t;
                    if ((*settings).on_message_begin).is_some() {
                        if 0 as i32
                            != ((*settings).on_message_begin)
                                .expect("non-null function pointer")(parser)
                        {
                            return p.offset_from(data) as i64 as size_t;
                        }
                    }
                    if ch as i32 == 'H' as i32 {
                        state = state::s_res_or_resp_H;
                        current_block = 13472856163611868459;
                    } else {
                        (*parser)
                            .set_type_0(http_parser_type::HTTP_REQUEST as i32 as u8);
                        current_block = 3415824759247930212;
                    }
                }
            }
            3 => {
                if ch as i32 == 'T' as i32 {
                    (*parser).set_type_0(http_parser_type::HTTP_RESPONSE as i32 as u8);
                    state = state::s_res_HT;
                } else {
                    if ch as i32 != 'E' as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    (*parser).set_type_0(http_parser_type::HTTP_REQUEST as i32 as u8);
                    (*parser).method = http_method::HTTP_HEAD as i32 as u8;
                    index = 2 as i32 as uint64_t;
                    state = state::s_req_method;
                }
                current_block = 13472856163611868459;
            }
            4 => {
                (*parser).set_flags(0 as i32 as u8);
                (*parser).content_length = -(1 as i32) as int64_t;
                if ((*settings).on_message_begin).is_some() {
                    if 0 as i32
                        != ((*settings).on_message_begin)
                            .expect("non-null function pointer")(parser)
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
                match ch as i32 {
                    72 => {
                        state = state::s_res_H;
                        current_block = 13472856163611868459;
                    }
                    13 | 10 => {
                        current_block = 13472856163611868459;
                    }
                    _ => {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
            }
            5 => {
                if ch as i32 != 'T' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_res_HT;
                current_block = 13472856163611868459;
            }
            6 => {
                if ch as i32 != 'T' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_res_HTT;
                current_block = 13472856163611868459;
            }
            7 => {
                if ch as i32 != 'P' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_res_HTTP;
                current_block = 13472856163611868459;
            }
            8 => {
                if ch as i32 != '/' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_res_first_http_major;
                current_block = 13472856163611868459;
            }
            9 => {
                if (ch as i32) < '1' as i32 || ch as i32 > '9' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                (*parser).http_major = (ch as i32 - '0' as i32) as libc::c_ushort;
                state = state::s_res_http_major;
                current_block = 13472856163611868459;
            }
            10 => {
                if ch as i32 == '.' as i32 {
                    state = state::s_res_first_http_minor;
                } else {
                    if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    (*parser).http_major = ((*parser).http_major as i32 * 10 as i32)
                        as libc::c_ushort;
                    (*parser).http_major = ((*parser).http_major as i32
                        + (ch as i32 - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_major as i32 > 999 as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            11 => {
                if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                (*parser).http_minor = (ch as i32 - '0' as i32) as libc::c_ushort;
                state = state::s_res_http_minor;
                current_block = 13472856163611868459;
            }
            12 => {
                if ch as i32 == ' ' as i32 {
                    state = state::s_res_first_status_code;
                } else {
                    if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    (*parser).http_minor = ((*parser).http_minor as i32 * 10 as i32)
                        as libc::c_ushort;
                    (*parser).http_minor = ((*parser).http_minor as i32
                        + (ch as i32 - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_minor as i32 > 999 as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            13 => {
                if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                    if !(ch as i32 == ' ' as i32) {
                        current_block = 5716811153459378242;
                        break;
                    }
                } else {
                    (*parser).status_code = (ch as i32 - '0' as i32) as libc::c_ushort;
                    state = state::s_res_status_code;
                }
                current_block = 13472856163611868459;
            }
            14 => {
                if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                    match ch as i32 {
                        32 => {
                            state = state::s_res_status;
                        }
                        13 => {
                            state = state::s_res_line_almost_done;
                        }
                        10 => {
                            state = state::s_header_field_start;
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                } else {
                    (*parser).status_code = ((*parser).status_code as i32 * 10 as i32)
                        as libc::c_ushort;
                    (*parser).status_code = ((*parser).status_code as i32
                        + (ch as i32 - '0' as i32)) as libc::c_ushort;
                    if (*parser).status_code as i32 > 999 as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            15 => {
                if ch as i32 == '\r' as i32 {
                    state = state::s_res_line_almost_done;
                } else if ch as i32 == '\n' as i32 {
                    state = state::s_header_field_start;
                }
                current_block = 13472856163611868459;
            }
            16 => {
                if ch as i32 != '\n' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_header_field_start;
                current_block = 13472856163611868459;
            }
            17 => {
                if ch as i32 == '\r' as i32 || ch as i32 == '\n' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    (*parser).set_flags(0 as i32 as u8);
                    (*parser).content_length = -(1 as i32) as int64_t;
                    if ((*settings).on_message_begin).is_some() {
                        if 0 as i32
                            != ((*settings).on_message_begin)
                                .expect("non-null function pointer")(parser)
                        {
                            return p.offset_from(data) as i64 as size_t;
                        }
                    }
                    if (ch as i32) < 'A' as i32 || ('Z' as i32) < ch as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    current_block = 3415824759247930212;
                }
            }
            18 => {
                if ch as i32 == '\0' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                let mut matcher: *const i8 = method_strings[(*parser).method as usize];
                if ch as i32 == ' ' as i32
                    && *matcher.offset(index as isize) as i32 == '\0' as i32
                {
                    state = state::s_req_spaces_before_url;
                } else if !(ch as i32 == *matcher.offset(index as isize) as i32) {
                    if (*parser).method as i32 == http_method::HTTP_CONNECT as i32 {
                        if index == 1 as i32 as u64 && ch as i32 == 'H' as i32 {
                            (*parser).method = http_method::HTTP_CHECKOUT as i32 as u8;
                        } else if index == 2 as i32 as u64 && ch as i32 == 'P' as i32 {
                            (*parser).method = http_method::HTTP_COPY as i32 as u8;
                        }
                    } else if (*parser).method as i32 == http_method::HTTP_MKCOL as i32 {
                        if index == 1 as i32 as u64 && ch as i32 == 'O' as i32 {
                            (*parser).method = http_method::HTTP_MOVE as i32 as u8;
                        } else if index == 1 as i32 as u64 && ch as i32 == 'E' as i32 {
                            (*parser).method = http_method::HTTP_MERGE as i32 as u8;
                        } else if index == 1 as i32 as u64 && ch as i32 == '-' as i32 {
                            (*parser).method = http_method::HTTP_MSEARCH as i32 as u8;
                        } else if index == 2 as i32 as u64 && ch as i32 == 'A' as i32 {
                            (*parser).method = http_method::HTTP_MKACTIVITY as i32 as u8;
                        }
                    } else if index == 1 as i32 as u64
                        && (*parser).method as i32 == http_method::HTTP_POST as i32
                        && ch as i32 == 'R' as i32
                    {
                        (*parser).method = http_method::HTTP_PROPFIND as i32 as u8;
                    } else if index == 1 as i32 as u64
                        && (*parser).method as i32 == http_method::HTTP_POST as i32
                        && ch as i32 == 'U' as i32
                    {
                        (*parser).method = http_method::HTTP_PUT as i32 as u8;
                    } else if index == 2 as i32 as u64
                        && (*parser).method as i32 == http_method::HTTP_UNLOCK as i32
                        && ch as i32 == 'S' as i32
                    {
                        (*parser).method = http_method::HTTP_UNSUBSCRIBE as i32 as u8;
                    } else {
                        if !(index == 4 as i32 as u64
                            && (*parser).method as i32
                                == http_method::HTTP_PROPFIND as i32
                            && ch as i32 == 'P' as i32)
                        {
                            current_block = 5716811153459378242;
                            break;
                        }
                        (*parser).method = http_method::HTTP_PROPPATCH as i32 as u8;
                    }
                }
                index = index.wrapping_add(1);
                index;
                current_block = 13472856163611868459;
            }
            19 => {
                if ch as i32 == ' ' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    if ch as i32 == '/' as i32 || ch as i32 == '*' as i32 {
                        url_mark = p;
                        path_mark = p;
                        state = state::s_req_path;
                    } else {
                        c = (ch as i32 | 0x20 as i32) as u8 as i8;
                        if !(c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32) {
                            current_block = 5716811153459378242;
                            break;
                        }
                        url_mark = p;
                        state = state::s_req_schema;
                    }
                    current_block = 13472856163611868459;
                }
            }
            20 => {
                c = (ch as i32 | 0x20 as i32) as u8 as i8;
                if c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    if ch as i32 == ':' as i32 {
                        state = state::s_req_schema_slash;
                    } else if ch as i32 == '.' as i32 {
                        state = state::s_req_host;
                    } else {
                        if !('0' as i32 <= ch as i32 && ch as i32 <= '9' as i32) {
                            current_block = 5716811153459378242;
                            break;
                        }
                        state = state::s_req_host;
                    }
                    current_block = 13472856163611868459;
                }
            }
            21 => {
                if ch as i32 != '/' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_req_schema_slash_slash;
                current_block = 13472856163611868459;
            }
            22 => {
                if ch as i32 != '/' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_req_host;
                current_block = 13472856163611868459;
            }
            23 => {
                c = (ch as i32 | 0x20 as i32) as u8 as i8;
                if c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32 {
                    current_block = 13472856163611868459;
                } else if ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32
                    || ch as i32 == '.' as i32 || ch as i32 == '-' as i32
                {
                    current_block = 13472856163611868459;
                } else {
                    match ch as i32 {
                        58 => {
                            state = state::s_req_port;
                        }
                        47 => {
                            path_mark = p;
                            state = state::s_req_path;
                        }
                        32 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_url)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const i8;
                            state = state::s_req_http_start;
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            24 => {
                if ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as i32 {
                        47 => {
                            path_mark = p;
                            state = state::s_req_path;
                        }
                        32 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_url)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const i8;
                            state = state::s_req_http_start;
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            25 => {
                if normal_url_char[ch as u8 as usize] != 0 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as i32 {
                        32 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_url)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const i8;
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_path)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const i8;
                            state = state::s_req_http_start;
                        }
                        13 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_url)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const i8;
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_path)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const i8;
                            (*parser).http_major = 0 as i32 as libc::c_ushort;
                            (*parser).http_minor = 9 as i32 as libc::c_ushort;
                            state = state::s_req_line_almost_done;
                        }
                        10 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_url)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const i8;
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_path)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const i8;
                            (*parser).http_major = 0 as i32 as libc::c_ushort;
                            (*parser).http_minor = 9 as i32 as libc::c_ushort;
                            state = state::s_header_field_start;
                        }
                        63 => {
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_path)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const i8;
                            state = state::s_req_query_string_start;
                        }
                        35 => {
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as i32
                                        != ((*settings).on_path)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as i64 as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as i64 as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const i8;
                            state = state::s_req_fragment_start;
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            26 => {
                if normal_url_char[ch as u8 as usize] != 0 {
                    query_string_mark = p;
                    state = state::s_req_query_string;
                } else {
                    match ch as i32 {
                        63 => {}
                        32 => {
                            current_block = 7514558797810173966;
                            match current_block {
                                6677089456533421798 => {
                                    state = state::s_req_fragment_start;
                                }
                                12190775748064332689 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                7514558797810173966 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        13 => {
                            current_block = 12190775748064332689;
                            match current_block {
                                6677089456533421798 => {
                                    state = state::s_req_fragment_start;
                                }
                                12190775748064332689 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                7514558797810173966 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        10 => {
                            current_block = 14647551731762175849;
                            match current_block {
                                6677089456533421798 => {
                                    state = state::s_req_fragment_start;
                                }
                                12190775748064332689 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                7514558797810173966 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        35 => {
                            current_block = 6677089456533421798;
                            match current_block {
                                6677089456533421798 => {
                                    state = state::s_req_fragment_start;
                                }
                                12190775748064332689 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                7514558797810173966 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                }
                current_block = 13472856163611868459;
            }
            27 => {
                if normal_url_char[ch as u8 as usize] != 0 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as i32 {
                        63 => {
                            current_block = 13472856163611868459;
                        }
                        32 => {
                            current_block = 16530171831619307892;
                            match current_block {
                                15549159295339167397 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_fragment_start;
                                }
                                13407439525415555836 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                16530171831619307892 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        13 => {
                            current_block = 13407439525415555836;
                            match current_block {
                                15549159295339167397 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_fragment_start;
                                }
                                13407439525415555836 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                16530171831619307892 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        10 => {
                            current_block = 6886367987138400562;
                            match current_block {
                                15549159295339167397 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_fragment_start;
                                }
                                13407439525415555836 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                16530171831619307892 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        35 => {
                            current_block = 15549159295339167397;
                            match current_block {
                                15549159295339167397 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_fragment_start;
                                }
                                13407439525415555836 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                16530171831619307892 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_query_string)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                }
            }
            28 => {
                if normal_url_char[ch as u8 as usize] != 0 {
                    fragment_mark = p;
                    state = state::s_req_fragment;
                } else {
                    match ch as i32 {
                        32 => {
                            current_block = 18165338285857167437;
                            match current_block {
                                13435336783582280770 => {
                                    fragment_mark = p;
                                    state = state::s_req_fragment;
                                }
                                11004849488906185852 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                18165338285857167437 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        13 => {
                            current_block = 11004849488906185852;
                            match current_block {
                                13435336783582280770 => {
                                    fragment_mark = p;
                                    state = state::s_req_fragment;
                                }
                                11004849488906185852 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                18165338285857167437 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        10 => {
                            current_block = 13424006242428025653;
                            match current_block {
                                13435336783582280770 => {
                                    fragment_mark = p;
                                    state = state::s_req_fragment;
                                }
                                11004849488906185852 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                18165338285857167437 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        63 => {
                            current_block = 13435336783582280770;
                            match current_block {
                                13435336783582280770 => {
                                    fragment_mark = p;
                                    state = state::s_req_fragment;
                                }
                                11004849488906185852 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                18165338285857167437 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                        }
                        35 => {}
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                }
                current_block = 13472856163611868459;
            }
            29 => {
                if normal_url_char[ch as u8 as usize] != 0 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as i32 {
                        32 => {
                            current_block = 10213527007967979201;
                            match current_block {
                                10213527007967979201 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                2721638304048114319 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        13 => {
                            current_block = 2721638304048114319;
                            match current_block {
                                10213527007967979201 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                2721638304048114319 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        10 => {
                            current_block = 2209868826789542165;
                            match current_block {
                                10213527007967979201 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    state = state::s_req_http_start;
                                }
                                2721638304048114319 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_req_line_almost_done;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_url)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const i8;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as i32
                                                != ((*settings).on_fragment)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as i64 as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as i64 as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const i8;
                                    (*parser).http_major = 0 as i32 as libc::c_ushort;
                                    (*parser).http_minor = 9 as i32 as libc::c_ushort;
                                    state = state::s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        63 | 35 => {
                            current_block = 13472856163611868459;
                        }
                        _ => {
                            current_block = 5716811153459378242;
                            break;
                        }
                    }
                }
            }
            30 => {
                match ch as i32 {
                    72 => {
                        state = state::s_req_http_H;
                        current_block = 13472856163611868459;
                    }
                    32 => {
                        current_block = 13472856163611868459;
                    }
                    _ => {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
            }
            31 => {
                if ch as i32 != 'T' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_req_http_HT;
                current_block = 13472856163611868459;
            }
            32 => {
                if ch as i32 != 'T' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_req_http_HTT;
                current_block = 13472856163611868459;
            }
            33 => {
                if ch as i32 != 'P' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_req_http_HTTP;
                current_block = 13472856163611868459;
            }
            34 => {
                if ch as i32 != '/' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_req_first_http_major;
                current_block = 13472856163611868459;
            }
            35 => {
                if (ch as i32) < '1' as i32 || ch as i32 > '9' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                (*parser).http_major = (ch as i32 - '0' as i32) as libc::c_ushort;
                state = state::s_req_http_major;
                current_block = 13472856163611868459;
            }
            36 => {
                if ch as i32 == '.' as i32 {
                    state = state::s_req_first_http_minor;
                } else {
                    if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    (*parser).http_major = ((*parser).http_major as i32 * 10 as i32)
                        as libc::c_ushort;
                    (*parser).http_major = ((*parser).http_major as i32
                        + (ch as i32 - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_major as i32 > 999 as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            37 => {
                if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                (*parser).http_minor = (ch as i32 - '0' as i32) as libc::c_ushort;
                state = state::s_req_http_minor;
                current_block = 13472856163611868459;
            }
            38 => {
                if ch as i32 == '\r' as i32 {
                    state = state::s_req_line_almost_done;
                } else if ch as i32 == '\n' as i32 {
                    state = state::s_header_field_start;
                } else {
                    if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    (*parser).http_minor = ((*parser).http_minor as i32 * 10 as i32)
                        as libc::c_ushort;
                    (*parser).http_minor = ((*parser).http_minor as i32
                        + (ch as i32 - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_minor as i32 > 999 as i32 {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            39 => {
                if ch as i32 != '\n' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_header_field_start;
                current_block = 13472856163611868459;
            }
            40 => {
                if ch as i32 == '\r' as i32 {
                    state = state::s_headers_almost_done;
                    current_block = 13472856163611868459;
                } else if ch as i32 == '\n' as i32 {
                    state = state::s_headers_almost_done;
                    current_block = 6436225509474489267;
                } else {
                    c = tokens[ch as u8 as usize];
                    if c == 0 {
                        current_block = 5716811153459378242;
                        break;
                    }
                    header_field_mark = p;
                    index = 0 as i32 as uint64_t;
                    state = state::s_header_field;
                    match c as i32 {
                        99 => {
                            header_state = header_states::h_C;
                        }
                        112 => {
                            header_state = header_states::h_matching_proxy_connection;
                        }
                        116 => {
                            header_state = header_states::h_matching_transfer_encoding;
                        }
                        117 => {
                            header_state = header_states::h_matching_upgrade;
                        }
                        _ => {
                            header_state = header_states::h_general;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            41 => {
                c = tokens[ch as u8 as usize];
                if c != 0 {
                    match header_state as u32 {
                        0 => {}
                        1 => {
                            index = index.wrapping_add(1);
                            index;
                            header_state = (if c as i32 == 'o' as i32 {
                                header_states::h_CO as i32
                            } else {
                                header_states::h_general as i32
                            }) as header_states;
                        }
                        2 => {
                            index = index.wrapping_add(1);
                            index;
                            header_state = (if c as i32 == 'n' as i32 {
                                header_states::h_CON as i32
                            } else {
                                header_states::h_general as i32
                            }) as header_states;
                        }
                        3 => {
                            index = index.wrapping_add(1);
                            index;
                            match c as i32 {
                                110 => {
                                    header_state = header_states::h_matching_connection;
                                }
                                116 => {
                                    header_state = header_states::h_matching_content_length;
                                }
                                _ => {
                                    header_state = header_states::h_general;
                                }
                            }
                        }
                        4 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::core::mem::size_of::<[i8; 11]>() as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                || c as i32
                                    != (*::core::mem::transmute::<
                                        &[u8; 11],
                                        &[i8; 11],
                                    >(b"connection\0"))[index as usize] as i32
                            {
                                header_state = header_states::h_general;
                            } else if index
                                == (::core::mem::size_of::<[i8; 11]>() as u64)
                                    .wrapping_sub(2 as i32 as u64)
                            {
                                header_state = header_states::h_connection;
                            }
                        }
                        5 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::core::mem::size_of::<[i8; 17]>() as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                || c as i32
                                    != (*::core::mem::transmute::<
                                        &[u8; 17],
                                        &[i8; 17],
                                    >(b"proxy-connection\0"))[index as usize] as i32
                            {
                                header_state = header_states::h_general;
                            } else if index
                                == (::core::mem::size_of::<[i8; 17]>() as u64)
                                    .wrapping_sub(2 as i32 as u64)
                            {
                                header_state = header_states::h_connection;
                            }
                        }
                        6 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::core::mem::size_of::<[i8; 15]>() as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                || c as i32
                                    != (*::core::mem::transmute::<
                                        &[u8; 15],
                                        &[i8; 15],
                                    >(b"content-length\0"))[index as usize] as i32
                            {
                                header_state = header_states::h_general;
                            } else if index
                                == (::core::mem::size_of::<[i8; 15]>() as u64)
                                    .wrapping_sub(2 as i32 as u64)
                            {
                                header_state = header_states::h_content_length;
                            }
                        }
                        7 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::core::mem::size_of::<[i8; 18]>() as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                || c as i32
                                    != (*::core::mem::transmute::<
                                        &[u8; 18],
                                        &[i8; 18],
                                    >(b"transfer-encoding\0"))[index as usize] as i32
                            {
                                header_state = header_states::h_general;
                            } else if index
                                == (::core::mem::size_of::<[i8; 18]>() as u64)
                                    .wrapping_sub(2 as i32 as u64)
                            {
                                header_state = header_states::h_transfer_encoding;
                            }
                        }
                        8 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::core::mem::size_of::<[i8; 8]>() as u64)
                                    .wrapping_sub(1 as i32 as u64)
                                || c as i32
                                    != (*::core::mem::transmute::<
                                        &[u8; 8],
                                        &[i8; 8],
                                    >(b"upgrade\0"))[index as usize] as i32
                            {
                                header_state = header_states::h_general;
                            } else if index
                                == (::core::mem::size_of::<[i8; 8]>() as u64)
                                    .wrapping_sub(2 as i32 as u64)
                            {
                                header_state = header_states::h_upgrade;
                            }
                        }
                        9 | 10 | 11 | 12 => {
                            if ch as i32 != ' ' as i32 {
                                header_state = header_states::h_general;
                            }
                        }
                        _ => {
                            if 0 as i32 != 0
                                && !(b"Unknown header_state\0" as *const u8 as *const i8)
                                    .is_null()
                            {} else {
                                __assert_fail(
                                    b"0 && \"Unknown header_state\"\0" as *const u8
                                        as *const i8,
                                    b"src/http-parser/http_parser.c\0" as *const u8
                                        as *const i8,
                                    1153 as i32 as u32,
                                    (*::core::mem::transmute::<
                                        &[u8; 94],
                                        &[i8; 94],
                                    >(
                                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                    ))
                                        .as_ptr(),
                                );
                            };
                        }
                    }
                } else if ch as i32 == ':' as i32 {
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() {
                            if 0 as i32
                                != ((*settings).on_header_field)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as i64 as size_t,
                                )
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                    }
                    header_field_mark = 0 as *const i8;
                    state = state::s_header_value_start;
                } else if ch as i32 == '\r' as i32 {
                    state = state::s_header_almost_done;
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() {
                            if 0 as i32
                                != ((*settings).on_header_field)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as i64 as size_t,
                                )
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                    }
                    header_field_mark = 0 as *const i8;
                } else {
                    if !(ch as i32 == '\n' as i32) {
                        current_block = 5716811153459378242;
                        break;
                    }
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() {
                            if 0 as i32
                                != ((*settings).on_header_field)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as i64 as size_t,
                                )
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                    }
                    header_field_mark = 0 as *const i8;
                    state = state::s_header_field_start;
                }
                current_block = 13472856163611868459;
            }
            42 => {
                if ch as i32 == ' ' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    header_value_mark = p;
                    state = state::s_header_value;
                    index = 0 as i32 as uint64_t;
                    c = (ch as i32 | 0x20 as i32) as u8 as i8;
                    if ch as i32 == '\r' as i32 {
                        if !header_value_mark.is_null() {
                            if ((*settings).on_header_value).is_some() {
                                if 0 as i32
                                    != ((*settings).on_header_value)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as i64 as size_t,
                                    )
                                {
                                    return p.offset_from(data) as i64 as size_t;
                                }
                            }
                        }
                        header_value_mark = 0 as *const i8;
                        header_state = header_states::h_general;
                        state = state::s_header_almost_done;
                    } else if ch as i32 == '\n' as i32 {
                        if !header_value_mark.is_null() {
                            if ((*settings).on_header_value).is_some() {
                                if 0 as i32
                                    != ((*settings).on_header_value)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as i64 as size_t,
                                    )
                                {
                                    return p.offset_from(data) as i64 as size_t;
                                }
                            }
                        }
                        header_value_mark = 0 as *const i8;
                        state = state::s_header_field_start;
                    } else {
                        match header_state as u32 {
                            12 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | flags::F_UPGRADE as i32 as u8,
                                    );
                                header_state = header_states::h_general;
                            }
                            11 => {
                                if 'c' as i32 == c as i32 {
                                    header_state = header_states::h_matching_transfer_encoding_chunked;
                                } else {
                                    header_state = header_states::h_general;
                                }
                            }
                            10 => {
                                if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                    current_block = 5716811153459378242;
                                    break;
                                }
                                (*parser).content_length = (ch as i32 - '0' as i32)
                                    as int64_t;
                            }
                            9 => {
                                if c as i32 == 'k' as i32 {
                                    header_state = header_states::h_matching_connection_keep_alive;
                                } else if c as i32 == 'c' as i32 {
                                    header_state = header_states::h_matching_connection_close;
                                } else {
                                    header_state = header_states::h_general;
                                }
                            }
                            _ => {
                                header_state = header_states::h_general;
                            }
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            43 => {
                c = (ch as i32 | 0x20 as i32) as u8 as i8;
                if ch as i32 == '\r' as i32 {
                    if !header_value_mark.is_null() {
                        if ((*settings).on_header_value).is_some() {
                            if 0 as i32
                                != ((*settings).on_header_value)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    parser,
                                    header_value_mark,
                                    p.offset_from(header_value_mark) as i64 as size_t,
                                )
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                    }
                    header_value_mark = 0 as *const i8;
                    state = state::s_header_almost_done;
                    current_block = 13472856163611868459;
                } else if ch as i32 == '\n' as i32 {
                    if !header_value_mark.is_null() {
                        if ((*settings).on_header_value).is_some() {
                            if 0 as i32
                                != ((*settings).on_header_value)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    parser,
                                    header_value_mark,
                                    p.offset_from(header_value_mark) as i64 as size_t,
                                )
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                    }
                    header_value_mark = 0 as *const i8;
                    current_block = 11020582689779776601;
                } else {
                    match header_state as u32 {
                        0 => {
                            current_block = 13472856163611868459;
                        }
                        9 | 11 => {
                            current_block = 7179728134253626143;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        10 => {
                            current_block = 6777586944787613766;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        13 => {
                            current_block = 1121609260800691507;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        14 => {
                            current_block = 4254522497841701048;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        15 => {
                            current_block = 3176680614765783213;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        16 | 17 | 18 => {
                            current_block = 4589371822259783267;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        _ => {
                            current_block = 11122109046452143732;
                            match current_block {
                                4589371822259783267 => {
                                    if ch as i32 != ' ' as i32 {
                                        header_state = header_states::h_general;
                                    }
                                }
                                4254522497841701048 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 11],
                                                &[i8; 11],
                                            >(b"keep-alive\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 11]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_keep_alive;
                                    }
                                }
                                1121609260800691507 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 8],
                                                &[i8; 8],
                                            >(b"chunked\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_transfer_encoding_chunked;
                                    }
                                }
                                6777586944787613766 => {
                                    if !(ch as i32 == ' ' as i32) {
                                        if (ch as i32) < '0' as i32 || ch as i32 > '9' as i32 {
                                            current_block = 5716811153459378242;
                                            break;
                                        }
                                        (*parser).content_length *= 10 as i32 as i64;
                                        (*parser).content_length += (ch as i32 - '0' as i32) as i64;
                                    }
                                }
                                11122109046452143732 => {
                                    state = state::s_header_value;
                                    header_state = header_states::h_general;
                                }
                                7179728134253626143 => {
                                    if 0 as i32 != 0
                                        && !(b"Shouldn't get here.\0" as *const u8 as *const i8)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8 as *const i8,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const i8,
                                            1264 as i32 as u32,
                                            (*::core::mem::transmute::<
                                                &[u8; 94],
                                                &[i8; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                        || c as i32
                                            != (*::core::mem::transmute::<
                                                &[u8; 6],
                                                &[i8; 6],
                                            >(b"close\0"))[index as usize] as i32
                                    {
                                        header_state = header_states::h_general;
                                    } else if index
                                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(2 as i32 as u64)
                                    {
                                        header_state = header_states::h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                    }
                }
            }
            44 => {
                current_block = 11020582689779776601;
            }
            45 => {
                current_block = 6436225509474489267;
            }
            53 => {
                to_read = if (pe.offset_from(p) as i64) < (*parser).content_length {
                    pe.offset_from(p) as i64
                } else {
                    (*parser).content_length
                };
                if to_read > 0 as i32 as i64 {
                    if ((*settings).on_body).is_some() {
                        ((*settings).on_body)
                            .expect(
                                "non-null function pointer",
                            )(parser, p, to_read as size_t);
                    }
                    p = p.offset((to_read - 1 as i32 as i64) as isize);
                    (*parser).content_length -= to_read;
                    if (*parser).content_length == 0 as i32 as i64 {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as i32
                                != ((*settings).on_message_complete)
                                    .expect("non-null function pointer")(parser)
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
                            if (*parser).type_0() as i32
                                == http_parser_type::HTTP_REQUEST as i32
                            {
                                state::s_start_req as i32
                            } else {
                                state::s_start_res as i32
                            }
                        } else {
                            state::s_dead as i32
                        }) as state;
                    }
                }
                current_block = 13472856163611868459;
            }
            54 => {
                to_read = pe.offset_from(p) as i64;
                if to_read > 0 as i32 as i64 {
                    if ((*settings).on_body).is_some() {
                        ((*settings).on_body)
                            .expect(
                                "non-null function pointer",
                            )(parser, p, to_read as size_t);
                    }
                    p = p.offset((to_read - 1 as i32 as i64) as isize);
                }
                current_block = 13472856163611868459;
            }
            46 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1440 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                c = unhex[ch as u8 as usize] as i8;
                if c as i32 == -(1 as i32) as i8 as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                (*parser).content_length = c as int64_t;
                state = state::s_chunk_size;
                current_block = 13472856163611868459;
            }
            47 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1451 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ch as i32 == '\r' as i32 {
                    state = state::s_chunk_size_almost_done;
                } else {
                    c = unhex[ch as u8 as usize] as i8;
                    if c as i32 == -(1 as i32) as i8 as i32 {
                        if !(ch as i32 == ';' as i32 || ch as i32 == ' ' as i32) {
                            current_block = 5716811153459378242;
                            break;
                        }
                        state = state::s_chunk_parameters;
                    } else {
                        (*parser).content_length *= 16 as i32 as i64;
                        (*parser).content_length += c as i64;
                    }
                }
                current_block = 13472856163611868459;
            }
            49 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1475 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ch as i32 == '\r' as i32 {
                    state = state::s_chunk_size_almost_done;
                    current_block = 13472856163611868459;
                } else {
                    current_block = 13472856163611868459;
                }
            }
            48 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1486 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ch as i32 != '\n' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                if (*parser).content_length == 0 as i32 as i64 {
                    (*parser)
                        .set_flags((*parser).flags() | flags::F_TRAILING as i32 as u8);
                    state = state::s_header_field_start;
                } else {
                    state = state::s_chunk_data;
                }
                current_block = 13472856163611868459;
            }
            50 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1500 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                to_read = if (pe.offset_from(p) as i64) < (*parser).content_length {
                    pe.offset_from(p) as i64
                } else {
                    (*parser).content_length
                };
                if to_read > 0 as i32 as i64 {
                    if ((*settings).on_body).is_some() {
                        ((*settings).on_body)
                            .expect(
                                "non-null function pointer",
                            )(parser, p, to_read as size_t);
                    }
                    p = p.offset((to_read - 1 as i32 as i64) as isize);
                }
                if to_read == (*parser).content_length {
                    state = state::s_chunk_data_almost_done;
                }
                (*parser).content_length -= to_read;
                current_block = 13472856163611868459;
            }
            51 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1518 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ch as i32 != '\r' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_chunk_data_done;
                current_block = 13472856163611868459;
            }
            52 => {
                if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {} else {
                    __assert_fail(
                        b"parser->flags & flags::F_CHUNKED\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1524 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ch as i32 != '\n' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_chunk_size_start;
                current_block = 13472856163611868459;
            }
            _ => {
                if 0 as i32 != 0
                    && !(b"unhandled state\0" as *const u8 as *const i8).is_null()
                {} else {
                    __assert_fail(
                        b"0 && \"unhandled state\"\0" as *const u8 as *const i8,
                        b"src/http-parser/http_parser.c\0" as *const u8 as *const i8,
                        1530 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 94],
                            &[i8; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                current_block = 5716811153459378242;
                break;
            }
        }
        match current_block {
            6436225509474489267 => {
                if ch as i32 != '\n' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                if (*parser).flags() as i32 & flags::F_TRAILING as i32 != 0 {
                    if ((*settings).on_message_complete).is_some() {
                        if 0 as i32
                            != ((*settings).on_message_complete)
                                .expect("non-null function pointer")(parser)
                        {
                            return p.offset_from(data) as i64 as size_t;
                        }
                    }
                    state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as i32
                            == http_parser_type::HTTP_REQUEST as i32
                        {
                            state::s_start_req as i32
                        } else {
                            state::s_start_res as i32
                        }
                    } else {
                        state::s_dead as i32
                    }) as state;
                } else {
                    nread = 0 as i32 as uint64_t;
                    if (*parser).flags() as i32 & flags::F_UPGRADE as i32 != 0
                        || (*parser).method as i32 == http_method::HTTP_CONNECT as i32
                    {
                        (*parser).upgrade = 1 as i32 as i8;
                    }
                    if ((*settings).on_headers_complete).is_some() {
                        match ((*settings).on_headers_complete)
                            .expect("non-null function pointer")(parser)
                        {
                            0 => {}
                            1 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | flags::F_SKIPBODY as i32 as u8,
                                    );
                            }
                            _ => return p.offset_from(data) as i64 as size_t,
                        }
                    }
                    if (*parser).upgrade != 0 {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as i32
                                != ((*settings).on_message_complete)
                                    .expect("non-null function pointer")(parser)
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                        return p.offset_from(data) as i64 as size_t;
                    }
                    if (*parser).flags() as i32 & flags::F_SKIPBODY as i32 != 0 {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as i32
                                != ((*settings).on_message_complete)
                                    .expect("non-null function pointer")(parser)
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
                            if (*parser).type_0() as i32
                                == http_parser_type::HTTP_REQUEST as i32
                            {
                                state::s_start_req as i32
                            } else {
                                state::s_start_res as i32
                            }
                        } else {
                            state::s_dead as i32
                        }) as state;
                    } else if (*parser).flags() as i32 & flags::F_CHUNKED as i32 != 0 {
                        state = state::s_chunk_size_start;
                    } else if (*parser).content_length == 0 as i32 as i64 {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as i32
                                != ((*settings).on_message_complete)
                                    .expect("non-null function pointer")(parser)
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
                            if (*parser).type_0() as i32
                                == http_parser_type::HTTP_REQUEST as i32
                            {
                                state::s_start_req as i32
                            } else {
                                state::s_start_res as i32
                            }
                        } else {
                            state::s_dead as i32
                        }) as state;
                    } else if (*parser).content_length > 0 as i32 as i64 {
                        state = state::s_body_identity;
                    } else if (*parser).type_0() as i32
                        == http_parser_type::HTTP_REQUEST as i32
                        || http_should_keep_alive(parser) != 0
                    {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as i32
                                != ((*settings).on_message_complete)
                                    .expect("non-null function pointer")(parser)
                            {
                                return p.offset_from(data) as i64 as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
                            if (*parser).type_0() as i32
                                == http_parser_type::HTTP_REQUEST as i32
                            {
                                state::s_start_req as i32
                            } else {
                                state::s_start_res as i32
                            }
                        } else {
                            state::s_dead as i32
                        }) as state;
                    } else {
                        state = state::s_body_identity_eof;
                    }
                }
            }
            11020582689779776601 => {
                if ch as i32 != '\n' as i32 {
                    current_block = 5716811153459378242;
                    break;
                }
                state = state::s_header_field_start;
                match header_state as u32 {
                    17 => {
                        (*parser)
                            .set_flags(
                                (*parser).flags()
                                    | flags::F_CONNECTION_KEEP_ALIVE as i32 as u8,
                            );
                    }
                    18 => {
                        (*parser)
                            .set_flags(
                                (*parser).flags() | flags::F_CONNECTION_CLOSE as i32 as u8,
                            );
                    }
                    16 => {
                        (*parser)
                            .set_flags(
                                (*parser).flags() | flags::F_CHUNKED as i32 as u8,
                            );
                    }
                    _ => {}
                }
            }
            3415824759247930212 => {
                (*parser).method = http_method::HTTP_DELETE as u8;
                index = 1 as i32 as uint64_t;
                match ch as i32 {
                    67 => {
                        (*parser).method = http_method::HTTP_CONNECT as i32 as u8;
                    }
                    68 => {
                        (*parser).method = http_method::HTTP_DELETE as i32 as u8;
                    }
                    71 => {
                        (*parser).method = http_method::HTTP_GET as i32 as u8;
                    }
                    72 => {
                        (*parser).method = http_method::HTTP_HEAD as i32 as u8;
                    }
                    76 => {
                        (*parser).method = http_method::HTTP_LOCK as i32 as u8;
                    }
                    77 => {
                        (*parser).method = http_method::HTTP_MKCOL as i32 as u8;
                    }
                    78 => {
                        (*parser).method = http_method::HTTP_NOTIFY as i32 as u8;
                    }
                    79 => {
                        (*parser).method = http_method::HTTP_OPTIONS as i32 as u8;
                    }
                    80 => {
                        (*parser).method = http_method::HTTP_POST as i32 as u8;
                    }
                    82 => {
                        (*parser).method = http_method::HTTP_REPORT as i32 as u8;
                    }
                    83 => {
                        (*parser).method = http_method::HTTP_SUBSCRIBE as i32 as u8;
                    }
                    84 => {
                        (*parser).method = http_method::HTTP_TRACE as i32 as u8;
                    }
                    85 => {
                        (*parser).method = http_method::HTTP_UNLOCK as i32 as u8;
                    }
                    _ => {
                        current_block = 5716811153459378242;
                        break;
                    }
                }
                state = state::s_req_method;
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        5716811153459378242 => {
            (*parser).state = state::s_dead as i32 as u8;
            return p.offset_from(data) as i64 as size_t;
        }
        _ => {
            if !header_field_mark.is_null() {
                if ((*settings).on_header_field).is_some() {
                    if 0 as i32
                        != ((*settings).on_header_field)
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            header_field_mark,
                            p.offset_from(header_field_mark) as i64 as size_t,
                        )
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
            }
            if !header_value_mark.is_null() {
                if ((*settings).on_header_value).is_some() {
                    if 0 as i32
                        != ((*settings).on_header_value)
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            header_value_mark,
                            p.offset_from(header_value_mark) as i64 as size_t,
                        )
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
            }
            if !fragment_mark.is_null() {
                if ((*settings).on_fragment).is_some() {
                    if 0 as i32
                        != ((*settings).on_fragment)
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            fragment_mark,
                            p.offset_from(fragment_mark) as i64 as size_t,
                        )
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
            }
            if !query_string_mark.is_null() {
                if ((*settings).on_query_string).is_some() {
                    if 0 as i32
                        != ((*settings).on_query_string)
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            query_string_mark,
                            p.offset_from(query_string_mark) as i64 as size_t,
                        )
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
            }
            if !path_mark.is_null() {
                if ((*settings).on_path).is_some() {
                    if 0 as i32
                        != ((*settings).on_path)
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            path_mark,
                            p.offset_from(path_mark) as i64 as size_t,
                        )
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
            }
            if !url_mark.is_null() {
                if ((*settings).on_url).is_some() {
                    if 0 as i32
                        != ((*settings).on_url)
                            .expect(
                                "non-null function pointer",
                            )(parser, url_mark, p.offset_from(url_mark) as i64 as size_t)
                    {
                        return p.offset_from(data) as i64 as size_t;
                    }
                }
            }
            (*parser).state = state as u8;
            (*parser).header_state = header_state as u8;
            (*parser).index = index as u8;
            (*parser).nread = nread as uint32_t;
            return len;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_should_keep_alive(mut parser: *mut http_parser) -> i32 {
    if (*parser).http_major as i32 > 0 as i32 && (*parser).http_minor as i32 > 0 as i32 {
        if (*parser).flags() as i32 & flags::F_CONNECTION_CLOSE as i32 != 0 {
            return 0 as i32
        } else {
            return 1 as i32
        }
    } else if (*parser).flags() as i32 & flags::F_CONNECTION_KEEP_ALIVE as i32 != 0 {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_method_str(mut m: http_method) -> *const i8 {
    return method_strings[m as usize];
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_init(
    mut parser: *mut http_parser,
    mut t: http_parser_type,
) {
    (*parser).set_type_0(t as u8);
    (*parser).state = (if t as u32 == http_parser_type::HTTP_REQUEST as i32 as u32 {
        state::s_start_req as i32
    } else if t as u32 == http_parser_type::HTTP_RESPONSE as i32 as u32 {
        state::s_start_res as i32
    } else {
        state::s_start_req_or_res as i32
    }) as u8;
    (*parser).nread = 0 as i32 as uint32_t;
    (*parser).upgrade = 0 as i32 as i8;
    (*parser).set_flags(0 as i32 as u8);
    (*parser).method = 0 as i32 as u8;
}