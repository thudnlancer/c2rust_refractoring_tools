/*!
Regular expression library definitions and routines.
*/

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{c_int, c_uint, size_t, ssize_t};
use std::os::raw::{c_char, c_void};

/// Define __USE_GNU to declare GNU extensions
pub const __USE_GNU: bool = cfg!(feature = "gnu");

/// Size type for regex operations
#[cfg(feature = "large_offsets")]
pub type __re_size_t = size_t;

/// Long size type for regex operations
#[cfg(feature = "large_offsets")]
pub type __re_long_size_t = size_t;

/// Traditional size type
#[cfg(not(feature = "large_offsets"))]
pub type __re_size_t = c_uint;

/// Traditional long size type
#[cfg(not(feature = "large_offsets"))]
pub type __re_long_size_t = c_uint;

/// Signed register type
pub type s_reg_t = c_int;

/// Active register type
pub type active_reg_t = c_uint;

/// Regex syntax type
pub type reg_syntax_t = c_uint;

/// Regex syntax flags
#[cfg(feature = "gnu")]
pub mod syntax_flags {
    use super::reg_syntax_t;
    
    pub const RE_BACKSLASH_ESCAPE_IN_LISTS: reg_syntax_t = 1 << 0;
    pub const RE_BK_PLUS_QM: reg_syntax_t = 1 << 1;
    pub const RE_CHAR_CLASSES: reg_syntax_t = 1 << 2;
    pub const RE_CONTEXT_INDEP_ANCHORS: reg_syntax_t = 1 << 3;
    pub const RE_CONTEXT_INDEP_OPS: reg_syntax_t = 1 << 4;
    pub const RE_CONTEXT_INVALID_OPS: reg_syntax_t = 1 << 5;
    pub const RE_DOT_NEWLINE: reg_syntax_t = 1 << 6;
    pub const RE_DOT_NOT_NULL: reg_syntax_t = 1 << 7;
    pub const RE_HAT_LISTS_NOT_NEWLINE: reg_syntax_t = 1 << 8;
    pub const RE_INTERVALS: reg_syntax_t = 1 << 9;
    pub const RE_LIMITED_OPS: reg_syntax_t = 1 << 10;
    pub const RE_NEWLINE_ALT: reg_syntax_t = 1 << 11;
    pub const RE_NO_BK_BRACES: reg_syntax_t = 1 << 12;
    pub const RE_NO_BK_PARENS: reg_syntax_t = 1 << 13;
    pub const RE_NO_BK_REFS: reg_syntax_t = 1 << 14;
    pub const RE_NO_BK_VBAR: reg_syntax_t = 1 << 15;
    pub const RE_NO_EMPTY_RANGES: reg_syntax_t = 1 << 16;
    pub const RE_UNMATCHED_RIGHT_PAREN_ORD: reg_syntax_t = 1 << 17;
    pub const RE_NO_POSIX_BACKTRACKING: reg_syntax_t = 1 << 18;
    pub const RE_NO_GNU_OPS: reg_syntax_t = 1 << 19;
    pub const RE_DEBUG: reg_syntax_t = 1 << 20;
    pub const RE_INVALID_INTERVAL_ORD: reg_syntax_t = 1 << 21;
    pub const RE_ICASE: reg_syntax_t = 1 << 22;
    pub const RE_CARET_ANCHORS_HERE: reg_syntax_t = 1 << 23;
    pub const RE_CONTEXT_INVALID_DUP: reg_syntax_t = 1 << 24;
    pub const RE_NO_SUB: reg_syntax_t = 1 << 25;
}

/// Predefined syntax patterns
#[cfg(feature = "gnu")]
pub mod syntax_patterns {
    use super::{reg_syntax_t, syntax_flags as F};
    
    pub const RE_SYNTAX_EMACS: reg_syntax_t = 0;
    
    pub const RE_SYNTAX_AWK: reg_syntax_t = 
        F::RE_BACKSLASH_ESCAPE_IN_LISTS | F::RE_DOT_NOT_NULL |
        F::RE_NO_BK_PARENS | F::RE_NO_BK_REFS |
        F::RE_NO_BK_VBAR | F::RE_NO_EMPTY_RANGES |
        F::RE_DOT_NEWLINE | F::RE_CONTEXT_INDEP_ANCHORS |
        F::RE_CHAR_CLASSES | F::RE_UNMATCHED_RIGHT_PAREN_ORD |
        F::RE_NO_GNU_OPS;
        
