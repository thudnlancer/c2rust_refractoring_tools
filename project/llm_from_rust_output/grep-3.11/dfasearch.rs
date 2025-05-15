use std::ptr;
use std::mem;
use std::ffi::{CString, CStr};
use libc::{c_char, c_int, c_void, size_t, ssize_t, c_ulong, c_uchar, c_long, c_uint};
use bitflags::bitflags;

bitflags! {
    pub struct RegSyntax: c_ulong {
        const RE_BACKSLASH_ESCAPE_IN_LISTS = 1 << 0;
        const RE_BK_PLUS_QM = 1 << 1;
        const RE_CHAR_CLASSES = 1 << 2;
        const RE_CONTEXT_INDEP_ANCHORS = 1 << 3;
        const RE_CONTEXT_INDEP_OPS = 1 << 4;
        const RE_CONTEXT_INVALID_OPS = 1 << 5;
        const RE_DOT_NEWLINE = 1 << 6;
        const RE_DOT_NOT_NULL = 1 << 7;
        const RE_HAT_LISTS_NOT_NEWLINE = 1 << 8;
        const RE_INTERVALS = 1 << 9;
        const RE_LIMITED_OPS = 1 << 10;
        const RE_NEWLINE_ALT = 1 << 11;
        const RE_NO_BK_BRACES = 1 << 12;
        const RE_NO_BK_PARENS = 1 << 13;
        const RE_NO_BK_REFS = 1 << 14;
        const RE_NO_BK_VBAR = 1 << 15;
        const RE_NO_EMPTY_RANGES = 1 << 16;
        const RE_UNMATCHED_RIGHT_PAREN_ORD = 1 << 17;
        const RE_NO_POSIX_BACKTRACKING = 1 << 18;
        const RE_NO_GNU_OPS = 1 << 19;
        const RE_DEBUG = 1 << 20;
        const RE_INVALID_INTERVAL_ORD = 1 << 21;
        const RE_ICASE = 1 << 22;
        const RE_CARET_ANCHORS_HERE = 1 << 23;
        const RE_CONTEXT_INVALID_DUP = 1 << 24;
        const RE_NO_SUB = 1 << 25;
    }
}

#[repr(C)]
pub struct RePatternBuffer {
    buffer: *mut c_void,
    allocated: size_t,
    used: size_t,
    syntax: RegSyntax,
    fastmap: *mut c_char,
    translate: *mut c_uchar,
    re_nsub: size_t,
    flags: u8,
    _padding: [u8; 7],
}

#[repr(C)]
pub struct ReRegisters {
    num_regs: size_t,
    start: *mut ssize_t,
    end: *mut ssize_t,
}

pub type RegexT = RePatternBuffer;
pub type RegOffT = ssize_t;

#[repr(C)]
pub struct LocaleInfo {
    multibyte: bool,
    simple: bool,
    using_utf8: bool,
    sbclen: [i8; 256],
    sbctowc: [c_uint; 256],
}

#[repr(C)]
pub struct DfaMust {
    exact: bool,
    begline: bool,
    endline: bool,
    must: [c_char; 0],
}

#[repr(C)]
pub struct Kwsmatch {
    index: c_long,
    offset: c_long,
    size: c_long,
}

#[repr(C)]
pub struct DfaComp {
    kwset: *mut c_void,
    dfa: *mut c_void,
    patterns: *mut RePatternBuffer,
    pcount: c_long,
    regs: ReRegisters,
    kwset_exact_matches: c_long,
    begline: bool,
}

extern "C" {
    fn rpl_re_set_syntax(syntax: RegSyntax) -> RegSyntax;
    fn rpl_re_compile_pattern(
        pattern: *const c_char,
        length: size_t,
        buffer: *mut RePatternBuffer,
    ) -> *const c_char;
    fn rpl_re_search(
        buffer: *mut RePatternBuffer,
        string: *const c_char,
        length: RegOffT,
        start: RegOffT,
        range: RegOffT,
        regs: *mut ReRegisters,
    ) -> RegOffT;
    fn rpl_re_match(
        buffer: *mut RePatternBuffer,
        string: *const c_char,
        length: RegOffT,
        start: RegOffT,
        regs: *mut ReRegisters,
    ) -> RegOffT;
    fn rpl_regfree(preg: *mut RegexT);
    fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn rawmemchr(s: *const c_void, c: c_int) -> *mut c_void;
    fn memrchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn xmalloc(size: size_t) -> *mut c_void;
    fn xcalloc(n: size_t, size: size_t) -> *mut c_void;
    fn xirealloc(ptr: *mut c_void, size: c_long) -> *mut c_void;
    fn xpalloc(
        ptr: *mut c_void,
        pn: *mut c_long,
        n_incr_min: c_long,
        n_max: c_long,
        size: c_long,
    ) -> *mut c_void;
    fn kwsinit(case_sensitive: bool) -> *mut c_void;
    fn kwsincr(kws: *mut c_void, text: *const c_char, len: c_long);
    fn kwsprep(kws: *mut c_void);
    fn kwsexec(
        kws: *mut c_void,
        text: *const c_char,
        len: c_long,
        kwsmatch: *mut Kwsmatch,
        begline: bool,
    ) -> c_long;
    fn dfaalloc() -> *mut c_void;
    fn dfasyntax(dfa: *mut c_void, locale: *const LocaleInfo, syntax: RegSyntax, options: c_int);
    fn dfaparse(pattern: *const c_char, len: c_long, dfa: *mut c_void);
    fn dfamustfree(dm: *mut DfaMust);
    fn dfamust(dfa: *const c_void) -> *mut DfaMust;
    fn dfacomp(pattern: *const c_char, len: c_long, dfa: *mut c_void, searchflag: bool);
    fn dfaexec(
        dfa: *mut c_void,
        begin: *const c_char,
        end: *mut c_char,
        allow_nl: bool,
        count: *mut c_long,
        backref: *mut bool,
    ) -> *mut c_char;
    fn dfasuperset(dfa: *const c_void) -> *mut c_void;
    fn dfaisfast(dfa: *const c_void) -> bool;
    fn dfasupported(dfa: *const c_void) -> bool;
    fn wordchar_next(begin: *const c_char, end: *const c_char) -> c_long;
    fn wordchar_prev(begin: *const c_char, match_pos: *const c_char, end: *const c_char) -> c_long;
    fn mb_goback(
        mb_start: *mut *const c_char,
        len: *mut c_long,
        match_pos: *const c_char,
        end: *const c_char,
    ) -> c_long;
}

