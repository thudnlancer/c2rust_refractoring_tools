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
extern "C" {
    pub type kwset;
    fn rpl_free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: i32) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    static mut match_words: bool;
    static mut match_lines: bool;
    static mut eolbyte: i8;
    fn kwsincr(_: kwset_t, _: *const i8, _: idx_t);
    fn kwswords(_: kwset_t) -> idx_t;
    fn kwsprep(_: kwset_t);
    fn kwsexec(
        _: kwset_t,
        _: *const i8,
        _: idx_t,
        _: *mut kwsmatch,
        _: bool,
    ) -> ptrdiff_t;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn kwsinit(_: bool) -> kwset_t;
    fn wordchars_size(_: *const i8, _: *const i8) -> idx_t;
    fn wordchar_next(_: *const i8, _: *const i8) -> idx_t;
    fn wordchar_prev(_: *const i8, _: *const i8, _: *const i8) -> idx_t;
    fn mb_goback(
        _: *mut *const i8,
        _: *mut idx_t,
        _: *const i8,
        _: *const i8,
    ) -> ptrdiff_t;
    fn GEAcompile(_: *mut i8, _: idx_t, _: reg_syntax_t, _: bool) -> *mut libc::c_void;
    fn EGexecute(
        _: *mut libc::c_void,
        _: *const i8,
        _: idx_t,
        _: *mut idx_t,
        _: *const i8,
    ) -> ptrdiff_t;
    fn fgrep_to_grep_pattern(_: *mut *mut i8, _: *mut idx_t);
    static mut localeinfo: localeinfo;
}
pub type size_t = u64;
pub type wint_t = u32;
pub type reg_syntax_t = u64;
pub type ptrdiff_t = i64;
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
pub struct kwsmatch {
    pub index: idx_t,
    pub offset: idx_t,
    pub size: idx_t,
}
pub type kwset_t = *mut kwset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwsearch {
    pub kwset: kwset_t,
    pub words: idx_t,
    pub pattern: *mut i8,
    pub size: idx_t,
    pub re: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn Fcompile(
    mut pattern: *mut i8,
    mut size: idx_t,
    mut ignored: reg_syntax_t,
    mut exact: bool,
) -> *mut libc::c_void {
    let mut kwset: kwset_t = 0 as *mut kwset;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut bufalloc: idx_t = 0 as i32 as idx_t;
    kwset = kwsinit(1 as i32 != 0);
    let mut p: *const i8 = pattern;
    loop {
        let mut sep: *const i8 = rawmemchr(p as *const libc::c_void, '\n' as i32)
            as *const i8;
        let mut len: idx_t = sep.offset_from(p) as i64;
        if match_lines {
            if eolbyte as i32 == '\n' as i32 && pattern < p as *mut i8 {
                p = p.offset(-1);
                p;
            } else {
                if bufalloc < len + 2 as i32 as i64 {
                    rpl_free(buf as *mut libc::c_void);
                    bufalloc = len;
                    buf = xpalloc(
                        0 as *mut libc::c_void,
                        &mut bufalloc,
                        2 as i32 as idx_t,
                        -(1 as i32) as ptrdiff_t,
                        1 as i32 as idx_t,
                    ) as *mut i8;
                    *buf.offset(0 as i32 as isize) = eolbyte;
                }
                memcpy(
                    buf.offset(1 as i32 as isize) as *mut libc::c_void,
                    p as *const libc::c_void,
                    len as u64,
                );
                *buf.offset((len + 1 as i32 as i64) as isize) = eolbyte;
                p = buf;
            }
            len += 2 as i32 as i64;
        }
        kwsincr(kwset, p, len);
        p = sep.offset(1 as i32 as isize);
        if !(p <= pattern.offset(size as isize)) {
            break;
        }
    }
    rpl_free(buf as *mut libc::c_void);
    let mut words: idx_t = kwswords(kwset);
    kwsprep(kwset);
    let mut kwsearch: *mut kwsearch = xmalloc(::core::mem::size_of::<kwsearch>() as u64)
        as *mut kwsearch;
    (*kwsearch).kwset = kwset;
    (*kwsearch).words = words;
    (*kwsearch).pattern = pattern;
    (*kwsearch).size = size;
    (*kwsearch).re = 0 as *mut libc::c_void;
    return kwsearch as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Fexecute(
    mut vcp: *mut libc::c_void,
    mut buf: *const i8,
    mut size: idx_t,
    mut match_size: *mut idx_t,
    mut start_ptr: *const i8,
) -> ptrdiff_t {
    let mut current_block: u64;
    let mut beg: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut mb_start: *const i8 = 0 as *const i8;
    let mut len: idx_t = 0;
    let mut eol: i8 = eolbyte;
    let mut kwsearch: *mut kwsearch = vcp as *mut kwsearch;
    let mut kwset: kwset_t = (*kwsearch).kwset;
    let mut mb_check: bool = localeinfo.multibyte as i32 & !localeinfo.using_utf8 as i32
        & !match_lines as i32 != 0;
    let mut longest: bool = (mb_check as i32 | !start_ptr.is_null() as i32
        | match_words as i32) & !match_lines as i32 != 0;
    beg = if !start_ptr.is_null() { start_ptr } else { buf };
    mb_start = beg;
    's_16: loop {
        if !(beg <= buf.offset(size as isize)) {
            current_block = 4090602189656566074;
            break;
        }
        let mut kwsmatch: kwsmatch = kwsmatch {
            index: 0,
            offset: 0,
            size: 0,
        };
        let mut offset: ptrdiff_t = kwsexec(
            kwset,
            beg.offset(-(match_lines as i32 as isize)),
            buf.offset(size as isize).offset_from(beg) as i64 + match_lines as i64,
            &mut kwsmatch,
            longest,
        );
        if offset < 0 as i32 as i64 {
            current_block = 4090602189656566074;
            break;
        }
        len = kwsmatch.size - (2 as i32 * match_lines as i32) as i64;
        let mut mbclen: idx_t = 0 as i32 as idx_t;
        if mb_check as i32 != 0
            && mb_goback(
                &mut mb_start,
                &mut mbclen,
                beg.offset(offset as isize),
                buf.offset(size as isize),
            ) != 0 as i32 as i64
        {
            beg = mb_start.offset(-(1 as i32 as isize));
        } else {
            beg = beg.offset(offset as isize);
            if !start_ptr.is_null() as i32 & !match_words as i32 != 0 {
                current_block = 12381812505308290051;
                break;
            }
            if match_lines {
                len += (start_ptr == 0 as *mut libc::c_void as *const i8) as i32 as i64;
                current_block = 12381812505308290051;
                break;
            } else {
                if !match_words {
                    current_block = 2957545398143155145;
                    break;
                }
                if mbclen == 0 as i32 as i64 {
                    let mut nl: *const i8 = memrchr(
                        mb_start as *const libc::c_void,
                        eol as i32,
                        beg.offset_from(mb_start) as i64 as size_t,
                    ) as *const i8;
                    if !nl.is_null() {
                        mb_start = nl.offset(1 as i32 as isize);
                    }
                }
                if if mbclen > 0 as i32 as i64 {
                    (wordchar_next(
                        beg.offset(-(mbclen as isize)),
                        buf.offset(size as isize),
                    ) == 0) as i32
                } else {
                    (wordchar_prev(mb_start, beg, buf.offset(size as isize)) == 0) as i32
                } != 0
                {
                    loop {
                        if wordchar_next(
                            beg.offset(len as isize),
                            buf.offset(size as isize),
                        ) == 0
                        {
                            if !start_ptr.is_null() {
                                current_block = 12381812505308290051;
                                break 's_16;
                            } else {
                                current_block = 2957545398143155145;
                                break 's_16;
                            }
                        } else if start_ptr.is_null() && !localeinfo.multibyte {
                            if ((*kwsearch).re).is_null() {
                                fgrep_to_grep_pattern(
                                    &mut (*kwsearch).pattern,
                                    &mut (*kwsearch).size,
                                );
                                (*kwsearch).re = GEAcompile(
                                    (*kwsearch).pattern,
                                    (*kwsearch).size,
                                    (((1 as i32 as u64) << 1 as i32) << 1 as i32
                                        | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                                        | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32
                                        | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32
                                        | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32 | (1 as i32 as u64) << 1 as i32
                                        | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32
                                        | (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32)
                                        & !(((((((((((((((((((((((((1 as i32 as u64) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                            << 1 as i32) << 1 as i32) << 1 as i32
                                            | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                                                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                                                << 1 as i32),
                                    !start_ptr.is_null(),
                                );
                            }
                            if beg.offset(len as isize) < buf.offset(size as isize) {
                                end = rawmemchr(
                                    beg.offset(len as isize) as *const libc::c_void,
                                    eol as i32,
                                ) as *const i8;
                                end = end.offset(1);
                                end;
                            } else {
                                end = buf.offset(size as isize);
                            }
                            if 0 as i32 as i64
                                <= EGexecute(
                                    (*kwsearch).re,
                                    beg,
                                    end.offset_from(beg) as i64,
                                    match_size,
                                    0 as *const i8,
                                )
                            {
                                current_block = 8444886305398829067;
                                break 's_16;
                            }
                            beg = end.offset(-(1 as i32 as isize));
                            break;
                        } else {
                            if len == 0 {
                                break;
                            }
                            let mut shorter_match: kwsmatch = kwsmatch {
                                index: 0,
                                offset: 0,
                                size: 0,
                            };
                            len -= 1;
                            if kwsexec(
                                kwset,
                                beg,
                                len,
                                &mut shorter_match,
                                1 as i32 != 0,
                            ) != 0 as i32 as i64
                            {
                                break;
                            }
                            len = shorter_match.size;
                        }
                    }
                }
                beg = beg
                    .offset(wordchars_size(beg, buf.offset(size as isize)) as isize);
                mb_start = beg;
            }
        }
        beg = beg.offset(1);
        beg;
    }
    match current_block {
        4090602189656566074 => return -(1 as i32) as ptrdiff_t,
        2957545398143155145 => {
            if beg.offset(len as isize) < buf.offset(size as isize) {
                end = rawmemchr(
                    beg.offset(len as isize) as *const libc::c_void,
                    eol as i32,
                ) as *const i8;
                end = end.offset(1);
                end;
            } else {
                end = buf.offset(size as isize);
            }
            current_block = 8444886305398829067;
        }
        _ => {}
    }
    match current_block {
        8444886305398829067 => {
            beg = memrchr(
                buf as *const libc::c_void,
                eol as i32,
                beg.offset_from(buf) as i64 as size_t,
            ) as *const i8;
            beg = if !beg.is_null() { beg.offset(1 as i32 as isize) } else { buf };
            len = end.offset_from(beg) as i64;
        }
        _ => {}
    }
    *match_size = len;
    return beg.offset_from(buf) as i64;
}