    pub const RE_SYNTAX_POSIX_AWK: reg_syntax_t =
        F::RE_SYNTAX_POSIX_EXTENDED | F::RE_BACKSLASH_ESCAPE_IN_LISTS |
        F::RE_INTERVALS | F::RE_NO_GNU_OPS | F::RE_INVALID_INTERVAL_ORD;
        
    pub const RE_SYNTAX_GREP: reg_syntax_t =
        (F::RE_SYNTAX_POSIX_BASIC | F::RE_NEWLINE_ALT) &
        !(F::RE_CONTEXT_INVALID_DUP | F::RE_DOT_NOT_NULL);
        
    pub const RE_SYNTAX_EGREP: reg_syntax_t =
        (F::RE_SYNTAX_POSIX_EXTENDED | F::RE_INVALID_INTERVAL_ORD | F::RE_NEWLINE_ALT) &
        !(F::RE_CONTEXT_INVALID_OPS | F::RE_DOT_NOT_NULL);
        
    pub const RE_SYNTAX_POSIX_EGREP: reg_syntax_t = RE_SYNTAX_EGREP;
    pub const RE_SYNTAX_ED: reg_syntax_t = F::RE_SYNTAX_POSIX_BASIC;
    pub const RE_SYNTAX_SED: reg_syntax_t = F::RE_SYNTAX_POSIX_BASIC;
    
    pub const _RE_SYNTAX_POSIX_COMMON: reg_syntax_t =
        F::RE_CHAR_CLASSES | F::RE_DOT_NEWLINE | F::RE_DOT_NOT_NULL |
        F::RE_INTERVALS | F::RE_NO_EMPTY_RANGES;
        
    pub const RE_SYNTAX_POSIX_BASIC: reg_syntax_t =
        _RE_SYNTAX_POSIX_COMMON | F::RE_BK_PLUS_QM | F::RE_CONTEXT_INVALID_DUP;
        
    pub const RE_SYNTAX_POSIX_MINIMAL_BASIC: reg_syntax_t =
        _RE_SYNTAX_POSIX_COMMON | F::RE_LIMITED_OPS;
        
    pub const RE_SYNTAX_POSIX_EXTENDED: reg_syntax_t =
        _RE_SYNTAX_POSIX_COMMON | F::RE_CONTEXT_INDEP_ANCHORS |
        F::RE_CONTEXT_INDEP_OPS | F::RE_NO_BK_BRACES |
        F::RE_NO_BK_PARENS | F::RE_NO_BK_VBAR |
        F::RE_CONTEXT_INVALID_OPS | F::RE_UNMATCHED_RIGHT_PAREN_ORD;
        
    pub const RE_SYNTAX_POSIX_MINIMAL_EXTENDED: reg_syntax_t =
        _RE_SYNTAX_POSIX_COMMON | F::RE_CONTEXT_INDEP_ANCHORS |
        F::RE_CONTEXT_INVALID_OPS | F::RE_NO_BK_BRACES |
        F::RE_NO_BK_PARENS | F::RE_NO_BK_REFS |
        F::RE_NO_BK_VBAR | F::RE_UNMATCHED_RIGHT_PAREN_ORD;
}

/// Maximum number of duplicates
#[cfg(feature = "gnu")]
pub const RE_DUP_MAX: c_int = 0x7fff;

/// Compilation flags
pub mod cflags {
    pub const REG_EXTENDED: c_int = 1;
    pub const REG_ICASE: c_int = 1 << 1;
    pub const REG_NEWLINE: c_int = 1 << 2;
    pub const REG_NOSUB: c_int = 1 << 3;
}

/// Execution flags
pub mod eflags {
    pub const REG_NOTBOL: c_int = 1;
    pub const REG_NOTEOL: c_int = 1 << 1;
    pub const REG_STARTEND: c_int = 1 << 2;
}

