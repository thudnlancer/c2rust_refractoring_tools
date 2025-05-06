#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type kwset;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const i8,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const i8;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const i8,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn rpl_re_match(
        __buffer: *mut re_pattern_buffer,
        __String: *const i8,
        __length: regoff_t,
        __start: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn rpl_regfree(__preg: *mut regex_t);
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: i32) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static mut match_icase: bool;
    static mut match_words: bool;
    static mut match_lines: bool;
    static mut eolbyte: i8;
    fn pattern_file_name(_: idx_t, _: *mut idx_t) -> *const i8;
    fn dfaalloc() -> *mut dfa;
    fn dfasyntax(_: *mut dfa, _: *const localeinfo, _: reg_syntax_t, _: i32);
    fn dfaparse(_: *const i8, _: idx_t, _: *mut dfa);
    fn dfamustfree(_: *mut dfamust);
    fn dfamust(_: *const dfa) -> *mut dfamust;
    fn dfacomp(_: *const i8, _: idx_t, _: *mut dfa, _: bool);
    fn dfaexec(
        d: *mut dfa,
        begin: *const i8,
        end: *mut i8,
        allow_nl: bool,
        count: *mut idx_t,
        backref: *mut bool,
    ) -> *mut i8;
    fn dfasuperset(d: *const dfa) -> *mut dfa;
    fn dfaisfast(_: *const dfa) -> bool;
    fn dfasupported(_: *const dfa) -> bool;
    fn kwsincr(_: kwset_t, _: *const i8, _: idx_t);
    fn kwsprep(_: kwset_t);
    fn kwsexec(
        _: kwset_t,
        _: *const i8,
        _: idx_t,
        _: *mut kwsmatch,
        _: bool,
    ) -> ptrdiff_t;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xirealloc(p: *mut libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn kwsinit(_: bool) -> kwset_t;
    fn wordchar_next(_: *const i8, _: *const i8) -> idx_t;
    fn wordchar_prev(_: *const i8, _: *const i8, _: *const i8) -> idx_t;
    fn mb_goback(
        _: *mut *const i8,
        _: *mut idx_t,
        _: *const i8,
        _: *const i8,
    ) -> ptrdiff_t;
    static mut localeinfo: localeinfo;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
}
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type wint_t = u32;
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut u8,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type ptrdiff_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    EXIT_TROUBLE = 2,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::EXIT_TROUBLE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            2 => C2RustUnnamed::EXIT_TROUBLE,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfamust {
    pub exact: bool,
    pub begline: bool,
    pub endline: bool,
    pub must: [i8; 0],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    DFA_PLUS_WARN = 32,
    DFA_STAR_WARN = 16,
    DFA_STRAY_BACKSLASH_WARN = 8,
    DFA_CONFUSING_BRACKETS_ERROR = 4,
    DFA_EOL_NUL = 2,
    DFA_ANCHOR = 1,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::DFA_PLUS_WARN => 32,
            C2RustUnnamed_0::DFA_STAR_WARN => 16,
            C2RustUnnamed_0::DFA_STRAY_BACKSLASH_WARN => 8,
            C2RustUnnamed_0::DFA_CONFUSING_BRACKETS_ERROR => 4,
            C2RustUnnamed_0::DFA_EOL_NUL => 2,
            C2RustUnnamed_0::DFA_ANCHOR => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            32 => C2RustUnnamed_0::DFA_PLUS_WARN,
            16 => C2RustUnnamed_0::DFA_STAR_WARN,
            8 => C2RustUnnamed_0::DFA_STRAY_BACKSLASH_WARN,
            4 => C2RustUnnamed_0::DFA_CONFUSING_BRACKETS_ERROR,
            2 => C2RustUnnamed_0::DFA_EOL_NUL,
            1 => C2RustUnnamed_0::DFA_ANCHOR,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwsmatch {
    pub index: idx_t,
    pub offset: idx_t,
    pub size: idx_t,
}
pub type kwset_t = *mut kwset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfa_comp {
    pub kwset: kwset_t,
    pub dfa: *mut dfa,
    pub patterns: *mut re_pattern_buffer,
    pub pcount: idx_t,
    pub regs: re_registers,
    pub kwset_exact_matches: idx_t,
    pub begline: bool,
}
#[no_mangle]
pub unsafe extern "C" fn dfaerror(mut mesg: *const i8) {
    if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
        error(
            C2RustUnnamed::EXIT_TROUBLE as i32,
            0 as i32,
            b"%s\0" as *const u8 as *const i8,
            mesg,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            C2RustUnnamed::EXIT_TROUBLE as i32,
            0 as i32,
            b"%s\0" as *const u8 as *const i8,
            mesg,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn dfawarn(mut mesg: *const i8) {
    error(
        0 as i32,
        0 as i32,
        dcgettext(0 as *const i8, b"warning: %s\0" as *const u8 as *const i8, 5 as i32),
        mesg,
    );
}
unsafe extern "C" fn kwsmusts(mut dc: *mut dfa_comp) {
    let mut dm: *mut dfamust = dfamust((*dc).dfa);
    if dm.is_null() {
        return;
    }
    (*dc).kwset = kwsinit(0 as i32 != 0);
    if (*dm).exact {
        (*dc).kwset_exact_matches += 1;
        (*dc).kwset_exact_matches;
        let mut old_len: idx_t = strlen(((*dm).must).as_mut_ptr()) as idx_t;
        let mut new_len: idx_t = old_len + (*dm).begline as i64 + (*dm).endline as i64;
        let mut must: *mut i8 = ximalloc(new_len) as *mut i8;
        let mut mp: *mut i8 = must;
        *mp = eolbyte;
        mp = mp.offset((*dm).begline as i32 as isize);
        (*dc).begline = ((*dc).begline as i32 | (*dm).begline as i32) as bool;
        memcpy(
            mp as *mut libc::c_void,
            ((*dm).must).as_mut_ptr() as *const libc::c_void,
            old_len as u64,
        );
        if (*dm).endline {
            *mp.offset(old_len as isize) = eolbyte;
        }
        kwsincr((*dc).kwset, must, new_len);
        rpl_free(must as *mut libc::c_void);
    } else {
        kwsincr(
            (*dc).kwset,
            ((*dm).must).as_mut_ptr(),
            strlen(((*dm).must).as_mut_ptr()) as idx_t,
        );
    }
    kwsprep((*dc).kwset);
    dfamustfree(dm);
}
unsafe extern "C" fn possible_backrefs_in_pattern(
    mut keys: *const i8,
    mut len: idx_t,
    mut bs_safe: bool,
) -> bool {
    let mut second_backslash: i32 = if bs_safe as i32 != 0 {
        '\\' as i32
    } else {
        127 as i32 + 1 as i32
    };
    len -= 1;
    len;
    if 0 as i32 as i64 <= len {
        let mut lim: *const i8 = keys.offset(len as isize);
        let mut p: *const i8 = keys;
        loop {
            p = memchr(
                p as *const libc::c_void,
                '\\' as i32,
                lim.offset_from(p) as i64 as u64,
            ) as *const i8;
            if p.is_null() {
                break;
            }
            if '1' as i32 <= *p.offset(1 as i32 as isize) as i32
                && *p.offset(1 as i32 as isize) as i32 <= '9' as i32
            {
                return 1 as i32 != 0;
            }
            if *p.offset(1 as i32 as isize) as i32 == second_backslash {
                p = p.offset(1);
                p;
                if p == lim {
                    break;
                }
            }
            p = p.offset(1);
            p;
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn regex_compile(
    mut dc: *mut dfa_comp,
    mut p: *const i8,
    mut len: idx_t,
    mut pcount: idx_t,
    mut lineno: idx_t,
    mut syntax_bits: reg_syntax_t,
    mut syntax_only: bool,
) -> bool {
    let mut pat: re_pattern_buffer = re_pattern_buffer {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut i8,
        translate: 0 as *mut u8,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    pat.buffer = 0 as *mut re_dfa_t;
    pat.allocated = 0 as i32 as __re_long_size_t;
    let mut uchar_max: idx_t = (127 as i32 * 2 as i32 + 1 as i32) as idx_t;
    pat.fastmap = (if syntax_only as i32 | match_icase as i32 != 0 {
        0 as *mut libc::c_void
    } else {
        ximalloc(uchar_max + 1 as i32 as i64)
    }) as *mut i8;
    pat.translate = 0 as *mut u8;
    if syntax_only {
        rpl_re_set_syntax(
            syntax_bits
                | (((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32,
        );
    } else {
        rpl_re_set_syntax(syntax_bits);
    }
    let mut err: *const i8 = rpl_re_compile_pattern(p, len as size_t, &mut pat);
    if err.is_null() {
        if syntax_only {
            rpl_regfree(&mut pat);
        } else {
            *((*dc).patterns).offset(pcount as isize) = pat;
        }
        return 1 as i32 != 0;
    }
    rpl_free(pat.fastmap as *mut libc::c_void);
    let mut pat_lineno: idx_t = 0;
    let mut pat_filename: *const i8 = if lineno < 0 as i32 as i64 {
        b"\0" as *const u8 as *const i8
    } else {
        pattern_file_name(lineno, &mut pat_lineno)
    };
    if *pat_filename as i32 == '\0' as i32 {
        error(0 as i32, 0 as i32, b"%s\0" as *const u8 as *const i8, err);
    } else {
        let mut n: ptrdiff_t = pat_lineno;
        error(
            0 as i32,
            0 as i32,
            b"%s:%td: %s\0" as *const u8 as *const i8,
            pat_filename,
            n,
            err,
        );
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn GEAcompile(
    mut pattern: *mut i8,
    mut size: idx_t,
    mut syntax_bits: reg_syntax_t,
    mut exact: bool,
) -> *mut libc::c_void {
    let mut motif: *mut i8 = 0 as *mut i8;
    let mut dc: *mut dfa_comp = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<dfa_comp>() as u64,
    ) as *mut dfa_comp;
    (*dc).dfa = dfaalloc();
    if match_icase {
        syntax_bits
            |= ((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32;
    }
    let mut dfaopts: i32 = C2RustUnnamed_0::DFA_CONFUSING_BRACKETS_ERROR as i32
        | C2RustUnnamed_0::DFA_STRAY_BACKSLASH_WARN as i32
        | C2RustUnnamed_0::DFA_PLUS_WARN as i32
        | (if syntax_bits
            & ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            != 0
        {
            C2RustUnnamed_0::DFA_STAR_WARN as i32
        } else {
            0 as i32
        })
        | (if eolbyte as i32 != 0 {
            0 as i32
        } else {
            C2RustUnnamed_0::DFA_EOL_NUL as i32
        });
    dfasyntax((*dc).dfa, &mut localeinfo, syntax_bits, dfaopts);
    let mut bs_safe: bool = !localeinfo.multibyte as i32 | localeinfo.using_utf8 as i32
        != 0;
    let mut p: *const i8 = pattern;
    let mut patlim: *const i8 = pattern.offset(size as isize);
    let mut compilation_failed: bool = 0 as i32 != 0;
    (*dc).patterns = xmalloc(::core::mem::size_of::<re_pattern_buffer>() as u64)
        as *mut re_pattern_buffer;
    (*dc).patterns = ((*dc).patterns).offset(1);
    (*dc).patterns;
    (*dc).pcount = 0 as i32 as idx_t;
    let mut palloc: idx_t = 1 as i32 as idx_t;
    let mut prev: *const i8 = pattern;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buflen: idx_t = 0 as i32 as idx_t;
    let mut bufalloc: idx_t = 0 as i32 as idx_t;
    let mut lineno: idx_t = 0 as i32 as idx_t;
    loop {
        let mut sep: *const i8 = rawmemchr(p as *const libc::c_void, '\n' as i32)
            as *const i8;
        let mut len: idx_t = sep.offset_from(p) as i64;
        let mut backref: bool = possible_backrefs_in_pattern(p, len, bs_safe);
        if backref as i32 != 0 && prev < p {
            let mut prevlen: idx_t = p.offset_from(prev) as i64;
            let mut bufshortage: ptrdiff_t = buflen - bufalloc + prevlen;
            if (0 as i32 as i64) < bufshortage {
                buf = xpalloc(
                    buf as *mut libc::c_void,
                    &mut bufalloc,
                    bufshortage,
                    -(1 as i32) as ptrdiff_t,
                    1 as i32 as idx_t,
                ) as *mut i8;
            }
            memcpy(
                buf.offset(buflen as isize) as *mut libc::c_void,
                prev as *const libc::c_void,
                prevlen as u64,
            );
            buflen += prevlen;
        }
        let mut shortage: ptrdiff_t = (*dc).pcount - palloc + 2 as i32 as i64;
        if (0 as i32 as i64) < shortage {
            (*dc).patterns = xpalloc(
                ((*dc).patterns).offset(-(1 as i32 as isize)) as *mut libc::c_void,
                &mut palloc,
                shortage,
                -(1 as i32) as ptrdiff_t,
                ::core::mem::size_of::<re_pattern_buffer>() as u64 as idx_t,
            ) as *mut re_pattern_buffer;
            (*dc).patterns = ((*dc).patterns).offset(1);
            (*dc).patterns;
        }
        if !regex_compile(dc, p, len, (*dc).pcount, lineno, syntax_bits, !backref) {
            compilation_failed = 1 as i32 != 0;
        }
        p = sep.offset(1 as i32 as isize);
        lineno += 1;
        lineno;
        if backref {
            (*dc).pcount += 1;
            (*dc).pcount;
            prev = p;
        }
        if !(p <= patlim) {
            break;
        }
    }
    if compilation_failed {
        exit(C2RustUnnamed::EXIT_TROUBLE as i32);
    }
    if patlim < prev {
        buflen -= 1;
        buflen;
    } else if pattern < prev as *mut i8 {
        let mut prevlen_0: idx_t = patlim.offset_from(prev) as i64;
        buf = xirealloc(buf as *mut libc::c_void, buflen + prevlen_0) as *mut i8;
        memcpy(
            buf.offset(buflen as isize) as *mut libc::c_void,
            prev as *const libc::c_void,
            prevlen_0 as u64,
        );
        buflen += prevlen_0;
    } else {
        buf = pattern;
        buflen = size;
    }
    if match_words as i32 != 0 || match_lines as i32 != 0 {
        static mut line_beg_no_bk: [i8; 3] = unsafe {
            *::core::mem::transmute::<&[u8; 3], &[i8; 3]>(b"^(\0")
        };
        static mut line_end_no_bk: [i8; 3] = unsafe {
            *::core::mem::transmute::<&[u8; 3], &[i8; 3]>(b")$\0")
        };
        static mut word_beg_no_bk: [i8; 19] = unsafe {
            *::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"(^|[^[:alnum:]_])(\0")
        };
        static mut word_end_no_bk: [i8; 19] = unsafe {
            *::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b")([^[:alnum:]_]|$)\0")
        };
        static mut line_beg_bk: [i8; 4] = unsafe {
            *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"^\\(\0")
        };
        static mut line_end_bk: [i8; 4] = unsafe {
            *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"\\)$\0")
        };
        static mut word_beg_bk: [i8; 23] = unsafe {
            *::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"\\(^\\|[^[:alnum:]_]\\)\\(\0")
        };
        static mut word_end_bk: [i8; 23] = unsafe {
            *::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"\\)\\([^[:alnum:]_]\\|$\\)\0")
        };
        let mut bk: i32 = (syntax_bits
            & (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32 == 0)
            as i32;
        let mut bracket_bytes: idx_t = (::core::mem::size_of::<[i8; 23]>() as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_add(::core::mem::size_of::<[i8; 23]>() as u64) as idx_t;
        let mut n: *mut i8 = ximalloc(size + bracket_bytes) as *mut i8;
        strcpy(
            n,
            if match_lines as i32 != 0 {
                if bk != 0 { line_beg_bk.as_ptr() } else { line_beg_no_bk.as_ptr() }
            } else if bk != 0 {
                word_beg_bk.as_ptr()
            } else {
                word_beg_no_bk.as_ptr()
            },
        );
        let mut total: idx_t = strlen(n) as idx_t;
        memcpy(
            n.offset(total as isize) as *mut libc::c_void,
            pattern as *const libc::c_void,
            size as u64,
        );
        total += size;
        strcpy(
            n.offset(total as isize),
            if match_lines as i32 != 0 {
                if bk != 0 { line_end_bk.as_ptr() } else { line_end_no_bk.as_ptr() }
            } else if bk != 0 {
                word_end_bk.as_ptr()
            } else {
                word_end_no_bk.as_ptr()
            },
        );
        total = (total as u64).wrapping_add(strlen(n.offset(total as isize))) as idx_t
            as idx_t;
        motif = n;
        pattern = motif;
        size = total;
    } else {
        motif = 0 as *mut i8;
    }
    dfaparse(pattern, size, (*dc).dfa);
    kwsmusts(dc);
    dfacomp(0 as *const i8, 0 as i32 as idx_t, (*dc).dfa, 1 as i32 != 0);
    if !buf.is_null() {
        if exact as i32 != 0 || !dfasupported((*dc).dfa) {
            (*dc).patterns = ((*dc).patterns).offset(-1);
            (*dc).patterns;
            (*dc).pcount += 1;
            (*dc).pcount;
            if !regex_compile(
                dc,
                buf,
                buflen,
                0 as i32 as idx_t,
                -(1 as i32) as idx_t,
                syntax_bits,
                0 as i32 != 0,
            ) {
                abort();
            }
        }
        if buf != pattern {
            rpl_free(buf as *mut libc::c_void);
        }
    }
    rpl_free(motif as *mut libc::c_void);
    return dc as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn EGexecute(
    mut vdc: *mut libc::c_void,
    mut buf: *const i8,
    mut size: idx_t,
    mut match_size: *mut idx_t,
    mut start_ptr: *const i8,
) -> ptrdiff_t {
    let mut current_block: u64;
    let mut buflim: *const i8 = 0 as *const i8;
    let mut beg: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut ptr: *const i8 = 0 as *const i8;
    let mut match_0: *const i8 = 0 as *const i8;
    let mut best_match: *const i8 = 0 as *const i8;
    let mut mb_start: *const i8 = 0 as *const i8;
    let mut eol: i8 = eolbyte;
    let mut start: regoff_t = 0;
    let mut len: idx_t = 0;
    let mut best_len: idx_t = 0;
    let mut kwsm: kwsmatch = kwsmatch {
        index: 0,
        offset: 0,
        size: 0,
    };
    let mut i: idx_t = 0;
    let mut dc: *mut dfa_comp = vdc as *mut dfa_comp;
    let mut superset: *mut dfa = dfasuperset((*dc).dfa);
    let mut dfafast: bool = dfaisfast((*dc).dfa);
    mb_start = buf;
    buflim = buf.offset(size as isize);
    end = buf;
    beg = end;
    's_26: loop {
        if !(end < buflim) {
            current_block = 12070711452894729854;
            break;
        }
        end = buflim;
        if start_ptr.is_null() {
            let mut next_beg: *const i8 = 0 as *const i8;
            let mut dfa_beg: *const i8 = beg;
            let mut count: idx_t = 0 as i32 as idx_t;
            let mut exact_kwset_match: bool = 0 as i32 != 0;
            let mut backref: bool = 0 as i32 != 0;
            if !((*dc).kwset).is_null() {
                let mut prev_beg: *const i8 = 0 as *const i8;
                let mut offset: ptrdiff_t = kwsexec(
                    (*dc).kwset,
                    beg.offset(-((*dc).begline as i32 as isize)),
                    buflim.offset_from(beg) as i64 + (*dc).begline as i64,
                    &mut kwsm,
                    1 as i32 != 0,
                );
                if offset < 0 as i32 as i64 {
                    return offset;
                }
                match_0 = beg.offset(offset as isize);
                prev_beg = beg;
                beg = memrchr(
                    buf as *const libc::c_void,
                    eol as i32,
                    match_0.offset_from(buf) as i64 as size_t,
                ) as *const i8;
                beg = if !beg.is_null() { beg.offset(1 as i32 as isize) } else { buf };
                dfa_beg = beg;
                exact_kwset_match = kwsm.index < (*dc).kwset_exact_matches;
                if exact_kwset_match as i32 != 0 || !dfafast
                    || (if 16 as i32 as i64 > match_0.offset_from(beg) as i64 {
                        16 as i32 as i64
                    } else {
                        match_0.offset_from(beg) as i64
                    }) < match_0.offset_from(prev_beg) as i64 >> 2 as i32
                {
                    end = rawmemchr(match_0 as *const libc::c_void, eol as i32)
                        as *const i8;
                    end = end.offset(1);
                    end;
                } else if (if 16 as i32 as i64 > match_0.offset_from(beg) as i64 {
                    16 as i32 as i64
                } else {
                    match_0.offset_from(beg) as i64
                }) < buflim.offset_from(prev_beg) as i64 >> 2 as i32
                {
                    end = rawmemchr(
                        prev_beg
                            .offset(
                                (4 as i32 as i64
                                    * (if 16 as i32 as i64 > match_0.offset_from(beg) as i64 {
                                        16 as i32 as i64
                                    } else {
                                        match_0.offset_from(beg) as i64
                                    })) as isize,
                            ) as *const libc::c_void,
                        eol as i32,
                    ) as *const i8;
                    end = end.offset(1);
                    end;
                } else {
                    end = buflim;
                }
                if exact_kwset_match {
                    if !localeinfo.multibyte as i32 | localeinfo.using_utf8 as i32 != 0 {
                        current_block = 11990273555315142001;
                        break;
                    }
                    if mb_start < beg {
                        mb_start = beg;
                    }
                    if mb_goback(&mut mb_start, 0 as *mut idx_t, match_0, buflim)
                        == 0 as i32 as i64
                    {
                        current_block = 11990273555315142001;
                        break;
                    }
                    dfa_beg = mb_start;
                }
            }
            if !superset.is_null() && !exact_kwset_match {
                next_beg = dfaexec(
                    superset,
                    dfa_beg,
                    end as *mut i8,
                    0 as i32 != 0,
                    &mut count,
                    0 as *mut bool,
                );
                if next_beg.is_null() || next_beg == end {
                    current_block = 15240798224410183470;
                } else {
                    if count != 0 as i32 as i64 {
                        beg = memrchr(
                            buf as *const libc::c_void,
                            eol as i32,
                            next_beg.offset_from(buf) as i64 as size_t,
                        ) as *const i8;
                        beg = beg.offset(1);
                        beg;
                        dfa_beg = beg;
                    }
                    end = rawmemchr(next_beg as *const libc::c_void, eol as i32)
                        as *const i8;
                    end = end.offset(1);
                    end;
                    count = 0 as i32 as idx_t;
                    current_block = 15597372965620363352;
                }
            } else {
                current_block = 15597372965620363352;
            }
            match current_block {
                15240798224410183470 => {}
                _ => {
                    next_beg = dfaexec(
                        (*dc).dfa,
                        dfa_beg,
                        end as *mut i8,
                        0 as i32 != 0,
                        &mut count,
                        &mut backref,
                    );
                    if next_beg.is_null() || next_beg == end {
                        current_block = 15240798224410183470;
                    } else {
                        if count != 0 as i32 as i64 {
                            beg = memrchr(
                                buf as *const libc::c_void,
                                eol as i32,
                                next_beg.offset_from(buf) as i64 as size_t,
                            ) as *const i8;
                            beg = beg.offset(1);
                            beg;
                        }
                        end = rawmemchr(next_beg as *const libc::c_void, eol as i32)
                            as *const i8;
                        end = end.offset(1);
                        end;
                        if !backref {
                            current_block = 11990273555315142001;
                            break;
                        }
                        ptr = beg;
                        current_block = 10399321362245223758;
                    }
                }
            }
        } else {
            ptr = start_ptr;
            current_block = 10399321362245223758;
        }
        match current_block {
            10399321362245223758 => {
                if (if (0 as i32 as regoff_t) < -(1 as i32) as regoff_t {
                    -(1 as i32) as regoff_t
                } else {
                    (((1 as i32 as regoff_t)
                        << (::core::mem::size_of::<regoff_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                }) < end.offset_from(beg) as i64 - 1 as i32 as i64
                {
                    xalloc_die();
                }
                best_match = end;
                best_len = 0 as i32 as idx_t;
                i = 0 as i32 as idx_t;
                while i < (*dc).pcount {
                    let ref mut fresh0 = *((*dc).patterns).offset(i as isize);
                    (*fresh0).set_not_eol(0 as i32 as u32);
                    let ref mut fresh1 = *((*dc).patterns).offset(i as isize);
                    (*fresh1)
                        .set_newline_anchor(
                            (eolbyte as i32 == '\n' as i32) as i32 as u32,
                        );
                    start = rpl_re_search(
                        &mut *((*dc).patterns).offset(i as isize),
                        beg,
                        end.offset_from(beg) as i64 - 1 as i32 as i64,
                        ptr.offset_from(beg) as i64,
                        end.offset_from(ptr) as i64 - 1 as i32 as i64,
                        &mut (*dc).regs,
                    );
                    if start < -(1 as i32) as i64 {
                        xalloc_die();
                    } else if 0 as i32 as i64 <= start {
                        len = *((*dc).regs.end).offset(0 as i32 as isize) - start;
                        match_0 = beg.offset(start as isize);
                        if !(match_0 > best_match) {
                            if !start_ptr.is_null() && !match_words {
                                current_block = 13613824696613444011;
                            } else if !match_lines && !match_words
                                || match_lines as i32 != 0
                                    && len == end.offset_from(ptr) as i64 - 1 as i32 as i64
                            {
                                match_0 = ptr;
                                len = end.offset_from(ptr) as i64;
                                current_block = 13613824696613444011;
                            } else if !match_lines && match_words as i32 != 0 {
                                loop {
                                    if !(match_0 <= best_match) {
                                        current_block = 11793792312832361944;
                                        break;
                                    }
                                    let mut shorter_len: regoff_t = 0 as i32 as regoff_t;
                                    if wordchar_next(
                                        match_0.offset(len as isize),
                                        end.offset(-(1 as i32 as isize)),
                                    ) == 0
                                        && wordchar_prev(
                                            beg,
                                            match_0,
                                            end.offset(-(1 as i32 as isize)),
                                        ) == 0
                                    {
                                        current_block = 13613824696613444011;
                                        break;
                                    }
                                    if len > 0 as i32 as i64 {
                                        len -= 1;
                                        len;
                                        let ref mut fresh2 = *((*dc).patterns).offset(i as isize);
                                        (*fresh2).set_not_eol(1 as i32 as u32);
                                        shorter_len = rpl_re_match(
                                            &mut *((*dc).patterns).offset(i as isize),
                                            beg,
                                            match_0.offset(len as isize).offset_from(ptr) as i64,
                                            match_0.offset_from(beg) as i64,
                                            &mut (*dc).regs,
                                        );
                                        if shorter_len < -(1 as i32) as i64 {
                                            xalloc_die();
                                        }
                                    }
                                    if (0 as i32 as i64) < shorter_len {
                                        len = shorter_len;
                                    } else {
                                        if match_0 == end.offset(-(1 as i32 as isize)) {
                                            current_block = 11793792312832361944;
                                            break;
                                        }
                                        match_0 = match_0.offset(1);
                                        match_0;
                                        let ref mut fresh3 = *((*dc).patterns).offset(i as isize);
                                        (*fresh3).set_not_eol(0 as i32 as u32);
                                        start = rpl_re_search(
                                            &mut *((*dc).patterns).offset(i as isize),
                                            beg,
                                            end.offset_from(beg) as i64 - 1 as i32 as i64,
                                            match_0.offset_from(beg) as i64,
                                            end.offset_from(match_0) as i64 - 1 as i32 as i64,
                                            &mut (*dc).regs,
                                        );
                                        if start < 0 as i32 as i64 {
                                            if start < -(1 as i32) as i64 {
                                                xalloc_die();
                                            }
                                            current_block = 11793792312832361944;
                                            break;
                                        } else {
                                            len = *((*dc).regs.end).offset(0 as i32 as isize) - start;
                                            match_0 = beg.offset(start as isize);
                                        }
                                    }
                                }
                            } else {
                                current_block = 11793792312832361944;
                            }
                            match current_block {
                                11793792312832361944 => {}
                                _ => {
                                    if start_ptr.is_null() {
                                        current_block = 11990273555315142001;
                                        break 's_26;
                                    }
                                    if match_0 < best_match
                                        || match_0 == best_match && len > best_len
                                    {
                                        best_match = match_0;
                                        best_len = len;
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                    i;
                }
                if best_match < end {
                    beg = best_match;
                    len = best_len;
                    current_block = 2616667235040759262;
                    break;
                }
            }
            _ => {}
        }
        beg = end;
    }
    match current_block {
        12070711452894729854 => return -(1 as i32) as ptrdiff_t,
        11990273555315142001 => {
            len = end.offset_from(beg) as i64;
        }
        _ => {}
    }
    *match_size = len;
    return beg.offset_from(buf) as i64;
}