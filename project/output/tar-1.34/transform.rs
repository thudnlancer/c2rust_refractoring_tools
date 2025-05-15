use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn rpl_regcomp(__preg: *mut regex_t, __pattern: *const i8, __cflags: i32) -> i32;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const i8,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: i32,
    ) -> i32;
    fn rpl_regerror(
        __errcode: i32,
        __preg: *const regex_t,
        __errbuf: *mut i8,
        __errbuf_size: size_t,
    ) -> size_t;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
    fn assign_string(dest: *mut *mut i8, src: *const i8);
    fn usage(_: i32);
}
pub type __int32_t = i32;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type ptrdiff_t = i64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
}
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
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform {
    pub next: *mut transform,
    pub transform_type: transform_type,
    pub flags: i32,
    pub match_number: u32,
    pub regex: regex_t,
    pub repl_head: *mut replace_segm,
    pub repl_tail: *mut replace_segm,
    pub segm_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replace_segm {
    pub next: *mut replace_segm,
    pub type_0: replace_segm_type,
    pub v: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub literal: C2RustUnnamed_3,
    pub ref_0: size_t,
    pub ctl: case_ctl_type,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum case_ctl_type {
    ctl_stop,
    ctl_upcase_next,
    ctl_locase_next,
    ctl_upcase,
    ctl_locase,
}
impl case_ctl_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            case_ctl_type::ctl_stop => 0,
            case_ctl_type::ctl_upcase_next => 1,
            case_ctl_type::ctl_locase_next => 2,
            case_ctl_type::ctl_upcase => 3,
            case_ctl_type::ctl_locase => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> case_ctl_type {
        match value {
            0 => case_ctl_type::ctl_stop,
            1 => case_ctl_type::ctl_upcase_next,
            2 => case_ctl_type::ctl_locase_next,
            3 => case_ctl_type::ctl_upcase,
            4 => case_ctl_type::ctl_locase,
            _ => panic!("Invalid value for case_ctl_type: {}", value),
        }
    }
}
impl AddAssign<u32> for case_ctl_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for case_ctl_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for case_ctl_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for case_ctl_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for case_ctl_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for case_ctl_type {
    type Output = case_ctl_type;
    fn add(self, rhs: u32) -> case_ctl_type {
        case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for case_ctl_type {
    type Output = case_ctl_type;
    fn sub(self, rhs: u32) -> case_ctl_type {
        case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for case_ctl_type {
    type Output = case_ctl_type;
    fn mul(self, rhs: u32) -> case_ctl_type {
        case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for case_ctl_type {
    type Output = case_ctl_type;
    fn div(self, rhs: u32) -> case_ctl_type {
        case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for case_ctl_type {
    type Output = case_ctl_type;
    fn rem(self, rhs: u32) -> case_ctl_type {
        case_ctl_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ptr: *mut i8,
    pub size: size_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum replace_segm_type {
    segm_case_ctl = 2,
    segm_backref = 1,
    segm_literal = 0,
}
impl replace_segm_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            replace_segm_type::segm_case_ctl => 2,
            replace_segm_type::segm_backref => 1,
            replace_segm_type::segm_literal => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> replace_segm_type {
        match value {
            2 => replace_segm_type::segm_case_ctl,
            1 => replace_segm_type::segm_backref,
            0 => replace_segm_type::segm_literal,
            _ => panic!("Invalid value for replace_segm_type: {}", value),
        }
    }
}
impl AddAssign<u32> for replace_segm_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for replace_segm_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for replace_segm_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for replace_segm_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for replace_segm_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for replace_segm_type {
    type Output = replace_segm_type;
    fn add(self, rhs: u32) -> replace_segm_type {
        replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for replace_segm_type {
    type Output = replace_segm_type;
    fn sub(self, rhs: u32) -> replace_segm_type {
        replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for replace_segm_type {
    type Output = replace_segm_type;
    fn mul(self, rhs: u32) -> replace_segm_type {
        replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for replace_segm_type {
    type Output = replace_segm_type;
    fn div(self, rhs: u32) -> replace_segm_type {
        replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for replace_segm_type {
    type Output = replace_segm_type;
    fn rem(self, rhs: u32) -> replace_segm_type {
        replace_segm_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum transform_type {
    transform_global = 1,
    transform_first = 0,
}
impl transform_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            transform_type::transform_global => 1,
            transform_type::transform_first => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> transform_type {
        match value {
            1 => transform_type::transform_global,
            0 => transform_type::transform_first,
            _ => panic!("Invalid value for transform_type: {}", value),
        }
    }
}
impl AddAssign<u32> for transform_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = transform_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for transform_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = transform_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for transform_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = transform_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for transform_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = transform_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for transform_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = transform_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for transform_type {
    type Output = transform_type;
    fn add(self, rhs: u32) -> transform_type {
        transform_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for transform_type {
    type Output = transform_type;
    fn sub(self, rhs: u32) -> transform_type {
        transform_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for transform_type {
    type Output = transform_type;
    fn mul(self, rhs: u32) -> transform_type {
        transform_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for transform_type {
    type Output = transform_type;
    fn div(self, rhs: u32) -> transform_type {
        transform_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for transform_type {
    type Output = transform_type;
    fn rem(self, rhs: u32) -> transform_type {
        transform_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut transform_flags: i32 = 0x1 as i32 | 0x2 as i32 | 0x4 as i32;
static mut transform_head: *mut transform = 0 as *const transform as *mut transform;
static mut transform_tail: *mut transform = 0 as *const transform as *mut transform;
unsafe extern "C" fn new_transform() -> *mut transform {
    let mut p: *mut transform = xzalloc(::core::mem::size_of::<transform>() as u64)
        as *mut transform;
    if !transform_tail.is_null() {
        (*transform_tail).next = p;
    } else {
        transform_head = p;
    }
    transform_tail = p;
    return p;
}
unsafe extern "C" fn add_segment(mut tf: *mut transform) -> *mut replace_segm {
    let mut segm: *mut replace_segm = xmalloc(
        ::core::mem::size_of::<replace_segm>() as u64,
    ) as *mut replace_segm;
    (*segm).next = 0 as *mut replace_segm;
    if !((*tf).repl_tail).is_null() {
        (*(*tf).repl_tail).next = segm;
    } else {
        (*tf).repl_head = segm;
    }
    (*tf).repl_tail = segm;
    (*tf).segm_count = ((*tf).segm_count).wrapping_add(1);
    (*tf).segm_count;
    return segm;
}
unsafe extern "C" fn add_literal_segment(
    mut tf: *mut transform,
    mut str: *const i8,
    mut end: *const i8,
) {
    let mut len: size_t = end.offset_from(str) as i64 as size_t;
    if len != 0 {
        let mut segm: *mut replace_segm = add_segment(tf);
        (*segm).type_0 = replace_segm_type::segm_literal;
        (*segm).v.literal.ptr = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        memcpy(
            (*segm).v.literal.ptr as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
        *((*segm).v.literal.ptr).offset(len as isize) = 0 as i32 as i8;
        (*segm).v.literal.size = len;
    }
}
unsafe extern "C" fn add_char_segment(mut tf: *mut transform, mut chr: i32) {
    let mut segm: *mut replace_segm = add_segment(tf);
    (*segm).type_0 = replace_segm_type::segm_literal;
    (*segm).v.literal.ptr = xmalloc(2 as i32 as size_t) as *mut i8;
    *((*segm).v.literal.ptr).offset(0 as i32 as isize) = chr as i8;
    *((*segm).v.literal.ptr).offset(1 as i32 as isize) = 0 as i32 as i8;
    (*segm).v.literal.size = 1 as i32 as size_t;
}
unsafe extern "C" fn add_backref_segment(mut tf: *mut transform, mut ref_0: size_t) {
    let mut segm: *mut replace_segm = add_segment(tf);
    (*segm).type_0 = replace_segm_type::segm_backref;
    (*segm).v.ref_0 = ref_0;
}
unsafe extern "C" fn parse_xform_flags(mut pflags: *mut i32, mut c: i32) -> i32 {
    match c {
        114 => {
            *pflags |= 0x1 as i32;
        }
        82 => {
            *pflags &= !(0x1 as i32);
        }
        104 => {
            *pflags |= 0x2 as i32;
        }
        72 => {
            *pflags &= !(0x2 as i32);
        }
        115 => {
            *pflags |= 0x4 as i32;
        }
        83 => {
            *pflags &= !(0x4 as i32);
        }
        _ => return 1 as i32,
    }
    return 0 as i32;
}
unsafe extern "C" fn add_case_ctl_segment(
    mut tf: *mut transform,
    mut ctl: case_ctl_type,
) {
    let mut segm: *mut replace_segm = add_segment(tf);
    (*segm).type_0 = replace_segm_type::segm_case_ctl;
    (*segm).v.ctl = ctl;
}
unsafe extern "C" fn parse_transform_expr(mut expr: *const i8) -> *const i8 {
    let mut delim: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut rc: i32 = 0;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut beg: *mut i8 = 0 as *mut i8;
    let mut cur: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut cflags: i32 = 0 as i32;
    let mut tf: *mut transform = new_transform();
    if *expr.offset(0 as i32 as isize) as i32 != 's' as i32 {
        if strncmp(expr, b"flags=\0" as *const u8 as *const i8, 6 as i32 as u64)
            == 0 as i32
        {
            transform_flags = 0 as i32;
            expr = expr.offset(6 as i32 as isize);
            while *expr != 0 {
                if *expr as i32 == ';' as i32 {
                    expr = expr.offset(1);
                    expr;
                    break;
                } else {
                    if parse_xform_flags(&mut transform_flags, *expr as i32) != 0 {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"Unknown transform flag: %c\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            *expr as i32,
                        );
                        usage(2 as i32);
                    }
                    expr = expr.offset(1);
                    expr;
                }
            }
            return expr;
        }
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Invalid transform expression\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(2 as i32);
    }
    delim = *expr.offset(1 as i32 as isize) as i32;
    if delim == 0 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Invalid transform expression\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(2 as i32);
    }
    i = 2 as i32;
    while *expr.offset(i as isize) as i32 != 0
        && *expr.offset(i as isize) as i32 != delim
    {
        if *expr.offset(i as isize) as i32 == '\\' as i32
            && *expr.offset((i + 1 as i32) as isize) as i32 != 0
        {
            i += 1;
            i;
        }
        i += 1;
        i;
    }
    if *expr.offset(i as isize) as i32 != delim {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Invalid transform expression\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(2 as i32);
    }
    j = i + 1 as i32;
    while *expr.offset(j as isize) as i32 != 0
        && *expr.offset(j as isize) as i32 != delim
    {
        if *expr.offset(j as isize) as i32 == '\\' as i32
            && *expr.offset((j + 1 as i32) as isize) as i32 != 0
        {
            j += 1;
            j;
        }
        j += 1;
        j;
    }
    if *expr.offset(j as isize) as i32 != delim {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Invalid transform expression\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(2 as i32);
    }
    (*tf).transform_type = transform_type::transform_first;
    (*tf).flags = transform_flags;
    p = expr.offset(j as isize).offset(1 as i32 as isize);
    while *p as i32 != 0 && *p as i32 != ';' as i32 {
        match *p as i32 {
            103 => {
                (*tf).transform_type = transform_type::transform_global;
            }
            105 => {
                cflags |= (1 as i32) << 1 as i32;
            }
            120 => {
                cflags |= 1 as i32;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                (*tf).match_number = strtoul(
                    p,
                    &mut p as *mut *const i8 as *mut *mut i8,
                    0 as i32,
                ) as u32;
                p = p.offset(-1);
                p;
            }
            _ => {
                if parse_xform_flags(&mut (*tf).flags, *p as i32) != 0 {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Unknown flag in transform expression: %c\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *p as i32,
                    );
                    usage(2 as i32);
                }
            }
        }
        p = p.offset(1);
        p;
    }
    if *p as i32 == ';' as i32 {
        p = p.offset(1);
        p;
    }
    str = xmalloc((i - 1 as i32) as size_t) as *mut i8;
    memcpy(
        str as *mut libc::c_void,
        expr.offset(2 as i32 as isize) as *const libc::c_void,
        (i - 2 as i32) as u64,
    );
    *str.offset((i - 2 as i32) as isize) = 0 as i32 as i8;
    rc = rpl_regcomp(&mut (*tf).regex, str, cflags);
    if rc != 0 {
        let mut errbuf: [i8; 512] = [0; 512];
        rpl_regerror(
            rc,
            &mut (*tf).regex,
            errbuf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 512]>() as u64,
        );
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Invalid transform expression: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            errbuf.as_mut_ptr(),
        );
        usage(2 as i32);
    }
    if *str.offset(0 as i32 as isize) as i32 == '^' as i32
        || i > 2 as i32 && *str.offset((i - 3 as i32) as isize) as i32 == '$' as i32
    {
        (*tf).transform_type = transform_type::transform_first;
    }
    rpl_free(str as *mut libc::c_void);
    i += 1;
    i;
    str = xmalloc((j - i + 1 as i32) as size_t) as *mut i8;
    memcpy(
        str as *mut libc::c_void,
        expr.offset(i as isize) as *const libc::c_void,
        (j - i) as u64,
    );
    *str.offset((j - i) as isize) = 0 as i32 as i8;
    beg = str;
    cur = beg;
    while *cur != 0 {
        if *cur as i32 == '\\' as i32 {
            let mut n: size_t = 0;
            add_literal_segment(tf, beg, cur);
            cur = cur.offset(1);
            match *cur as i32 {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    n = strtoul(cur, &mut cur, 10 as i32);
                    if n > (*tf).regex.re_nsub {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"Invalid transform replacement: back reference out of range\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        usage(2 as i32);
                    }
                    add_backref_segment(tf, n);
                }
                92 => {
                    add_char_segment(tf, '\\' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                97 => {
                    add_char_segment(tf, '\u{7}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                98 => {
                    add_char_segment(tf, '\u{8}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                102 => {
                    add_char_segment(tf, '\u{c}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                110 => {
                    add_char_segment(tf, '\n' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                114 => {
                    add_char_segment(tf, '\r' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                116 => {
                    add_char_segment(tf, '\t' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                118 => {
                    add_char_segment(tf, '\u{b}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                38 => {
                    add_char_segment(tf, '&' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                76 => {
                    add_case_ctl_segment(tf, case_ctl_type::ctl_locase);
                    cur = cur.offset(1);
                    cur;
                }
                108 => {
                    add_case_ctl_segment(tf, case_ctl_type::ctl_locase_next);
                    cur = cur.offset(1);
                    cur;
                }
                85 => {
                    add_case_ctl_segment(tf, case_ctl_type::ctl_upcase);
                    cur = cur.offset(1);
                    cur;
                }
                117 => {
                    add_case_ctl_segment(tf, case_ctl_type::ctl_upcase_next);
                    cur = cur.offset(1);
                    cur;
                }
                69 => {
                    add_case_ctl_segment(tf, case_ctl_type::ctl_stop);
                    cur = cur.offset(1);
                    cur;
                }
                _ => {
                    if *cur as i32 == delim {
                        add_char_segment(tf, delim);
                    } else {
                        let mut buf: [i8; 2] = [0; 2];
                        buf[0 as i32 as usize] = '\\' as i32 as i8;
                        buf[1 as i32 as usize] = *cur;
                        add_literal_segment(
                            tf,
                            buf.as_mut_ptr(),
                            buf.as_mut_ptr().offset(2 as i32 as isize),
                        );
                    }
                    cur = cur.offset(1);
                    cur;
                }
            }
            beg = cur;
        } else if *cur as i32 == '&' as i32 {
            add_literal_segment(tf, beg, cur);
            add_backref_segment(tf, 0 as i32 as size_t);
            cur = cur.offset(1);
            beg = cur;
        } else {
            cur = cur.offset(1);
            cur;
        }
    }
    add_literal_segment(tf, beg, cur);
    rpl_free(str as *mut libc::c_void);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn set_transform_expr(mut expr: *const i8) {
    while *expr != 0 {
        expr = parse_transform_expr(expr);
    }
}
unsafe extern "C" fn run_case_conv(
    mut case_ctl: case_ctl_type,
    mut ptr: *mut i8,
    mut size: size_t,
) -> *mut i8 {
    static mut case_ctl_buffer: *mut i8 = 0 as *const i8 as *mut i8;
    static mut case_ctl_bufsize: size_t = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    if case_ctl_bufsize < size {
        case_ctl_bufsize = size;
        case_ctl_buffer = xrealloc(
            case_ctl_buffer as *mut libc::c_void,
            case_ctl_bufsize,
        ) as *mut i8;
    }
    memcpy(case_ctl_buffer as *mut libc::c_void, ptr as *const libc::c_void, size);
    match case_ctl as u32 {
        1 => {
            *case_ctl_buffer.offset(0 as i32 as isize) = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *case_ctl_buffer.offset(0 as i32 as isize)
                            as u8 as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(
                            *case_ctl_buffer.offset(0 as i32 as isize) as u8 as i32,
                        );
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(
                            *case_ctl_buffer.offset(0 as i32 as isize) as u8 as i32
                                as isize,
                        );
                }
                __res
            }) as i8;
        }
        2 => {
            *case_ctl_buffer.offset(0 as i32 as isize) = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *case_ctl_buffer.offset(0 as i32 as isize)
                            as u8 as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(
                            *case_ctl_buffer.offset(0 as i32 as isize) as u8 as i32,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *case_ctl_buffer.offset(0 as i32 as isize) as u8 as i32
                                as isize,
                        );
                }
                __res
            }) as i8;
        }
        3 => {
            p = case_ctl_buffer;
            while p < case_ctl_buffer.offset(size as isize) {
                *p = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *p as u8 as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*p as u8 as i32);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*p as u8 as i32 as isize);
                    }
                    __res
                }) as i8;
                p = p.offset(1);
                p;
            }
        }
        4 => {
            p = case_ctl_buffer;
            while p < case_ctl_buffer.offset(size as isize) {
                *p = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *p as u8 as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*p as u8 as i32);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*p as u8 as i32 as isize);
                    }
                    __res
                }) as i8;
                p = p.offset(1);
                p;
            }
        }
        0 | _ => {}
    }
    return case_ctl_buffer;
}
static mut stk: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const i8 as *mut i8,
    next_free: 0 as *const i8 as *mut i8,
    chunk_limit: 0 as *const i8 as *mut i8,
    temp: C2RustUnnamed_1 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_0 { plain: None },
    freefun: C2RustUnnamed { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut stk_init: bool = false;
unsafe extern "C" fn _single_transform_name_to_obstack(
    mut tf: *mut transform,
    mut input: *mut i8,
) {
    let mut rmp: *mut regmatch_t = 0 as *mut regmatch_t;
    let mut rc: i32 = 0;
    let mut nmatches: size_t = 0 as i32 as size_t;
    let mut case_ctl: case_ctl_type = case_ctl_type::ctl_stop;
    let mut save_ctl: case_ctl_type = case_ctl_type::ctl_stop;
    rmp = xmalloc(
        ((*tf).regex.re_nsub)
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<regmatch_t>() as u64),
    ) as *mut regmatch_t;
    while *input != 0 {
        let mut disp: size_t = 0;
        let mut ptr: *mut i8 = 0 as *mut i8;
        rc = rpl_regexec(
            &mut (*tf).regex,
            input,
            ((*tf).regex.re_nsub).wrapping_add(1 as i32 as u64),
            rmp,
            0 as i32,
        );
        if rc == 0 as i32 {
            let mut segm: *mut replace_segm = 0 as *mut replace_segm;
            disp = (*rmp.offset(0 as i32 as isize)).rm_eo as size_t;
            if (*rmp.offset(0 as i32 as isize)).rm_so != 0 {
                let mut __o: *mut obstack = &mut stk;
                let mut __len: size_t = (*rmp.offset(0 as i32 as isize)).rm_so as size_t;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
                }) < __len
                {
                    _obstack_newchunk(__o, __len);
                }
                memcpy(
                    (*__o).next_free as *mut libc::c_void,
                    input as *const libc::c_void,
                    __len,
                );
                (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            }
            nmatches = nmatches.wrapping_add(1);
            nmatches;
            if (*tf).match_number != 0 && nmatches < (*tf).match_number as u64 {
                let mut __o_0: *mut obstack = &mut stk;
                let mut __len_0: size_t = disp;
                if ({
                    let mut __o1: *const obstack = __o_0;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
                }) < __len_0
                {
                    _obstack_newchunk(__o_0, __len_0);
                }
                memcpy(
                    (*__o_0).next_free as *mut libc::c_void,
                    input as *const libc::c_void,
                    __len_0,
                );
                (*__o_0).next_free = ((*__o_0).next_free).offset(__len_0 as isize);
                input = input.offset(disp as isize);
                continue;
            } else {
                segm = (*tf).repl_head;
                while !segm.is_null() {
                    match (*segm).type_0 as u32 {
                        0 => {
                            if case_ctl as u32 == case_ctl_type::ctl_stop as i32 as u32 {
                                ptr = (*segm).v.literal.ptr;
                            } else {
                                ptr = run_case_conv(
                                    case_ctl,
                                    (*segm).v.literal.ptr,
                                    (*segm).v.literal.size,
                                );
                                if case_ctl as u32
                                    == case_ctl_type::ctl_upcase_next as i32 as u32
                                    || case_ctl as u32
                                        == case_ctl_type::ctl_locase_next as i32 as u32
                                {
                                    case_ctl = save_ctl;
                                    save_ctl = case_ctl_type::ctl_stop;
                                }
                            }
                            let mut __o_1: *mut obstack = &mut stk;
                            let mut __len_1: size_t = (*segm).v.literal.size;
                            if ({
                                let mut __o1: *const obstack = __o_1;
                                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64
                                    as size_t
                            }) < __len_1
                            {
                                _obstack_newchunk(__o_1, __len_1);
                            }
                            memcpy(
                                (*__o_1).next_free as *mut libc::c_void,
                                ptr as *const libc::c_void,
                                __len_1,
                            );
                            (*__o_1).next_free = ((*__o_1).next_free)
                                .offset(__len_1 as isize);
                        }
                        1 => {
                            if (*rmp.offset((*segm).v.ref_0 as isize)).rm_so
                                != -(1 as i32) as i64
                                && (*rmp.offset((*segm).v.ref_0 as isize)).rm_eo
                                    != -(1 as i32) as i64
                            {
                                let mut size: size_t = ((*rmp
                                    .offset((*segm).v.ref_0 as isize))
                                    .rm_eo - (*rmp.offset((*segm).v.ref_0 as isize)).rm_so)
                                    as size_t;
                                ptr = input
                                    .offset(
                                        (*rmp.offset((*segm).v.ref_0 as isize)).rm_so as isize,
                                    );
                                if case_ctl as u32 != case_ctl_type::ctl_stop as i32 as u32
                                {
                                    ptr = run_case_conv(case_ctl, ptr, size);
                                    if case_ctl as u32
                                        == case_ctl_type::ctl_upcase_next as i32 as u32
                                        || case_ctl as u32
                                            == case_ctl_type::ctl_locase_next as i32 as u32
                                    {
                                        case_ctl = save_ctl;
                                        save_ctl = case_ctl_type::ctl_stop;
                                    }
                                }
                                let mut __o_2: *mut obstack = &mut stk;
                                let mut __len_2: size_t = size;
                                if ({
                                    let mut __o1: *const obstack = __o_2;
                                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64
                                        as size_t
                                }) < __len_2
                                {
                                    _obstack_newchunk(__o_2, __len_2);
                                }
                                memcpy(
                                    (*__o_2).next_free as *mut libc::c_void,
                                    ptr as *const libc::c_void,
                                    __len_2,
                                );
                                (*__o_2).next_free = ((*__o_2).next_free)
                                    .offset(__len_2 as isize);
                            }
                        }
                        2 => {
                            let mut current_block_58: u64;
                            match (*segm).v.ctl as u32 {
                                1 | 2 => {
                                    match save_ctl as u32 {
                                        0 | 3 | 4 => {
                                            save_ctl = case_ctl;
                                        }
                                        _ => {}
                                    }
                                    current_block_58 = 13869049070516366333;
                                }
                                3 | 4 | 0 => {
                                    current_block_58 = 13869049070516366333;
                                }
                                _ => {
                                    current_block_58 = 6545907279487748450;
                                }
                            }
                            match current_block_58 {
                                13869049070516366333 => {
                                    case_ctl = (*segm).v.ctl;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    segm = (*segm).next;
                }
            }
        } else {
            disp = strlen(input);
            let mut __o_3: *mut obstack = &mut stk;
            let mut __len_3: size_t = disp;
            if ({
                let mut __o1: *const obstack = __o_3;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
            }) < __len_3
            {
                _obstack_newchunk(__o_3, __len_3);
            }
            memcpy(
                (*__o_3).next_free as *mut libc::c_void,
                input as *const libc::c_void,
                __len_3,
            );
            (*__o_3).next_free = ((*__o_3).next_free).offset(__len_3 as isize);
        }
        input = input.offset(disp as isize);
        if !((*tf).transform_type as u32
            == transform_type::transform_first as i32 as u32)
        {
            continue;
        }
        let mut __o_4: *mut obstack = &mut stk;
        let mut __len_4: size_t = strlen(input);
        if ({
            let mut __o1: *const obstack = __o_4;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len_4
        {
            _obstack_newchunk(__o_4, __len_4);
        }
        memcpy(
            (*__o_4).next_free as *mut libc::c_void,
            input as *const libc::c_void,
            __len_4,
        );
        (*__o_4).next_free = ((*__o_4).next_free).offset(__len_4 as isize);
        break;
    }
    let mut __o_5: *mut obstack = &mut stk;
    if ({
        let mut __o1: *const obstack = __o_5;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < 1 as i32 as u64
    {
        _obstack_newchunk(__o_5, 1 as i32 as size_t);
    }
    let fresh0 = (*__o_5).next_free;
    (*__o_5).next_free = ((*__o_5).next_free).offset(1);
    *fresh0 = 0 as i32 as i8;
    rpl_free(rmp as *mut libc::c_void);
}
unsafe extern "C" fn _transform_name_to_obstack(
    mut flags: i32,
    mut input: *mut i8,
    mut output: *mut *mut i8,
) -> bool {
    let mut tf: *mut transform = 0 as *mut transform;
    let mut alloced: bool = 0 as i32 != 0;
    if !stk_init {
        _obstack_begin(
            &mut stk,
            0 as i32 as size_t,
            0 as i32 as size_t,
            Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        stk_init = 1 as i32 != 0;
    }
    tf = transform_head;
    while !tf.is_null() {
        if (*tf).flags & flags != 0 {
            _single_transform_name_to_obstack(tf, input);
            input = ({
                let mut __o1: *mut obstack = &mut stk as *mut obstack;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut i8 {
                    (*__o1).set_maybe_empty_object(1 as i32 as u32);
                }
                (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                {
                    (*__o1).object_base
                } else {
                    0 as *mut i8
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut i8
                                }),
                            ) as i64 as u64)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
                    > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            }) as *mut i8;
            alloced = 1 as i32 != 0;
        }
        tf = (*tf).next;
    }
    *output = input;
    return alloced;
}
#[no_mangle]
pub unsafe extern "C" fn transform_name_fp(
    mut pinput: *mut *mut i8,
    mut flags: i32,
    mut fun: Option<unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> *mut i8>,
    mut dat: *mut libc::c_void,
) -> bool {
    let mut str: *mut i8 = 0 as *mut i8;
    let mut ret: bool = _transform_name_to_obstack(flags, *pinput, &mut str);
    if ret {
        assign_string(
            pinput,
            if fun.is_some() {
                fun.expect("non-null function pointer")(str, dat)
            } else {
                str
            },
        );
        let mut __o: *mut obstack = &mut stk;
        let mut __obj: *mut libc::c_void = str as *mut libc::c_void;
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut i8;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
    } else if fun.is_some() {
        *pinput = 0 as *mut i8;
        assign_string(pinput, fun.expect("non-null function pointer")(str, dat));
        rpl_free(str as *mut libc::c_void);
        ret = 1 as i32 != 0;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn transform_name(
    mut pinput: *mut *mut i8,
    mut type_0: i32,
) -> bool {
    return transform_name_fp(pinput, type_0, None, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn transform_program_p() -> bool {
    return !transform_head.is_null();
}