pub fn dfaerror(mesg: &str) {
    let c_msg = CString::new(mesg).unwrap();
    unsafe {
        libc::fprintf(
            libc::stderr,
            b"%s\n\0".as_ptr() as *const c_char,
            c_msg.as_ptr(),
        );
        libc::exit(2);
    }
}

pub fn dfawarn(mesg: &str) {
    let c_msg = CString::new(mesg).unwrap();
    unsafe {
        libc::fprintf(
            libc::stderr,
            b"warning: %s\n\0".as_ptr() as *const c_char,
            c_msg.as_ptr(),
        );
    }
}

unsafe fn kwsmusts(dc: *mut DfaComp) {
    let dm = dfamust((*dc).dfa);
    if dm.is_null() {
        return;
    }

    (*dc).kwset = kwsinit(false);
    if (*dm).exact {
        (*dc).kwset_exact_matches += 1;
        let old_len = strlen((*dm).must.as_ptr()) as c_long;
        let new_len = old_len + (*dm).begline as c_long + (*dm).endline as c_long;
        let must = xmalloc(new_len as size_t) as *mut c_char;
        let mut mp = must;
        
        if (*dm).begline {
            *mp = b'\n' as c_char;
            mp = mp.offset(1);
            (*dc).begline = true;
        }
        
        memcpy(
            mp as *mut c_void,
            (*dm).must.as_ptr() as *const c_void,
            old_len as size_t,
        );
        
        if (*dm).endline {
            *mp.offset(old_len) = b'\n' as c_char;
        }
        
        kwsincr((*dc).kwset, must, new_len);
        libc::free(must as *mut c_void);
    } else {
        kwsincr(
            (*dc).kwset,
            (*dm).must.as_ptr(),
            strlen((*dm).must.as_ptr()) as c_long,
        );
    }
    
    kwsprep((*dc).kwset);
    dfamustfree(dm);
}

