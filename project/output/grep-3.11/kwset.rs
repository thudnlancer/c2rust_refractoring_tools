#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn abort() -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memchr2(s: *const libc::c_void, c1: i32, c2: i32, n: size_t) -> *mut libc::c_void;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type uintptr_t = u64;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwsmatch {
    pub index: idx_t,
    pub offset: idx_t,
    pub size: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwset {
    pub obstack: obstack,
    pub words: idx_t,
    pub trie: *mut trie,
    pub mind: idx_t,
    pub delta: [u8; 256],
    pub next: [*mut trie; 256],
    pub target: *mut i8,
    pub shift: *mut idx_t,
    pub trans: *const i8,
    pub gc1: i32,
    pub gc1help: i32,
    pub gc2: i8,
    pub kwsexec: Option<
        unsafe extern "C" fn(kwset_t, *const i8, idx_t, *mut kwsmatch, bool) -> ptrdiff_t,
    >,
}
pub type kwset_t = *mut kwset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trie {
    pub accepting: ptrdiff_t,
    pub links: *mut tree,
    pub parent: *mut trie,
    pub next: *mut trie,
    pub fail: *mut trie,
    pub depth: idx_t,
    pub shift: idx_t,
    pub maxshift: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree {
    pub llink: *mut tree,
    pub rlink: *mut tree,
    pub trie: *mut trie,
    pub label: u8,
    pub balance: i8,
}
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    L = 0,
    R = 1,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::L => 0,
            C2RustUnnamed_2::R => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            0 => C2RustUnnamed_2::L,
            1 => C2RustUnnamed_2::R,
            _ => panic!("Invalid value for C2RustUnnamed_2: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_2 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_2 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_2 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn add(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn sub(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn mul(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn div(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn rem(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    NCHAR = 256,
}
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_3::NCHAR => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_3 {
        match value {
            256 => C2RustUnnamed_3::NCHAR,
            _ => panic!("Invalid value for C2RustUnnamed_3: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_3 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_3 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_3 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_3 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_3 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn add(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn sub(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn mul(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn div(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn rem(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: i8) -> u8 {
    return ch as u8;
}
unsafe extern "C" fn U(mut ch: i8) -> u8 {
    return to_uchar(ch);
}
#[inline]
unsafe extern "C" fn tr(mut trans: *const i8, mut c: i8) -> i8 {
    return (if !trans.is_null() {
        *trans.offset(U(c) as isize) as i32
    } else {
        c as i32
    }) as i8;
}
#[no_mangle]
pub unsafe extern "C" fn kwsalloc(mut trans: *const i8) -> kwset_t {
    let mut kwset: *mut kwset = xmalloc(::core::mem::size_of::<kwset>() as u64)
        as *mut kwset;
    _obstack_begin(
        &mut (*kwset).obstack,
        0 as i32 as size_t,
        0 as i32 as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    (*kwset).words = 0 as i32 as idx_t;
    (*kwset).trie = ({
        let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = ::core::mem::size_of::<trie>() as u64;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        ({
            let mut __o1: *mut obstack = __h;
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
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    }) as *mut trie;
    (*(*kwset).trie).accepting = 0 as i32 as ptrdiff_t;
    (*(*kwset).trie).links = 0 as *mut tree;
    (*(*kwset).trie).parent = 0 as *mut trie;
    (*(*kwset).trie).next = 0 as *mut trie;
    (*(*kwset).trie).fail = 0 as *mut trie;
    (*(*kwset).trie).depth = 0 as i32 as idx_t;
    (*(*kwset).trie).shift = 0 as i32 as idx_t;
    (*kwset).mind = 9223372036854775807 as i64;
    (*kwset).target = 0 as *mut i8;
    (*kwset).trans = trans;
    (*kwset).kwsexec = Some(
        acexec
            as unsafe extern "C" fn(
                kwset_t,
                *const i8,
                idx_t,
                *mut kwsmatch,
                bool,
            ) -> ptrdiff_t,
    );
    return kwset;
}
#[no_mangle]
pub unsafe extern "C" fn kwsincr(
    mut kwset: kwset_t,
    mut text: *const i8,
    mut len: idx_t,
) {
    if 0 as i32 as i64 <= len {} else {
        unreachable!();
    };
    let mut trie: *mut trie = (*kwset).trie;
    let mut trans: *const i8 = (*kwset).trans;
    let mut reverse: bool = (*kwset).kwsexec
        == Some(
            bmexec
                as unsafe extern "C" fn(
                    kwset_t,
                    *const i8,
                    idx_t,
                    *mut kwsmatch,
                    bool,
                ) -> ptrdiff_t,
        );
    if reverse {
        text = text.offset(len as isize);
    }
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut uc: u8 = (if reverse as i32 != 0 {
            text = text.offset(-1);
            *text as i32
        } else {
            let fresh1 = text;
            text = text.offset(1);
            *fresh1 as i32
        }) as u8;
        let mut label: u8 = (if !trans.is_null() {
            *trans.offset(uc as isize) as i32
        } else {
            uc as i32
        }) as u8;
        let mut cur: *mut tree = (*trie).links;
        let mut links: [*mut tree; 12] = [0 as *mut tree; 12];
        let mut dirs: [C2RustUnnamed_2; 12] = [C2RustUnnamed_2::L; 12];
        links[0 as i32 as usize] = &mut (*trie).links as *mut *mut tree as *mut tree;
        dirs[0 as i32 as usize] = C2RustUnnamed_2::L;
        let mut depth: idx_t = 1 as i32 as idx_t;
        while !cur.is_null() && label as i32 != (*cur).label as i32 {
            links[depth as usize] = cur;
            if (label as i32) < (*cur).label as i32 {
                let fresh2 = depth;
                depth = depth + 1;
                dirs[fresh2 as usize] = C2RustUnnamed_2::L;
                cur = (*cur).llink;
            } else {
                let fresh3 = depth;
                depth = depth + 1;
                dirs[fresh3 as usize] = C2RustUnnamed_2::R;
                cur = (*cur).rlink;
            }
        }
        if cur.is_null() {
            cur = ({
                let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
                let mut __o: *mut obstack = __h;
                let mut __len: size_t = ::core::mem::size_of::<tree>() as u64;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
                }) < __len
                {
                    _obstack_newchunk(__o, __len);
                }
                (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                ({
                    let mut __o1: *mut obstack = __h;
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
                        > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8)
                            as i64 as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                })
            }) as *mut tree;
            (*cur).llink = 0 as *mut tree;
            (*cur).rlink = 0 as *mut tree;
            (*cur).trie = ({
                let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
                let mut __o: *mut obstack = __h;
                let mut __len: size_t = ::core::mem::size_of::<trie>() as u64;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
                }) < __len
                {
                    _obstack_newchunk(__o, __len);
                }
                (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                ({
                    let mut __o1: *mut obstack = __h;
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
                        > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8)
                            as i64 as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                })
            }) as *mut trie;
            (*(*cur).trie).accepting = 0 as i32 as ptrdiff_t;
            (*(*cur).trie).links = 0 as *mut tree;
            (*(*cur).trie).parent = trie;
            (*(*cur).trie).next = 0 as *mut trie;
            (*(*cur).trie).fail = 0 as *mut trie;
            (*(*cur).trie).depth = (*trie).depth + 1 as i32 as i64;
            (*(*cur).trie).shift = 0 as i32 as idx_t;
            (*cur).label = label;
            (*cur).balance = 0 as i32 as i8;
            depth -= 1;
            if dirs[depth as usize] as u32 == C2RustUnnamed_2::L as i32 as u32 {
                (*links[depth as usize]).llink = cur;
            } else {
                (*links[depth as usize]).rlink = cur;
            }
            while depth != 0 && (*links[depth as usize]).balance == 0 {
                if dirs[depth as usize] as u32 == C2RustUnnamed_2::L as i32 as u32 {
                    (*links[depth as usize]).balance -= 1;
                    (*links[depth as usize]).balance;
                } else {
                    (*links[depth as usize]).balance += 1;
                    (*links[depth as usize]).balance;
                }
                depth -= 1;
                depth;
            }
            if depth != 0
                && (dirs[depth as usize] as u32 == C2RustUnnamed_2::L as i32 as u32
                    && {
                        (*links[depth as usize]).balance -= 1;
                        (*links[depth as usize]).balance as i32 != 0
                    }
                    || dirs[depth as usize] as u32 == C2RustUnnamed_2::R as i32 as u32
                        && {
                            (*links[depth as usize]).balance += 1;
                            (*links[depth as usize]).balance as i32 != 0
                        })
            {
                let mut t: *mut tree = 0 as *mut tree;
                let mut r: *mut tree = 0 as *mut tree;
                let mut l: *mut tree = 0 as *mut tree;
                let mut rl: *mut tree = 0 as *mut tree;
                let mut lr: *mut tree = 0 as *mut tree;
                match (*links[depth as usize]).balance as i32 {
                    -2 => {
                        match dirs[(depth + 1 as i32 as i64) as usize] as u32 {
                            0 => {
                                r = links[depth as usize];
                                t = (*r).llink;
                                rl = (*t).rlink;
                                (*t).rlink = r;
                                (*r).llink = rl;
                                (*r).balance = 0 as i32 as i8;
                                (*t).balance = (*r).balance;
                            }
                            1 => {
                                r = links[depth as usize];
                                l = (*r).llink;
                                t = (*l).rlink;
                                rl = (*t).rlink;
                                lr = (*t).llink;
                                (*t).llink = l;
                                (*l).rlink = lr;
                                (*t).rlink = r;
                                (*r).llink = rl;
                                (*l).balance = (if (*t).balance as i32 != 1 as i32 {
                                    0 as i32
                                } else {
                                    -(1 as i32)
                                }) as i8;
                                (*r).balance = (if (*t).balance as i32
                                    != -(1 as i32) as i8 as i32
                                {
                                    0 as i32
                                } else {
                                    1 as i32
                                }) as i8;
                                (*t).balance = 0 as i32 as i8;
                            }
                            _ => {
                                abort();
                            }
                        }
                    }
                    2 => {
                        match dirs[(depth + 1 as i32 as i64) as usize] as u32 {
                            1 => {
                                l = links[depth as usize];
                                t = (*l).rlink;
                                lr = (*t).llink;
                                (*t).llink = l;
                                (*l).rlink = lr;
                                (*l).balance = 0 as i32 as i8;
                                (*t).balance = (*l).balance;
                            }
                            0 => {
                                l = links[depth as usize];
                                r = (*l).rlink;
                                t = (*r).llink;
                                lr = (*t).llink;
                                rl = (*t).rlink;
                                (*t).llink = l;
                                (*l).rlink = lr;
                                (*t).rlink = r;
                                (*r).llink = rl;
                                (*l).balance = (if (*t).balance as i32 != 1 as i32 {
                                    0 as i32
                                } else {
                                    -(1 as i32)
                                }) as i8;
                                (*r).balance = (if (*t).balance as i32
                                    != -(1 as i32) as i8 as i32
                                {
                                    0 as i32
                                } else {
                                    1 as i32
                                }) as i8;
                                (*t).balance = 0 as i32 as i8;
                            }
                            _ => {
                                abort();
                            }
                        }
                    }
                    _ => {
                        abort();
                    }
                }
                if dirs[(depth - 1 as i32 as i64) as usize] as u32
                    == C2RustUnnamed_2::L as i32 as u32
                {
                    (*links[(depth - 1 as i32 as i64) as usize]).llink = t;
                } else {
                    (*links[(depth - 1 as i32 as i64) as usize]).rlink = t;
                }
            }
        }
        trie = (*cur).trie;
    }
    if (*trie).accepting == 0 {
        (*trie).accepting = 2 as i32 as i64 * (*kwset).words + 1 as i32 as i64;
    }
    (*kwset).words += 1;
    (*kwset).words;
    if (*trie).depth < (*kwset).mind {
        (*kwset).mind = (*trie).depth;
    }
}
#[no_mangle]
pub unsafe extern "C" fn kwswords(mut kwset: kwset_t) -> idx_t {
    return (*kwset).words;
}
unsafe extern "C" fn enqueue(mut tree: *mut tree, mut last: *mut *mut trie) {
    if tree.is_null() {
        return;
    }
    enqueue((*tree).llink, last);
    enqueue((*tree).rlink, last);
    (**last).next = (*tree).trie;
    *last = (**last).next;
}
unsafe extern "C" fn treefails(
    mut tree: *const tree,
    mut fail: *const trie,
    mut recourse: *mut trie,
    mut reverse: bool,
) {
    let mut cur: *mut tree = 0 as *mut tree;
    if tree.is_null() {
        return;
    }
    treefails((*tree).llink, fail, recourse, reverse);
    treefails((*tree).rlink, fail, recourse, reverse);
    while !fail.is_null() {
        cur = (*fail).links;
        while !cur.is_null() && (*tree).label as i32 != (*cur).label as i32 {
            if ((*tree).label as i32) < (*cur).label as i32 {
                cur = (*cur).llink;
            } else {
                cur = (*cur).rlink;
            }
        }
        if !cur.is_null() {
            (*(*tree).trie).fail = (*cur).trie;
            if !reverse && (*(*cur).trie).accepting != 0
                && (*(*tree).trie).accepting == 0
            {
                (*(*tree).trie).accepting = -(1 as i32) as ptrdiff_t;
            }
            return;
        }
        fail = (*fail).fail;
    }
    (*(*tree).trie).fail = recourse;
}
unsafe extern "C" fn treedelta(
    mut tree: *const tree,
    mut depth: idx_t,
    mut delta: *mut u8,
) {
    if tree.is_null() {
        return;
    }
    treedelta((*tree).llink, depth, delta);
    treedelta((*tree).rlink, depth, delta);
    if depth < *delta.offset((*tree).label as isize) as i64 {
        *delta.offset((*tree).label as isize) = depth as u8;
    }
}
unsafe extern "C" fn hasevery(mut a: *const tree, mut b: *const tree) -> bool {
    if b.is_null() {
        return 1 as i32 != 0;
    }
    if !hasevery(a, (*b).llink) {
        return 0 as i32 != 0;
    }
    if !hasevery(a, (*b).rlink) {
        return 0 as i32 != 0;
    }
    while !a.is_null() && (*b).label as i32 != (*a).label as i32 {
        if ((*b).label as i32) < (*a).label as i32 {
            a = (*a).llink;
        } else {
            a = (*a).rlink;
        }
    }
    return !a.is_null();
}
unsafe extern "C" fn treenext(mut tree: *const tree, mut next: *mut *mut trie) {
    if tree.is_null() {
        return;
    }
    treenext((*tree).llink, next);
    treenext((*tree).rlink, next);
    let ref mut fresh4 = *next.offset((*tree).label as isize);
    *fresh4 = (*tree).trie;
}
#[no_mangle]
pub unsafe extern "C" fn kwsprep(mut kwset: kwset_t) {
    let mut trans: *const i8 = (*kwset).trans;
    let mut deltabuf: [u8; 256] = [0; 256];
    let mut delta: *mut u8 = if !trans.is_null() {
        deltabuf.as_mut_ptr()
    } else {
        ((*kwset).delta).as_mut_ptr()
    };
    let mut curr: *mut trie = 0 as *mut trie;
    let mut last: *mut trie = 0 as *mut trie;
    let mut reverse: bool = (*kwset).words == 1 as i32 as i64;
    if reverse {
        let mut new_kwset: kwset_t = 0 as *mut kwset;
        last = (*kwset).trie;
        curr = last;
        while !curr.is_null() {
            enqueue((*curr).links, &mut last);
            curr = (*curr).next;
        }
        (*kwset).target = ({
            let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
            let mut __o: *mut obstack = __h;
            let mut __len: size_t = (*kwset).mind as size_t;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            ({
                let mut __o1: *mut obstack = __h;
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
            })
        }) as *mut i8;
        curr = (*kwset).trie;
        let mut i: idx_t = 0 as i32 as idx_t;
        while i < (*kwset).mind {
            *((*kwset).target).offset(i as isize) = (*(*curr).links).label as i8;
            curr = (*curr).next;
            i += 1;
            i;
        }
        new_kwset = kwsalloc((*kwset).trans);
        (*new_kwset).kwsexec = Some(
            bmexec
                as unsafe extern "C" fn(
                    kwset_t,
                    *const i8,
                    idx_t,
                    *mut kwsmatch,
                    bool,
                ) -> ptrdiff_t,
        );
        kwsincr(new_kwset, (*kwset).target, (*kwset).mind);
        let mut __o: *mut obstack = &mut (*kwset).obstack;
        let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut i8;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
        *kwset = *new_kwset;
        rpl_free(new_kwset as *mut libc::c_void);
    }
    memset(
        delta as *mut libc::c_void,
        (if (*kwset).mind < (127 as i32 * 2 as i32 + 1 as i32) as i64 {
            (*kwset).mind
        } else {
            (127 as i32 * 2 as i32 + 1 as i32) as i64
        }) as i32,
        ::core::mem::size_of::<[u8; 256]>() as u64,
    );
    last = (*kwset).trie;
    curr = last;
    while !curr.is_null() {
        enqueue((*curr).links, &mut last);
        treedelta((*curr).links, (*curr).depth, delta);
        treefails((*curr).links, (*curr).fail, (*kwset).trie, reverse);
        if reverse {
            (*curr).shift = (*kwset).mind;
            (*curr).maxshift = (*kwset).mind;
            let mut fail: *mut trie = 0 as *mut trie;
            fail = (*curr).fail;
            while !fail.is_null() {
                if !hasevery((*fail).links, (*curr).links) {
                    if (*curr).depth - (*fail).depth < (*fail).shift {
                        (*fail).shift = (*curr).depth - (*fail).depth;
                    }
                }
                if (*curr).accepting != 0
                    && (*fail).maxshift > (*curr).depth - (*fail).depth
                {
                    (*fail).maxshift = (*curr).depth - (*fail).depth;
                }
                fail = (*fail).fail;
            }
        }
        curr = (*curr).next;
    }
    if reverse {
        curr = (*(*kwset).trie).next;
        while !curr.is_null() {
            if (*curr).maxshift > (*(*curr).parent).maxshift {
                (*curr).maxshift = (*(*curr).parent).maxshift;
            }
            if (*curr).shift > (*curr).maxshift {
                (*curr).shift = (*curr).maxshift;
            }
            curr = (*curr).next;
        }
    }
    let mut nextbuf: [*mut trie; 256] = [0 as *mut trie; 256];
    let mut next: *mut *mut trie = if !trans.is_null() {
        nextbuf.as_mut_ptr()
    } else {
        ((*kwset).next).as_mut_ptr()
    };
    memset(
        next as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[*mut trie; 256]>() as u64,
    );
    treenext((*(*kwset).trie).links, next);
    let mut gc1: i32 = -(2 as i32);
    let mut gc1help: i32 = -(1 as i32);
    let mut i_0: i32 = 0 as i32;
    while i_0 < C2RustUnnamed_3::NCHAR as i32 {
        let mut ti: i32 = i_0;
        if !trans.is_null() {
            ti = U(*trans.offset(i_0 as isize)) as i32;
            (*kwset).next[i_0 as usize] = *next.offset(ti as isize);
        }
        if !((*kwset).next[i_0 as usize]).is_null() {
            if gc1 < -(1 as i32) {
                gc1 = ti;
                gc1help = i_0;
            } else if gc1 == ti {
                gc1help = if gc1help == ti { i_0 } else { -(1 as i32) };
            } else if i_0 == ti && gc1 == gc1help {
                gc1help = i_0;
            } else {
                gc1 = -(1 as i32);
            }
        }
        i_0 += 1;
        i_0;
    }
    (*kwset).gc1 = gc1;
    (*kwset).gc1help = gc1help;
    if reverse {
        (*kwset).target = ({
            let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
            let mut __o_0: *mut obstack = __h;
            let mut __len: size_t = (*kwset).mind as size_t;
            if ({
                let mut __o1: *const obstack = __o_0;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
            }) < __len
            {
                _obstack_newchunk(__o_0, __len);
            }
            (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
            ({
                let mut __o1: *mut obstack = __h;
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
            })
        }) as *mut i8;
        curr = (*kwset).trie;
        let mut i_1: idx_t = (*kwset).mind;
        while (0 as i32 as i64) < i_1 {
            *((*kwset).target).offset((i_1 - 1 as i32 as i64) as isize) = (*(*curr)
                .links)
                .label as i8;
            curr = (*curr).next;
            i_1 -= 1;
            i_1;
        }
        if (*kwset).mind > 1 as i32 as i64 {
            (*kwset).shift = ({
                let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
                let mut __o_0: *mut obstack = __h;
                let mut __len: size_t = (::core::mem::size_of::<idx_t>() as u64)
                    .wrapping_mul(((*kwset).mind - 1 as i32 as i64) as u64);
                if ({
                    let mut __o1: *const obstack = __o_0;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
                }) < __len
                {
                    _obstack_newchunk(__o_0, __len);
                }
                (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
                ({
                    let mut __o1: *mut obstack = __h;
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
                        > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8)
                            as i64 as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                })
            }) as *mut idx_t;
            curr = (*(*kwset).trie).next;
            let mut i_2: idx_t = 0 as i32 as idx_t;
            while i_2 < (*kwset).mind - 1 as i32 as i64 {
                *((*kwset).shift).offset(i_2 as isize) = (*curr).shift;
                curr = (*curr).next;
                i_2 += 1;
                i_2;
            }
            (*kwset).gc2 = tr(
                trans,
                *((*kwset).target).offset(((*kwset).mind - 2 as i32 as i64) as isize),
            );
        }
    }
    if !trans.is_null() {
        let mut i_3: i32 = 0 as i32;
        while i_3 < C2RustUnnamed_3::NCHAR as i32 {
            (*kwset).delta[i_3 as usize] = *delta
                .offset(U(*trans.offset(i_3 as isize)) as isize);
            i_3 += 1;
            i_3;
        }
    }
}
#[inline]
unsafe extern "C" fn bm_delta2_search(
    mut tpp: *mut *const i8,
    mut ep: *const i8,
    mut sp: *const i8,
    mut len: idx_t,
    mut trans: *const i8,
    mut gc1: i8,
    mut gc2: i8,
    mut d1: *const u8,
    mut kwset: kwset_t,
) -> bool {
    let mut tp: *const i8 = *tpp;
    let mut d: idx_t = len;
    let mut skip: idx_t = 0 as i32 as idx_t;
    loop {
        let mut i: idx_t = 2 as i32 as idx_t;
        if tr(trans, *tp.offset(-(2 as i32) as isize)) as i32 == gc2 as i32 {
            loop {
                i += 1;
                if !(i <= d) {
                    break;
                }
                if tr(trans, *tp.offset(-i as isize)) as i32
                    != tr(trans, *sp.offset(-i as isize)) as i32
                {
                    break;
                }
            }
            if i > d {
                i = d + skip + 1 as i32 as i64;
                while i <= len {
                    if tr(trans, *tp.offset(-i as isize)) as i32
                        != tr(trans, *sp.offset(-i as isize)) as i32
                    {
                        break;
                    }
                    i += 1;
                    i;
                }
                if i > len {
                    *tpp = tp.offset(-(len as isize));
                    return 1 as i32 != 0;
                }
            }
        }
        d = *((*kwset).shift).offset((i - 2 as i32 as i64) as isize);
        tp = tp.offset(d as isize);
        if tp > ep {
            break;
        }
        if tr(trans, *tp.offset(-(1 as i32) as isize)) as i32 != gc1 as i32 {
            if !d1.is_null() {
                tp = tp
                    .offset(
                        *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32
                            as isize,
                    );
            }
            break;
        } else {
            skip = i - 1 as i32 as i64;
        }
    }
    *tpp = tp;
    return 0 as i32 != 0;
}
unsafe extern "C" fn memchr_kwset(
    mut s: *const i8,
    mut n: idx_t,
    mut kwset: kwset_t,
) -> *const i8 {
    let mut slim: *const i8 = s.offset(n as isize);
    if (*kwset).gc1help < 0 as i32 {
        while s < slim {
            if !((*kwset).next[U(*s) as usize]).is_null() {
                return s;
            }
            s = s.offset(1);
            s;
        }
    } else {
        let mut small_heuristic: i32 = 2 as i32;
        let mut small_bytes: idx_t = (small_heuristic as u64)
            .wrapping_mul(::core::mem::size_of::<u64>() as u64) as idx_t;
        while s < slim {
            if !((*kwset).next[U(*s) as usize]).is_null() {
                return s;
            }
            s = s.offset(1);
            s;
            if (s as uintptr_t).wrapping_rem(small_bytes as u64) == 0 as i32 as u64 {
                return memchr2(
                    s as *const libc::c_void,
                    (*kwset).gc1,
                    (*kwset).gc1help,
                    slim.offset_from(s) as i64 as size_t,
                ) as *const i8;
            }
        }
    }
    return 0 as *const i8;
}
#[inline]
unsafe extern "C" fn bmexec_trans(
    mut kwset: kwset_t,
    mut text: *const i8,
    mut size: idx_t,
) -> ptrdiff_t {
    if 0 as i32 as i64 <= size {} else {
        unreachable!();
    };
    let mut d1: *const u8 = 0 as *const u8;
    let mut ep: *const i8 = 0 as *const i8;
    let mut sp: *const i8 = 0 as *const i8;
    let mut tp: *const i8 = 0 as *const i8;
    let mut d: i32 = 0;
    let mut len: idx_t = (*kwset).mind;
    let mut trans: *const i8 = (*kwset).trans;
    if len == 0 as i32 as i64 {
        return 0 as i32 as ptrdiff_t;
    }
    if len > size {
        return -(1 as i32) as ptrdiff_t;
    }
    if len == 1 as i32 as i64 {
        tp = memchr_kwset(text, size, kwset);
        return if !tp.is_null() {
            tp.offset_from(text) as i64
        } else {
            -(1 as i32) as i64
        };
    }
    d1 = ((*kwset).delta).as_mut_ptr();
    sp = ((*kwset).target).offset(len as isize);
    tp = text.offset(len as isize);
    let mut gc1: i8 = (*kwset).gc1 as i8;
    let mut gc2: i8 = (*kwset).gc2;
    let mut len12: idx_t = 0;
    if !((if ::core::mem::size_of::<idx_t>() as u64
        == ::core::mem::size_of::<libc::c_schar>() as u64
    {
        (if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
            (if (if (12 as i32) < 0 as i32 {
                (if len < 0 as i32 as i64 {
                    (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 }) + 12 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        (len < (127 as i32 / 12 as i32) as i64) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                            - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32
                        }) < 0 as i32
                        {
                            ((12 as i32)
                                < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < 12 as i32) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + 127 as i32
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            127 as i32 / -(12 as i32)
                        }) as i64 <= -(1 as i32) as i64 - len) as i32
                    })
                } else {
                    (if (if (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                            + (-(127 as i32) - 1 as i32)
                    }) - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + (-(127 as i32) - 1 as i32)
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + (-(127 as i32) - 1 as i32)
                        }) + 0 as i32
                    }) < 0 as i32
                    {
                        ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                            + (-(127 as i32) - 1 as i32)
                            < -(if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (-(127 as i32) - 1 as i32)
                            }) - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + (-(127 as i32) - 1 as i32)
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + (-(127 as i32) - 1 as i32)
                                }) - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32)
                            < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + (-(127 as i32) - 1 as i32)) as i32
                    }) != 0 && 12 as i32 == -(1 as i32)
                    {
                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((0 as i32 as i64) < len + (-(127 as i32) - 1 as i32) as i64)
                                as i32
                        } else {
                            ((0 as i32 as i64) < len
                                && ((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                                    < len - 1 as i32 as i64) as i32
                        })
                    } else {
                        ((((-(127 as i32) - 1 as i32) / 12 as i32) as i64) < len) as i32
                    })
                })
            } else {
                (if 12 as i32 == 0 as i32 {
                    0 as i32
                } else {
                    (if len < 0 as i32 as i64 {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                + (-(127 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                + (-(127 as i32) - 1 as i32) as i64)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + (-(127 as i32) - 1 as i32) as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(127 as i32) - 1 as i32) as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(127 as i32) - 1 as i32) as i64
                                    }) - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64)
                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + (-(127 as i32) - 1 as i32) as i64) as i32
                        }) != 0 && len == -(1 as i32) as i64
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                - 1 as i32) < 0 as i32
                            {
                                ((0 as i32) < 12 as i32 + (-(127 as i32) - 1 as i32)) as i32
                            } else {
                                (-(1 as i32) - (-(127 as i32) - 1 as i32)
                                    < 12 as i32 - 1 as i32) as i32
                            })
                        } else {
                            ((-(127 as i32) - 1 as i32) as i64 / len < 12 as i32 as i64)
                                as i32
                        })
                    } else {
                        (((127 as i32 / 12 as i32) as i64) < len) as i32
                    })
                })
            }) != 0
            {
                len12 = (len as u32).wrapping_mul(12 as i32 as u32) as libc::c_schar
                    as idx_t;
                1 as i32
            } else {
                len12 = (len as u32).wrapping_mul(12 as i32 as u32) as libc::c_schar
                    as idx_t;
                0 as i32
            })
        } else {
            (if (if (12 as i32) < 0 as i32 {
                (if len < 0 as i32 as i64 {
                    (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            127 as i32 * 2 as i32 + 1 as i32
                        }) + 12 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        (len < ((127 as i32 * 2 as i32 + 1 as i32) / 12 as i32) as i64)
                            as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                            - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32
                        }) < 0 as i32
                        {
                            ((12 as i32)
                                < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < 12 as i32) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + (127 as i32 * 2 as i32 + 1 as i32)
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            (127 as i32 * 2 as i32 + 1 as i32) / -(12 as i32)
                        }) as i64 <= -(1 as i32) as i64 - len) as i32
                    })
                } else {
                    (if (if (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32
                        }) + 0 as i32
                    }) < 0 as i32
                    {
                        (((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32)
                            < -(if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32)
                            < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + 0 as i32) as i32
                    }) != 0 && 12 as i32 == -(1 as i32)
                    {
                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((0 as i32 as i64) < len + 0 as i32 as i64) as i32
                        } else {
                            ((0 as i32 as i64) < len
                                && ((-(1 as i32) - 0 as i32) as i64)
                                    < len - 1 as i32 as i64) as i32
                        })
                    } else {
                        (((0 as i32 / 12 as i32) as i64) < len) as i32
                    })
                })
            } else {
                (if 12 as i32 == 0 as i32 {
                    0 as i32
                } else {
                    (if len < 0 as i32 as i64 {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + 0 as i32 as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + 0 as i32 as i64
                            }) + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                + 0 as i32 as i64)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + 0 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64)
                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + 0 as i32 as i64) as i32
                        }) != 0 && len == -(1 as i32) as i64
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                - 1 as i32) < 0 as i32
                            {
                                ((0 as i32) < 12 as i32 + 0 as i32) as i32
                            } else {
                                ((-(1 as i32) - 0 as i32) < 12 as i32 - 1 as i32) as i32
                            })
                        } else {
                            (0 as i32 as i64 / len < 12 as i32 as i64) as i32
                        })
                    } else {
                        ((((127 as i32 * 2 as i32 + 1 as i32) / 12 as i32) as i64) < len)
                            as i32
                    })
                })
            }) != 0
            {
                len12 = (len as u32).wrapping_mul(12 as i32 as u32) as u8 as idx_t;
                1 as i32
            } else {
                len12 = (len as u32).wrapping_mul(12 as i32 as u32) as u8 as idx_t;
                0 as i32
            })
        })
    } else {
        (if ::core::mem::size_of::<idx_t>() as u64
            == ::core::mem::size_of::<libc::c_short>() as u64
        {
            (if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
                (if (if (12 as i32) < 0 as i32 {
                    (if len < 0 as i32 as i64 {
                        (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 })
                                + 12 as i32
                        }) - 1 as i32) < 0 as i32
                        {
                            (len < (32767 as i32 / 12 as i32) as i64) as i32
                        } else {
                            ((if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                12 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32
                            }) < 0 as i32
                            {
                                ((12 as i32)
                                    < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32) < 12 as i32) as i32
                            }) != 0
                            {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 32767 as i32
                                    >> (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                            } else {
                                32767 as i32 / -(12 as i32)
                            }) as i64 <= -(1 as i32) as i64 - len) as i32
                        })
                    } else {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + (-(32767 as i32) - 1 as i32)
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (-(32767 as i32) - 1 as i32)
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (-(32767 as i32) - 1 as i32)
                            }) + 0 as i32
                        }) < 0 as i32
                        {
                            ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + (-(32767 as i32) - 1 as i32)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + (-(32767 as i32) - 1 as i32)
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + (-(32767 as i32) - 1 as i32)
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + (-(32767 as i32) - 1 as i32)
                                    }) - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32)
                                < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (-(32767 as i32) - 1 as i32)) as i32
                        }) != 0 && 12 as i32 == -(1 as i32)
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((0 as i32 as i64)
                                    < len + (-(32767 as i32) - 1 as i32) as i64) as i32
                            } else {
                                ((0 as i32 as i64) < len
                                    && ((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                                        < len - 1 as i32 as i64) as i32
                            })
                        } else {
                            ((((-(32767 as i32) - 1 as i32) / 12 as i32) as i64) < len)
                                as i32
                        })
                    })
                } else {
                    (if 12 as i32 == 0 as i32 {
                        0 as i32
                    } else {
                        (if len < 0 as i32 as i64 {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + (-(32767 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + (-(32767 as i32) - 1 as i32) as i64
                                }) + 1 as i32 as i64)
                                    << (::core::mem::size_of::<i64>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + (-(32767 as i32) - 1 as i32) as i64
                                }) + 0 as i32 as i64
                            }) < 0 as i32 as i64
                            {
                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + (-(32767 as i32) - 1 as i32) as i64)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(32767 as i32) - 1 as i32) as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + (-(32767 as i32) - 1 as i32) as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + (-(32767 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64
                                    })) as i32
                            } else {
                                ((0 as i32 as i64)
                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + (-(32767 as i32) - 1 as i32) as i64) as i32
                            }) != 0 && len == -(1 as i32) as i64
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((0 as i32) < 12 as i32 + (-(32767 as i32) - 1 as i32))
                                        as i32
                                } else {
                                    (-(1 as i32) - (-(32767 as i32) - 1 as i32)
                                        < 12 as i32 - 1 as i32) as i32
                                })
                            } else {
                                ((-(32767 as i32) - 1 as i32) as i64 / len
                                    < 12 as i32 as i64) as i32
                            })
                        } else {
                            (((32767 as i32 / 12 as i32) as i64) < len) as i32
                        })
                    })
                }) != 0
                {
                    len12 = (len as u32).wrapping_mul(12 as i32 as u32) as libc::c_short
                        as idx_t;
                    1 as i32
                } else {
                    len12 = (len as u32).wrapping_mul(12 as i32 as u32) as libc::c_short
                        as idx_t;
                    0 as i32
                })
            } else {
                (if (if (12 as i32) < 0 as i32 {
                    (if len < 0 as i32 as i64 {
                        (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                32767 as i32 * 2 as i32 + 1 as i32
                            }) + 12 as i32
                        }) - 1 as i32) < 0 as i32
                        {
                            (len
                                < ((32767 as i32 * 2 as i32 + 1 as i32) / 12 as i32) as i64)
                                as i32
                        } else {
                            ((if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                12 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32
                            }) < 0 as i32
                            {
                                ((12 as i32)
                                    < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32) < 12 as i32) as i32
                            }) != 0
                            {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (32767 as i32 * 2 as i32 + 1 as i32)
                                    >> (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                            } else {
                                (32767 as i32 * 2 as i32 + 1 as i32) / -(12 as i32)
                            }) as i64 <= -(1 as i32) as i64 - len) as i32
                        })
                    } else {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) + 0 as i32
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32
                            }) + 0 as i32
                        }) < 0 as i32
                        {
                            (((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                + 0 as i32)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32)
                                < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32) as i32
                        }) != 0 && 12 as i32 == -(1 as i32)
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((0 as i32 as i64) < len + 0 as i32 as i64) as i32
                            } else {
                                ((0 as i32 as i64) < len
                                    && ((-(1 as i32) - 0 as i32) as i64)
                                        < len - 1 as i32 as i64) as i32
                            })
                        } else {
                            (((0 as i32 / 12 as i32) as i64) < len) as i32
                        })
                    })
                } else {
                    (if 12 as i32 == 0 as i32 {
                        0 as i32
                    } else {
                        (if len < 0 as i32 as i64 {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + 0 as i32 as i64
                            }) - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + 0 as i32 as i64
                                }) + 1 as i32 as i64)
                                    << (::core::mem::size_of::<i64>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + 0 as i32 as i64
                                }) + 0 as i32 as i64
                            }) < 0 as i32 as i64
                            {
                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    + 0 as i32 as i64)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64
                                    })) as i32
                            } else {
                                ((0 as i32 as i64)
                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + 0 as i32 as i64) as i32
                            }) != 0 && len == -(1 as i32) as i64
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((0 as i32) < 12 as i32 + 0 as i32) as i32
                                } else {
                                    ((-(1 as i32) - 0 as i32) < 12 as i32 - 1 as i32) as i32
                                })
                            } else {
                                (0 as i32 as i64 / len < 12 as i32 as i64) as i32
                            })
                        } else {
                            ((((32767 as i32 * 2 as i32 + 1 as i32) / 12 as i32) as i64)
                                < len) as i32
                        })
                    })
                }) != 0
                {
                    len12 = (len as u32).wrapping_mul(12 as i32 as u32) as libc::c_ushort
                        as idx_t;
                    1 as i32
                } else {
                    len12 = (len as u32).wrapping_mul(12 as i32 as u32) as libc::c_ushort
                        as idx_t;
                    0 as i32
                })
            })
        } else {
            (if ::core::mem::size_of::<idx_t>() as u64
                == ::core::mem::size_of::<i32>() as u64
            {
                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len12 })
                    - 1 as i32 as i64) < 0 as i32 as i64
                {
                    (if (if (12 as i32) < 0 as i32 {
                        (if len < 0 as i32 as i64 {
                            (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 })
                                    + 12 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                (len < (2147483647 as i32 / 12 as i32) as i64) as i32
                            } else {
                                ((if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    12 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) < 0 as i32
                                {
                                    ((12 as i32)
                                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32) < 12 as i32) as i32
                                }) != 0
                                {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 2147483647 as i32
                                        >> (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                } else {
                                    2147483647 as i32 / -(12 as i32)
                                }) as i64 <= -(1 as i32) as i64 - len) as i32
                            })
                        } else {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (-(2147483647 as i32) - 1 as i32)
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + (-(2147483647 as i32) - 1 as i32)
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + (-(2147483647 as i32) - 1 as i32)
                                }) + 0 as i32
                            }) < 0 as i32
                            {
                                ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + (-(2147483647 as i32) - 1 as i32)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + (-(2147483647 as i32) - 1 as i32)
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + (-(2147483647 as i32) - 1 as i32)
                                        }) + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + (-(2147483647 as i32) - 1 as i32)
                                        }) - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32)
                                    < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + (-(2147483647 as i32) - 1 as i32)) as i32
                            }) != 0 && 12 as i32 == -(1 as i32)
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((0 as i32 as i64)
                                        < len + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                } else {
                                    ((0 as i32 as i64) < len
                                        && ((-(1 as i32) - (-(2147483647 as i32) - 1 as i32))
                                            as i64) < len - 1 as i32 as i64) as i32
                                })
                            } else {
                                ((((-(2147483647 as i32) - 1 as i32) / 12 as i32) as i64)
                                    < len) as i32
                            })
                        })
                    } else {
                        (if 12 as i32 == 0 as i32 {
                            0 as i32
                        } else {
                            (if len < 0 as i32 as i64 {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + (-(2147483647 as i32) - 1 as i32) as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(2147483647 as i32) - 1 as i32) as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(2147483647 as i32) - 1 as i32) as i64
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + (-(2147483647 as i32) - 1 as i32) as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + (-(2147483647 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                }) != 0 && len == -(1 as i32) as i64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((0 as i32) < 12 as i32 + (-(2147483647 as i32) - 1 as i32))
                                            as i32
                                    } else {
                                        (-(1 as i32) - (-(2147483647 as i32) - 1 as i32)
                                            < 12 as i32 - 1 as i32) as i32
                                    })
                                } else {
                                    ((-(2147483647 as i32) - 1 as i32) as i64 / len
                                        < 12 as i32 as i64) as i32
                                })
                            } else {
                                (((2147483647 as i32 / 12 as i32) as i64) < len) as i32
                            })
                        })
                    }) != 0
                    {
                        len12 = (len as u32).wrapping_mul(12 as i32 as u32) as i32
                            as idx_t;
                        1 as i32
                    } else {
                        len12 = (len as u32).wrapping_mul(12 as i32 as u32) as i32
                            as idx_t;
                        0 as i32
                    })
                } else {
                    (if (if (12 as i32) < 0 as i32 {
                        (if len < 0 as i32 as i64 {
                            (if (if 1 as i32 != 0 {
                                0 as i32 as u32
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32 as u32
                                } else {
                                    (2147483647 as i32 as u32)
                                        .wrapping_mul(2 as u32)
                                        .wrapping_add(1 as u32)
                                })
                                    .wrapping_add(12 as i32 as u32)
                            })
                                .wrapping_sub(1 as i32 as u32) < 0 as i32 as u32
                            {
                                (len
                                    < (2147483647 as i32 as u32)
                                        .wrapping_mul(2 as u32)
                                        .wrapping_add(1 as u32)
                                        .wrapping_div(12 as i32 as u32) as i64) as i32
                            } else {
                                ((if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    12 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) < 0 as i32
                                {
                                    ((12 as i32)
                                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32) < 12 as i32) as i32
                                }) != 0
                                {
                                    ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as u32)
                                        .wrapping_add(
                                            (2147483647 as i32 as u32)
                                                .wrapping_mul(2 as u32)
                                                .wrapping_add(1 as u32),
                                        )
                                        >> (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                } else {
                                    (2147483647 as i32 as u32)
                                        .wrapping_mul(2 as u32)
                                        .wrapping_add(1 as u32)
                                        .wrapping_div(-(12 as i32) as u32)
                                }) as i64 <= -(1 as i32) as i64 - len) as i32
                            })
                        } else {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) + 0 as i32
                            }) < 0 as i32
                            {
                                (((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                    + 0 as i32)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + 0 as i32
                                        }) + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + 0 as i32
                                        }) - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32)
                                    < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32) as i32
                            }) != 0 && 12 as i32 == -(1 as i32)
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((0 as i32 as i64) < len + 0 as i32 as i64) as i32
                                } else {
                                    ((0 as i32 as i64) < len
                                        && ((-(1 as i32) - 0 as i32) as i64)
                                            < len - 1 as i32 as i64) as i32
                                })
                            } else {
                                (((0 as i32 / 12 as i32) as i64) < len) as i32
                            })
                        })
                    } else {
                        (if 12 as i32 == 0 as i32 {
                            0 as i32
                        } else {
                            (if len < 0 as i32 as i64 {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + 0 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        + 0 as i32 as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + 0 as i32 as i64
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64) as i32
                                }) != 0 && len == -(1 as i32) as i64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((0 as i32) < 12 as i32 + 0 as i32) as i32
                                    } else {
                                        ((-(1 as i32) - 0 as i32) < 12 as i32 - 1 as i32) as i32
                                    })
                                } else {
                                    (0 as i32 as i64 / len < 12 as i32 as i64) as i32
                                })
                            } else {
                                (((2147483647 as i32 as u32)
                                    .wrapping_mul(2 as u32)
                                    .wrapping_add(1 as u32)
                                    .wrapping_div(12 as i32 as u32) as i64) < len) as i32
                            })
                        })
                    }) != 0
                    {
                        len12 = (len as u32).wrapping_mul(12 as i32 as u32) as idx_t;
                        1 as i32
                    } else {
                        len12 = (len as u32).wrapping_mul(12 as i32 as u32) as idx_t;
                        0 as i32
                    })
                })
            } else {
                (if ::core::mem::size_of::<idx_t>() as u64
                    == ::core::mem::size_of::<i64>() as u64
                {
                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len12 })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (if (if (12 as i32) < 0 as i32 {
                            (if len < 0 as i32 as i64 {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        9223372036854775807 as i64
                                    }) + 12 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    (len < 9223372036854775807 as i64 / 12 as i32 as i64) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        12 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((12 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 12 as i32) as i32
                                    }) != 0
                                    {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                            + 9223372036854775807 as i64
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        9223372036854775807 as i64 / -(12 as i32) as i64
                                    }) <= -(1 as i32) as i64 - len) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as i64
                                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                }) != 0 && 12 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64)
                                            < len + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                    } else {
                                        ((0 as i32 as i64) < len
                                            && -(1 as i32) as i64
                                                - (-(9223372036854775807 as i64) - 1 as i64)
                                                < len - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    (((-(9223372036854775807 as i64) - 1 as i64)
                                        / 12 as i32 as i64) < len) as i32
                                })
                            })
                        } else {
                            (if 12 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if len < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                    }) != 0 && len == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32 as i64)
                                                < 12 as i32 as i64
                                                    + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                        } else {
                                            (-(1 as i32) as i64
                                                - (-(9223372036854775807 as i64) - 1 as i64)
                                                < (12 as i32 - 1 as i32) as i64) as i32
                                        })
                                    } else {
                                        ((-(9223372036854775807 as i64) - 1 as i64) / len
                                            < 12 as i32 as i64) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as i64 / 12 as i32 as i64) < len)
                                        as i32
                                })
                            })
                        }) != 0
                        {
                            len12 = (len as u64).wrapping_mul(12 as i32 as u64) as i64;
                            1 as i32
                        } else {
                            len12 = (len as u64).wrapping_mul(12 as i32 as u64) as i64;
                            0 as i32
                        })
                    } else {
                        (if (if (12 as i32) < 0 as i32 {
                            (if len < 0 as i32 as i64 {
                                (if (if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as u64
                                    } else {
                                        (9223372036854775807 as i64 as u64)
                                            .wrapping_mul(2 as u64)
                                            .wrapping_add(1 as u64)
                                    })
                                        .wrapping_add(12 as i32 as u64)
                                })
                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                {
                                    ((len as u64)
                                        < (9223372036854775807 as i64 as u64)
                                            .wrapping_mul(2 as u64)
                                            .wrapping_add(1 as u64)
                                            .wrapping_div(12 as i32 as u64)) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        12 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((12 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 12 as i32) as i32
                                    }) != 0
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 }) as u64)
                                            .wrapping_add(
                                                (9223372036854775807 as i64 as u64)
                                                    .wrapping_mul(2 as u64)
                                                    .wrapping_add(1 as u64),
                                            )
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        (9223372036854775807 as i64 as u64)
                                            .wrapping_mul(2 as u64)
                                            .wrapping_add(1 as u64)
                                            .wrapping_div(-(12 as i32) as u64)
                                    }) <= (-(1 as i32) as i64 - len) as u64) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) + 0 as i32
                                }) < 0 as i32
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + 0 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 0 as i32
                                            }) + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 0 as i32
                                            }) - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32) as i32
                                }) != 0 && 12 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64) < len + 0 as i32 as i64) as i32
                                    } else {
                                        ((0 as i32 as i64) < len
                                            && ((-(1 as i32) - 0 as i32) as i64)
                                                < len - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    (((0 as i32 / 12 as i32) as i64) < len) as i32
                                })
                            })
                        } else {
                            (if 12 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if len < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64) as i32
                                    }) != 0 && len == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32) < 12 as i32 + 0 as i32) as i32
                                        } else {
                                            ((-(1 as i32) - 0 as i32) < 12 as i32 - 1 as i32) as i32
                                        })
                                    } else {
                                        (0 as i32 as i64 / len < 12 as i32 as i64) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as i64 as u64)
                                        .wrapping_mul(2 as u64)
                                        .wrapping_add(1 as u64)
                                        .wrapping_div(12 as i32 as u64) < len as u64) as i32
                                })
                            })
                        }) != 0
                        {
                            len12 = (len as u64).wrapping_mul(12 as i32 as u64) as idx_t;
                            1 as i32
                        } else {
                            len12 = (len as u64).wrapping_mul(12 as i32 as u64) as idx_t;
                            0 as i32
                        })
                    })
                } else {
                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len12 })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (if (if (12 as i32) < 0 as i32 {
                            (if len < 0 as i32 as i64 {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as libc::c_longlong
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                    }) + 12 as i32 as libc::c_longlong
                                }) - 1 as i32 as libc::c_longlong)
                                    < 0 as i32 as libc::c_longlong
                                {
                                    ((len as libc::c_longlong)
                                        < 9223372036854775807 as libc::c_longlong
                                            / 12 as i32 as libc::c_longlong) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        12 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((12 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 12 as i32) as i32
                                    }) != 0
                                    {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            as libc::c_longlong
                                            + 9223372036854775807 as libc::c_longlong
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                            / -(12 as i32) as libc::c_longlong
                                    }) <= (-(1 as i32) as i64 - len) as libc::c_longlong) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as libc::c_longlong
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                }) - 1 as i32 as libc::c_longlong)
                                    < 0 as i32 as libc::c_longlong
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 1 as i32 as libc::c_longlong)
                                        << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64))
                                        - 1 as i32 as libc::c_longlong)
                                        * 2 as i32 as libc::c_longlong
                                        + 1 as i32 as libc::c_longlong)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 0 as i32 as libc::c_longlong
                                }) < 0 as i32 as libc::c_longlong
                                {
                                    ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) - 1 as i32 as libc::c_longlong)
                                            < 0 as i32 as libc::c_longlong
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) + 1 as i32 as libc::c_longlong)
                                                << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64))
                                                - 1 as i32 as libc::c_longlong)
                                                * 2 as i32 as libc::c_longlong
                                                + 1 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as i32 as libc::c_longlong
                                        })) as i32
                                } else {
                                    ((0 as i32 as libc::c_longlong)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)) as i32
                                }) != 0 && 12 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as libc::c_longlong)
                                            < len as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as i32
                                    } else {
                                        ((0 as i32 as i64) < len
                                            && -(1 as i32) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (len - 1 as i32 as i64) as libc::c_longlong) as i32
                                    })
                                } else {
                                    (((-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) / 12 as i32 as libc::c_longlong)
                                        < len as libc::c_longlong) as i32
                                })
                            })
                        } else {
                            (if 12 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if len < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) - 1 as i32 as libc::c_longlong)
                                        < 0 as i32 as libc::c_longlong
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 1 as i32 as libc::c_longlong)
                                            << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64))
                                            - 1 as i32 as libc::c_longlong)
                                            * 2 as i32 as libc::c_longlong
                                            + 1 as i32 as libc::c_longlong)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 0 as i32 as libc::c_longlong
                                    }) < 0 as i32 as libc::c_longlong
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as i32 as libc::c_longlong)
                                                < 0 as i32 as libc::c_longlong
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 1 as i32 as libc::c_longlong)
                                                    << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64))
                                                    - 1 as i32 as libc::c_longlong)
                                                    * 2 as i32 as libc::c_longlong
                                                    + 1 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as i32 as libc::c_longlong
                                            })) as i32
                                    } else {
                                        ((0 as i32 as libc::c_longlong)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as i32
                                    }) != 0 && len == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32 as libc::c_longlong)
                                                < 12 as i32 as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)) as i32
                                        } else {
                                            (-(1 as i32) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (12 as i32 - 1 as i32) as libc::c_longlong) as i32
                                        })
                                    } else {
                                        (((-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) / len as libc::c_longlong)
                                            < 12 as i32 as libc::c_longlong) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        / 12 as i32 as libc::c_longlong) < len as libc::c_longlong)
                                        as i32
                                })
                            })
                        }) != 0
                        {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as i32 as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            1 as i32
                        } else {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as i32 as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            0 as i32
                        })
                    } else {
                        (if (if (12 as i32) < 0 as i32 {
                            (if len < 0 as i32 as i64 {
                                (if (if 1 as i32 != 0 {
                                    0 as i32 as libc::c_ulonglong
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as libc::c_ulonglong
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                    })
                                        .wrapping_add(12 as i32 as libc::c_ulonglong)
                                })
                                    .wrapping_sub(1 as i32 as libc::c_ulonglong)
                                    < 0 as i32 as libc::c_ulonglong
                                {
                                    ((len as libc::c_ulonglong)
                                        < (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(12 as i32 as libc::c_ulonglong)) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        12 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((12 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 12 as i32) as i32
                                    }) != 0
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            as libc::c_ulonglong)
                                            .wrapping_add(
                                                (9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong),
                                            )
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(-(12 as i32) as libc::c_ulonglong)
                                    }) <= (-(1 as i32) as i64 - len) as libc::c_ulonglong)
                                        as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32
                                    }) + 0 as i32
                                }) < 0 as i32
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                        + 0 as i32)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                + 0 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 0 as i32
                                            }) + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                                    + 0 as i32
                                            }) - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            + 0 as i32) as i32
                                }) != 0 && 12 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                        - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64) < len + 0 as i32 as i64) as i32
                                    } else {
                                        ((0 as i32 as i64) < len
                                            && ((-(1 as i32) - 0 as i32) as i64)
                                                < len - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    (((0 as i32 / 12 as i32) as i64) < len) as i32
                                })
                            })
                        } else {
                            (if 12 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if len < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                            + 0 as i32 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { len })
                                                + 0 as i32 as i64) as i32
                                    }) != 0 && len == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 12 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32) < 12 as i32 + 0 as i32) as i32
                                        } else {
                                            ((-(1 as i32) - 0 as i32) < 12 as i32 - 1 as i32) as i32
                                        })
                                    } else {
                                        (0 as i32 as i64 / len < 12 as i32 as i64) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        as libc::c_ulonglong)
                                        .wrapping_mul(2 as libc::c_ulonglong)
                                        .wrapping_add(1 as libc::c_ulonglong)
                                        .wrapping_div(12 as i32 as libc::c_ulonglong)
                                        < len as libc::c_ulonglong) as i32
                                })
                            })
                        }) != 0
                        {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as i32 as libc::c_ulonglong) as idx_t;
                            1 as i32
                        } else {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as i32 as libc::c_ulonglong) as idx_t;
                            0 as i32
                        })
                    })
                })
            })
        })
    }) != 0) && len12 < size
    {
        ep = text.offset(size as isize).offset(-((11 as i32 as i64 * len) as isize));
        while tp <= ep {
            let mut tp0: *const i8 = tp;
            d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
            tp = tp.offset(d as isize);
            d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
            tp = tp.offset(d as isize);
            if d != 0 as i32 {
                d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
                tp = tp.offset(d as isize);
                d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
                tp = tp.offset(d as isize);
                d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
                tp = tp.offset(d as isize);
                if d != 0 as i32 {
                    d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
                    tp = tp.offset(d as isize);
                    d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
                    tp = tp.offset(d as isize);
                    d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
                    tp = tp.offset(d as isize);
                    if d != 0 as i32 {
                        d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize)
                            as i32;
                        tp = tp.offset(d as isize);
                        d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize)
                            as i32;
                        tp = tp.offset(d as isize);
                        let mut advance_heuristic: i32 = (16 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<i64>() as u64) as i32;
                        if advance_heuristic as i64 <= tp.offset_from(tp0) as i64 {
                            continue;
                        }
                        tp = tp.offset(-1);
                        tp;
                        tp = memchr_kwset(
                            tp,
                            text.offset(size as isize).offset_from(tp) as i64,
                            kwset,
                        );
                        if tp.is_null() {
                            return -(1 as i32) as ptrdiff_t;
                        }
                        tp = tp.offset(1);
                        tp;
                        if ep <= tp {
                            break;
                        }
                    }
                }
            }
            if bm_delta2_search(&mut tp, ep, sp, len, trans, gc1, gc2, d1, kwset) {
                return tp.offset_from(text) as i64;
            }
        }
    }
    ep = text.offset(size as isize);
    d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
    while d as i64 <= ep.offset_from(tp) as i64 {
        tp = tp.offset(d as isize);
        d = *d1.offset(U(*tp.offset(-(1 as i32) as isize)) as isize) as i32;
        if d != 0 as i32 {
            continue;
        }
        if bm_delta2_search(
            &mut tp,
            ep,
            sp,
            len,
            trans,
            gc1,
            gc2,
            0 as *const u8,
            kwset,
        ) {
            return tp.offset_from(text) as i64;
        }
    }
    return -(1 as i32) as ptrdiff_t;
}
unsafe extern "C" fn bmexec(
    mut kwset: kwset_t,
    mut text: *const i8,
    mut size: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    let mut ret: ptrdiff_t = if !((*kwset).trans).is_null() {
        bmexec_trans(kwset, text, size)
    } else {
        bmexec_trans(kwset, text, size)
    };
    (*kwsmatch).index = 0 as i32 as idx_t;
    (*kwsmatch).offset = ret;
    (*kwsmatch).size = (*kwset).mind;
    return ret;
}
#[inline]
unsafe extern "C" fn acexec_trans(
    mut kwset: kwset_t,
    mut text: *const i8,
    mut len: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    let mut trie: *const trie = 0 as *const trie;
    let mut accept: *const trie = 0 as *const trie;
    let mut tp: *const i8 = 0 as *const i8;
    let mut left: *const i8 = 0 as *const i8;
    let mut lim: *const i8 = 0 as *const i8;
    let mut tree: *const tree = 0 as *const tree;
    let mut trans: *const i8 = 0 as *const i8;
    if len < (*kwset).mind {
        return -(1 as i32) as ptrdiff_t;
    }
    trans = (*kwset).trans;
    trie = (*kwset).trie;
    lim = text.offset(len as isize);
    tp = text;
    if (*trie).accepting == 0 {
        let mut c: u8 = 0;
        let mut gc1: i32 = (*kwset).gc1;
        's_38: loop {
            if gc1 < 0 as i32 {
                loop {
                    let fresh5 = tp;
                    tp = tp.offset(1);
                    c = tr(trans, *fresh5) as u8;
                    trie = (*kwset).next[c as usize];
                    if !trie.is_null() {
                        break;
                    }
                    if tp >= lim {
                        return -(1 as i32) as ptrdiff_t;
                    }
                }
            } else {
                tp = memchr_kwset(tp, lim.offset_from(tp) as i64, kwset);
                if tp.is_null() {
                    return -(1 as i32) as ptrdiff_t;
                }
                let fresh6 = tp;
                tp = tp.offset(1);
                c = tr(trans, *fresh6) as u8;
                trie = (*kwset).next[c as usize];
            }
            's_85: loop {
                if (*trie).accepting != 0 {
                    break 's_38;
                }
                if tp >= lim {
                    return -(1 as i32) as ptrdiff_t;
                }
                let fresh7 = tp;
                tp = tp.offset(1);
                c = tr(trans, *fresh7) as u8;
                tree = (*trie).links;
                while c as i32 != (*tree).label as i32 {
                    tree = if (c as i32) < (*tree).label as i32 {
                        (*tree).llink
                    } else {
                        (*tree).rlink
                    };
                    if !tree.is_null() {
                        continue;
                    }
                    trie = (*trie).fail;
                    if trie.is_null() {
                        trie = (*kwset).next[c as usize];
                        if !trie.is_null() {
                            continue 's_85;
                        }
                        if tp >= lim {
                            return -(1 as i32) as ptrdiff_t;
                        }
                        break 's_85;
                    } else if (*trie).accepting != 0 {
                        tp = tp.offset(-1);
                        tp;
                        break 's_38;
                    } else {
                        tree = (*trie).links;
                    }
                }
                trie = (*tree).trie;
            }
        }
    }
    accept = trie;
    while (*accept).accepting < 0 as i32 as i64 {
        accept = (*accept).fail;
    }
    left = tp.offset(-((*accept).depth as isize));
    if longest {
        while tp < lim {
            let mut accept1: *const trie = 0 as *const trie;
            let mut left1: *const i8 = 0 as *const i8;
            let fresh8 = tp;
            tp = tp.offset(1);
            let mut c_0: u8 = tr(trans, *fresh8) as u8;
            loop {
                tree = (*trie).links;
                while !tree.is_null() && c_0 as i32 != (*tree).label as i32 {
                    tree = if (c_0 as i32) < (*tree).label as i32 {
                        (*tree).llink
                    } else {
                        (*tree).rlink
                    };
                }
                if !(tree.is_null()
                    && {
                        trie = (*trie).fail;
                        !trie.is_null()
                    } && (*accept).depth <= (*trie).depth)
                {
                    break;
                }
            }
            if tree.is_null() {
                break;
            }
            trie = (*tree).trie;
            if (*trie).accepting != 0 {
                accept1 = trie;
                while (*accept1).accepting < 0 as i32 as i64 {
                    accept1 = (*accept1).fail;
                }
                left1 = tp.offset(-((*accept1).depth as isize));
                if left1 <= left {
                    left = left1;
                    accept = accept1;
                }
            }
        }
    }
    (*kwsmatch).index = (*accept).accepting >> 1 as i32;
    (*kwsmatch).offset = left.offset_from(text) as i64;
    (*kwsmatch).size = (*accept).depth;
    return left.offset_from(text) as i64;
}
unsafe extern "C" fn acexec(
    mut kwset: kwset_t,
    mut text: *const i8,
    mut size: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    if 0 as i32 as i64 <= size {} else {
        unreachable!();
    };
    return if !((*kwset).trans).is_null() {
        acexec_trans(kwset, text, size, kwsmatch, longest)
    } else {
        acexec_trans(kwset, text, size, kwsmatch, longest)
    };
}
#[no_mangle]
pub unsafe extern "C" fn kwsexec(
    mut kwset: kwset_t,
    mut text: *const i8,
    mut size: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    return ((*kwset).kwsexec)
        .expect("non-null function pointer")(kwset, text, size, kwsmatch, longest);
}
#[no_mangle]
pub unsafe extern "C" fn kwsfree(mut kwset: kwset_t) {
    let mut __o: *mut obstack = &mut (*kwset).obstack;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut i8;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    rpl_free(kwset as *mut libc::c_void);
}