/// Error codes
#[derive(Debug, PartialEq)]
pub enum RegErrcode {
    REG_ENOSYS = -1,
    REG_NOERROR = 0,
    REG_NOMATCH,
    REG_BADPAT,
    REG_ECOLLATE,
    REG_ECTYPE,
    REG_EESCAPE,
    REG_ESUBREG,
    REG_EBRACK,
    REG_EPAREN,
    REG_EBRACE,
    REG_BADBR,
    REG_ERANGE,
    REG_ESPACE,
    REG_BADRPT,
    REG_EEND,
    REG_ESIZE,
    REG_ERPAREN,
}

/// Pattern buffer structure
#[repr(C)]
pub struct re_pattern_buffer {
    buffer: *mut c_void,
    allocated: __re_long_size_t,
    used: __re_long_size_t,
    syntax: reg_syntax_t,
    fastmap: *mut c_char,
    translate: *mut c_char,
    re_nsub: size_t,
    can_be_null: c_uint,
    regs_allocated: c_uint,
    fastmap_accurate: c_uint,
    no_sub: c_uint,
    not_bol: c_uint,
    not_eol: c_uint,
    newline_anchor: c_uint,
}

/// Regex type
pub type regex_t = re_pattern_buffer;

/// Offset type
#[cfg(feature = "large_offsets")]
pub type regoff_t = ssize_t;

/// Traditional offset type
#[cfg(not(feature = "large_offsets"))]
pub type regoff_t = c_int;

/// Register structure
#[cfg(feature = "gnu")]
#[repr(C)]
pub struct re_registers {
    num_regs: __re_size_t,
    start: *mut regoff_t,
    end: *mut regoff_t,
}

/// Number of registers
#[cfg(feature = "gnu")]
pub const RE_NREGS: usize = 30;

/// Match structure
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}

/// Regex functions
#[cfg(feature = "gnu")]
pub mod functions {
    use super::*;
    
    extern "C" {
        pub fn re_set_syntax(syntax: reg_syntax_t) -> reg_syntax_t;
        pub fn re_compile_pattern(
            pattern: *const c_char,
            length: size_t,
            buffer: *mut re_pattern_buffer,
        ) -> *const c_char;
        pub fn re_compile_fastmap(buffer: *mut re_pattern_buffer) -> c_int;
        pub fn re_search(
            buffer: *mut re_pattern_buffer,
            string: *const c_char,
            length: regoff_t,
            start: regoff_t,
            range: regoff_t,
            regs: *mut re_registers,
        ) -> regoff_t;
        pub fn re_search_2(
            buffer: *mut re_pattern_buffer,
            string1: *const c_char,
            length1: regoff_t,
            string2: *const c_char,
            length2: regoff_t,
            start: regoff_t,
            range: regoff_t,
            regs: *mut re_registers,
            stop: regoff_t,
        ) -> regoff_t;
        pub fn re_match(
            buffer: *mut re_pattern_buffer,
            string: *const c_char,
            length: regoff_t,
            start: regoff_t,
            regs: *mut re_registers,
        ) -> regoff_t;
        pub fn re_match_2(
            buffer: *mut re_pattern_buffer,
            string1: *const c_char,
            length1: regoff_t,
            string2: *const c_char,
            length2: regoff_t,
            start: regoff_t,
            regs: *mut re_registers,
            stop: regoff_t,
        ) -> regoff_t;
        pub fn re_set_registers(
            buffer: *mut re_pattern_buffer,
            regs: *mut re_registers,
            num_regs: __re_size_t,
            starts: *mut regoff_t,
            ends: *mut regoff_t,
        );
    }
}

/// POSIX functions
extern "C" {
    pub fn regcomp(
        preg: *mut regex_t,
        pattern: *const c_char,
        cflags: c_int,
    ) -> c_int;
    
    pub fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    
    pub fn regerror(
        errcode: c_int,
        preg: *const regex_t,
        errbuf: *mut c_char,
        errbuf_size: size_t,
    ) -> size_t;
    
    pub fn regfree(preg: *mut regex_t);
}

#[cfg(any(feature = "re_comp", all(feature = "libc", feature = "misc")))]
extern "C" {
    pub fn re_comp(pattern: *const c_char) -> *mut c_char;
    pub fn re_exec(string: *const c_char) -> c_int;
}