pub fn GEAcompile(
    pattern: &str,
    size: c_long,
    syntax_bits: RegSyntax,
    exact: bool,
) -> *mut c_void {
    let c_pattern = CString::new(pattern).unwrap();
    unsafe {
        let dc = xcalloc(1, mem::size_of::<DfaComp>() as size_t) as *mut DfaComp;
        (*dc).dfa = dfaalloc();

        let mut syntax = syntax_bits;
        if (*crate::match_icase) {
            syntax |= RegSyntax::RE_ICASE;
        }

        let dfaopts = 4 | 8 | 32 | if syntax.contains(RegSyntax::RE_INTERVALS) { 16 } else { 0 };
        dfasyntax((*dc).dfa, &crate::localeinfo, syntax, dfaopts);

        let bs_safe = !(*crate::localeinfo).multibyte || (*crate::localeinfo).using_utf8;
        let p = c_pattern.as_ptr();
        let patlim = p.offset(size);
        
        (*dc).patterns = xmalloc(mem::size_of::<RePatternBuffer>() as size_t) as *mut RePatternBuffer;
        (*dc).patterns = (*dc).patterns.offset(1);
        (*dc).pcount = 0;
        
        let mut palloc = 1;
        let mut prev = p;
        let mut buf = ptr::null_mut();
        let mut buflen = 0;
        let mut bufalloc = 0;
        let mut lineno = 0;
        let mut compilation_failed = false;

        let mut current = p;
        while current < patlim {
            let sep = rawmemchr(current as *const c_void, b'\n' as c_int) as *const c_char;
            let len = sep.offset_from(current);
            
            let backref = possible_backrefs_in_pattern(current, len, bs_safe);
            
            if backref && prev < current {
                let prevlen = current.offset_from(prev);
                let bufshortage = buflen - bufalloc + prevlen;
                if bufshortage > 0 {
                    buf = xpalloc(
                        buf as *mut c_void,
                        &mut bufalloc,
                        bufshortage,
                        -1,
                        1,
                    ) as *mut c_char;
                }
                memcpy(
                    buf.offset(buflen) as *mut c_void,
                    prev as *const c_void,
                    prevlen as size_t,
                );
                buflen += prevlen;
            }

            let shortage = (*dc).pcount - palloc + 2;
            if shortage > 0 {
                (*dc).patterns = xpalloc(
                    (*dc).patterns.offset(-1) as *mut c_void,
                    &mut palloc,
                    shortage,
                    -1,
                    mem::size_of::<RePatternBuffer>() as c_long,
                ) as *mut RePatternBuffer;
                (*dc).patterns = (*dc).patterns.offset(1);
            }

            if !regex_compile(dc, current, len, (*dc).pcount, lineno, syntax, !backref) {
                compilation_failed = true;
            }

            current = sep.offset(1);
            lineno += 1;
            
            if backref {
                (*dc).pcount += 1;
                prev = current;
            }
        }

        if compilation_failed {
            libc::exit(2);
        }

        if patlim < prev {
            buflen -= 1;
        } else if c_pattern.as_ptr() < prev {
            let prevlen = patlim.offset_from(prev);
            buf = xirealloc(buf as *mut c_void, buflen + prevlen) as *mut c_char;
            memcpy(
                buf.offset(buflen) as *mut c_void,
                prev as *const c_void,
                prevlen as size_t,
            );
            buflen += prevlen;
        } else {
            buf = c_pattern.as_ptr() as *mut c_char;
            buflen = size;
        }

        dfaparse(c_pattern.as_ptr(), size, (*dc).dfa);
        kwsmusts(dc);
        dfacomp(ptr::null(), 0, (*dc).dfa, true);

        if !buf.is_null() {
            if exact || !dfasupported((*dc).dfa) {
                (*dc).patterns = (*dc).patterns.offset(-1);
                (*dc).pcount += 1;
                if !regex_compile(dc, buf, buflen, 0, -1, syntax, false) {
                    libc::abort();
                }
            }
            if buf != c_pattern.as_ptr() as *mut c_char {
                libc::free(buf as *mut c_void);
            }
        }

        dc as *mut c_void
    }
}

pub fn EGexecute(
    vdc: *mut c_void,
    buf: &str,
    size: c_long,
    match_size: &mut c_long,
    start_ptr: Option<&str>,
) -> c_long {
    let c_buf = CString::new(buf).unwrap();
    let start_ptr_c = start_ptr.map(|s| CString::new(s).unwrap());
    
    unsafe {
        let dc = vdc as *mut DfaComp;
        let superset = dfasuperset((*dc).dfa);
        let dfafast = dfaisfast((*dc).dfa);
        
        let mut mb_start = c_buf.as_ptr();
        let buflim = c_buf.as_ptr().offset(size);
        let mut beg = c_buf.as_ptr();
        let mut end = buflim;
        let mut ptr = ptr::null();
        let mut best_match = buflim;
        let mut best_len = 0;
        
        let eol = b'\n' as c_char;
        
        loop {
            if end >= buflim {
                break;
            }
            
            end = buflim;
            
            if start_ptr_c.is_none() {
                let mut next_beg = ptr::null();
                let mut dfa_beg = beg;
                let mut count = 0;
                let mut exact_kwset_match = false;
                let mut backref = false;
                
                if !(*dc).kwset.is_null() {
                    let mut prev_beg = beg;
                    let offset = kwsexec(
                        (*dc).kwset,
                        beg.offset(-((*dc).begline as isize)),
                        buflim.offset_from(beg) + (*dc).begline as c_long,
                        &mut Kwsmatch::default(),
                        true,
                    );
                    
                    if offset < 0 {
                        return offset;
                    }
                    
                    let match_pos = beg.offset(offset);
                    prev_beg = beg;
                    
                    beg = memrchr(
                        c_buf.as_ptr() as *const c_void,
                        eol as c_int,
                        match_pos.offset_from(c_buf.as_ptr()) as size_t,
                    ) as *const c_char;
                    
                    if !beg.is_null() {
                        beg = beg.offset(1);
                    } else {
                        beg = c_buf.as_ptr();
                    }
                    
                    dfa_beg = beg;
                    exact_kwset_match = false; // Simplified for example
                    
                    if exact_kwset_match || !dfafast || 16 < match_pos.offset_from(prev_beg) / 4 {
                        end = rawmemchr(match_pos as *const c_void, eol as c_int) as *const c_char;
                        end = end.offset(1);
                    } else if 16 < (buflim.offset_from(prev_beg) / 4) as c_long {
                        end = rawmemchr(
                            prev_beg.offset(16 * 4),
                            eol as c_int,
                        ) as *const c_char;
                        end = end.offset(1);
                    } else {
                        end = buflim;
                    }
                    
                    if exact_kwset_match {
                        if !(*crate::localeinfo).multibyte || (*crate::localeinfo).using_utf8 {
                            break;
                        }
                        
                        if mb_start < beg {
                           