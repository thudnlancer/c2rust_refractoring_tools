use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
pub type gsl_bst_cmp_function = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(size_t, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_node {
    pub rb_link: [*mut gsl_bst_rb_node; 2],
    pub rb_data: *mut libc::c_void,
    pub rb_color: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_table {
    pub rb_root: *mut gsl_bst_rb_node,
    pub rb_compare: Option<gsl_bst_cmp_function>,
    pub rb_param: *mut libc::c_void,
    pub rb_alloc: *const gsl_bst_allocator,
    pub rb_count: size_t,
    pub rb_generation: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_traverser {
    pub rb_table: *const gsl_bst_rb_table,
    pub rb_node: *mut gsl_bst_rb_node,
    pub rb_stack: [*mut gsl_bst_rb_node; 48],
    pub rb_height: size_t,
    pub rb_generation: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_type {
    pub name: *const i8,
    pub node_size: size_t,
    pub init: Option<
        unsafe extern "C" fn(
            *const gsl_bst_allocator,
            Option<gsl_bst_cmp_function>,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub nodes: Option<unsafe extern "C" fn(*const libc::c_void) -> size_t>,
    pub insert: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub find: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub remove: Option<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub empty: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub trav_init: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> i32,
    >,
    pub trav_first: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_last: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_find: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_insert: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_copy: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_next: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub trav_prev: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub trav_cur: Option<unsafe extern "C" fn(*const libc::c_void) -> *mut libc::c_void>,
    pub trav_replace: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
}
pub type rb_traverser = gsl_bst_rb_traverser;
pub type rb_node = gsl_bst_rb_node;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub type rb_table = gsl_bst_rb_table;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum rb_color {
    RB_BLACK,
    RB_RED,
}
impl rb_color {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            rb_color::RB_BLACK => 0,
            rb_color::RB_RED => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> rb_color {
        match value {
            0 => rb_color::RB_BLACK,
            1 => rb_color::RB_RED,
            _ => panic!("Invalid value for rb_color: {}", value),
        }
    }
}
impl AddAssign<u32> for rb_color {
    fn add_assign(&mut self, rhs: u32) {
        *self = rb_color::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for rb_color {
    fn sub_assign(&mut self, rhs: u32) {
        *self = rb_color::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for rb_color {
    fn mul_assign(&mut self, rhs: u32) {
        *self = rb_color::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for rb_color {
    fn div_assign(&mut self, rhs: u32) {
        *self = rb_color::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for rb_color {
    fn rem_assign(&mut self, rhs: u32) {
        *self = rb_color::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for rb_color {
    type Output = rb_color;
    fn add(self, rhs: u32) -> rb_color {
        rb_color::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for rb_color {
    type Output = rb_color;
    fn sub(self, rhs: u32) -> rb_color {
        rb_color::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for rb_color {
    type Output = rb_color;
    fn mul(self, rhs: u32) -> rb_color {
        rb_color::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for rb_color {
    type Output = rb_color;
    fn div(self, rhs: u32) -> rb_color {
        rb_color::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for rb_color {
    type Output = rb_color;
    fn rem(self, rhs: u32) -> rb_color {
        rb_color::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
unsafe extern "C" fn rb_init(
    mut allocator: *const gsl_bst_allocator,
    mut compare: Option<gsl_bst_cmp_function>,
    mut params: *mut libc::c_void,
    mut vtable: *mut libc::c_void,
) -> i32 {
    let mut table: *mut rb_table = vtable as *mut rb_table;
    (*table).rb_alloc = allocator;
    (*table).rb_compare = compare;
    (*table).rb_param = params;
    (*table).rb_root = 0 as *mut gsl_bst_rb_node;
    (*table).rb_count = 0 as i32 as size_t;
    (*table).rb_generation = 0 as i32 as u64;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn rb_nodes(mut vtable: *const libc::c_void) -> size_t {
    let mut table: *const rb_table = vtable as *const rb_table;
    return (*table).rb_count;
}
unsafe extern "C" fn rb_empty(mut vtable: *mut libc::c_void) -> i32 {
    let mut table: *mut rb_table = vtable as *mut rb_table;
    let mut p: *mut rb_node = 0 as *mut rb_node;
    let mut q: *mut rb_node = 0 as *mut rb_node;
    p = (*table).rb_root;
    while !p.is_null() {
        if ((*p).rb_link[0 as i32 as usize]).is_null() {
            q = (*p).rb_link[1 as i32 as usize];
            ((*(*table).rb_alloc).free)
                .expect(
                    "non-null function pointer",
                )(p as *mut libc::c_void, (*table).rb_param);
        } else {
            q = (*p).rb_link[0 as i32 as usize];
            (*p).rb_link[0 as i32 as usize] = (*q).rb_link[1 as i32 as usize];
            (*q).rb_link[1 as i32 as usize] = p;
        }
        p = q;
    }
    (*table).rb_root = 0 as *mut gsl_bst_rb_node;
    (*table).rb_count = 0 as i32 as size_t;
    (*table).rb_generation = 0 as i32 as u64;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn rb_probe(
    mut item: *mut libc::c_void,
    mut table: *mut rb_table,
) -> *mut *mut libc::c_void {
    let mut pa: [*mut rb_node; 48] = [0 as *mut rb_node; 48];
    let mut da: [u8; 48] = [0; 48];
    let mut k: i32 = 0;
    let mut p: *mut rb_node = 0 as *mut rb_node;
    let mut n: *mut rb_node = 0 as *mut rb_node;
    pa[0 as i32 as usize] = &mut (*table).rb_root as *mut *mut gsl_bst_rb_node
        as *mut rb_node;
    da[0 as i32 as usize] = 0 as i32 as u8;
    k = 1 as i32;
    p = (*table).rb_root;
    while !p.is_null() {
        let mut cmp: i32 = ((*table).rb_compare)
            .expect("non-null function pointer")(item, (*p).rb_data, (*table).rb_param);
        if cmp == 0 as i32 {
            return &mut (*p).rb_data;
        }
        pa[k as usize] = p;
        let fresh0 = k;
        k = k + 1;
        da[fresh0 as usize] = (cmp > 0 as i32) as i32 as u8;
        p = (*p).rb_link[da[(k - 1 as i32) as usize] as usize];
    }
    (*pa[(k - 1 as i32) as usize]).rb_link[da[(k - 1 as i32) as usize] as usize] = ((*(*table)
        .rb_alloc)
        .alloc)
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<rb_node>() as u64, (*table).rb_param)
        as *mut gsl_bst_rb_node;
    n = (*pa[(k - 1 as i32) as usize]).rb_link[da[(k - 1 as i32) as usize] as usize];
    if n.is_null() {
        return 0 as *mut *mut libc::c_void;
    }
    (*n).rb_data = item;
    (*n).rb_link[1 as i32 as usize] = 0 as *mut gsl_bst_rb_node;
    (*n).rb_link[0 as i32 as usize] = (*n).rb_link[1 as i32 as usize];
    (*n).rb_color = rb_color::RB_RED as i32 as u8;
    (*table).rb_count = ((*table).rb_count).wrapping_add(1);
    (*table).rb_count;
    (*table).rb_generation = ((*table).rb_generation).wrapping_add(1);
    (*table).rb_generation;
    while k >= 3 as i32
        && (*pa[(k - 1 as i32) as usize]).rb_color as i32 == rb_color::RB_RED as i32
    {
        if da[(k - 2 as i32) as usize] as i32 == 0 as i32 {
            let mut y: *mut rb_node = (*pa[(k - 2 as i32) as usize])
                .rb_link[1 as i32 as usize];
            if !y.is_null() && (*y).rb_color as i32 == rb_color::RB_RED as i32 {
                (*y).rb_color = rb_color::RB_BLACK as i32 as u8;
                (*pa[(k - 1 as i32) as usize]).rb_color = (*y).rb_color;
                (*pa[(k - 2 as i32) as usize]).rb_color = rb_color::RB_RED as i32 as u8;
                k -= 2 as i32;
            } else {
                let mut x: *mut rb_node = 0 as *mut rb_node;
                if da[(k - 1 as i32) as usize] as i32 == 0 as i32 {
                    y = pa[(k - 1 as i32) as usize];
                } else {
                    x = pa[(k - 1 as i32) as usize];
                    y = (*x).rb_link[1 as i32 as usize];
                    (*x).rb_link[1 as i32 as usize] = (*y).rb_link[0 as i32 as usize];
                    (*y).rb_link[0 as i32 as usize] = x;
                    (*pa[(k - 2 as i32) as usize]).rb_link[0 as i32 as usize] = y;
                }
                x = pa[(k - 2 as i32) as usize];
                (*x).rb_color = rb_color::RB_RED as i32 as u8;
                (*y).rb_color = rb_color::RB_BLACK as i32 as u8;
                (*x).rb_link[0 as i32 as usize] = (*y).rb_link[1 as i32 as usize];
                (*y).rb_link[1 as i32 as usize] = x;
                (*pa[(k - 3 as i32) as usize])
                    .rb_link[da[(k - 3 as i32) as usize] as usize] = y;
                break;
            }
        } else {
            let mut y_0: *mut rb_node = (*pa[(k - 2 as i32) as usize])
                .rb_link[0 as i32 as usize];
            if !y_0.is_null() && (*y_0).rb_color as i32 == rb_color::RB_RED as i32 {
                (*y_0).rb_color = rb_color::RB_BLACK as i32 as u8;
                (*pa[(k - 1 as i32) as usize]).rb_color = (*y_0).rb_color;
                (*pa[(k - 2 as i32) as usize]).rb_color = rb_color::RB_RED as i32 as u8;
                k -= 2 as i32;
            } else {
                let mut x_0: *mut rb_node = 0 as *mut rb_node;
                if da[(k - 1 as i32) as usize] as i32 == 1 as i32 {
                    y_0 = pa[(k - 1 as i32) as usize];
                } else {
                    x_0 = pa[(k - 1 as i32) as usize];
                    y_0 = (*x_0).rb_link[0 as i32 as usize];
                    (*x_0).rb_link[0 as i32 as usize] = (*y_0)
                        .rb_link[1 as i32 as usize];
                    (*y_0).rb_link[1 as i32 as usize] = x_0;
                    (*pa[(k - 2 as i32) as usize]).rb_link[1 as i32 as usize] = y_0;
                }
                x_0 = pa[(k - 2 as i32) as usize];
                (*x_0).rb_color = rb_color::RB_RED as i32 as u8;
                (*y_0).rb_color = rb_color::RB_BLACK as i32 as u8;
                (*x_0).rb_link[1 as i32 as usize] = (*y_0).rb_link[0 as i32 as usize];
                (*y_0).rb_link[0 as i32 as usize] = x_0;
                (*pa[(k - 3 as i32) as usize])
                    .rb_link[da[(k - 3 as i32) as usize] as usize] = y_0;
                break;
            }
        }
    }
    (*(*table).rb_root).rb_color = rb_color::RB_BLACK as i32 as u8;
    return &mut (*n).rb_data;
}
unsafe extern "C" fn rb_insert(
    mut item: *mut libc::c_void,
    mut vtable: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut p: *mut *mut libc::c_void = rb_probe(item, vtable as *mut rb_table);
    return if p.is_null() || *p == item { 0 as *mut libc::c_void } else { *p };
}
unsafe extern "C" fn rb_find(
    mut item: *const libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const rb_table = vtable as *const rb_table;
    let mut p: *const rb_node = 0 as *const rb_node;
    p = (*table).rb_root;
    while !p.is_null() {
        let mut cmp: i32 = ((*table).rb_compare)
            .expect("non-null function pointer")(item, (*p).rb_data, (*table).rb_param);
        if cmp < 0 as i32 {
            p = (*p).rb_link[0 as i32 as usize];
        } else if cmp > 0 as i32 {
            p = (*p).rb_link[1 as i32 as usize];
        } else {
            return (*p).rb_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn rb_remove(
    mut item: *const libc::c_void,
    mut vtable: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut table: *mut rb_table = vtable as *mut rb_table;
    let mut pa: [*mut rb_node; 48] = [0 as *mut rb_node; 48];
    let mut da: [u8; 48] = [0; 48];
    let mut k: i32 = 0;
    let mut p: *mut rb_node = 0 as *mut rb_node;
    let mut cmp: i32 = 0;
    k = 0 as i32;
    p = &mut (*table).rb_root as *mut *mut gsl_bst_rb_node as *mut rb_node;
    cmp = -(1 as i32);
    while cmp != 0 as i32 {
        let mut dir: i32 = (cmp > 0 as i32) as i32;
        pa[k as usize] = p;
        let fresh1 = k;
        k = k + 1;
        da[fresh1 as usize] = dir as u8;
        p = (*p).rb_link[dir as usize];
        if p.is_null() {
            return 0 as *mut libc::c_void;
        }
        cmp = ((*table).rb_compare)
            .expect("non-null function pointer")(item, (*p).rb_data, (*table).rb_param);
    }
    item = (*p).rb_data;
    if ((*p).rb_link[1 as i32 as usize]).is_null() {
        (*pa[(k - 1 as i32) as usize]).rb_link[da[(k - 1 as i32) as usize] as usize] = (*p)
            .rb_link[0 as i32 as usize];
    } else {
        let mut t: rb_color = rb_color::RB_BLACK;
        let mut r: *mut rb_node = (*p).rb_link[1 as i32 as usize];
        if ((*r).rb_link[0 as i32 as usize]).is_null() {
            (*r).rb_link[0 as i32 as usize] = (*p).rb_link[0 as i32 as usize];
            t = (*r).rb_color as rb_color;
            (*r).rb_color = (*p).rb_color;
            (*p).rb_color = t as u8;
            (*pa[(k - 1 as i32) as usize])
                .rb_link[da[(k - 1 as i32) as usize] as usize] = r;
            da[k as usize] = 1 as i32 as u8;
            let fresh2 = k;
            k = k + 1;
            pa[fresh2 as usize] = r;
        } else {
            let mut s: *mut rb_node = 0 as *mut rb_node;
            let fresh3 = k;
            k = k + 1;
            let mut j: i32 = fresh3;
            loop {
                da[k as usize] = 0 as i32 as u8;
                let fresh4 = k;
                k = k + 1;
                pa[fresh4 as usize] = r;
                s = (*r).rb_link[0 as i32 as usize];
                if ((*s).rb_link[0 as i32 as usize]).is_null() {
                    break;
                }
                r = s;
            }
            da[j as usize] = 1 as i32 as u8;
            pa[j as usize] = s;
            (*pa[(j - 1 as i32) as usize])
                .rb_link[da[(j - 1 as i32) as usize] as usize] = s;
            (*s).rb_link[0 as i32 as usize] = (*p).rb_link[0 as i32 as usize];
            (*r).rb_link[0 as i32 as usize] = (*s).rb_link[1 as i32 as usize];
            (*s).rb_link[1 as i32 as usize] = (*p).rb_link[1 as i32 as usize];
            t = (*s).rb_color as rb_color;
            (*s).rb_color = (*p).rb_color;
            (*p).rb_color = t as u8;
        }
    }
    if (*p).rb_color as i32 == rb_color::RB_BLACK as i32 {
        loop {
            let mut x: *mut rb_node = (*pa[(k - 1 as i32) as usize])
                .rb_link[da[(k - 1 as i32) as usize] as usize];
            if !x.is_null() && (*x).rb_color as i32 == rb_color::RB_RED as i32 {
                (*x).rb_color = rb_color::RB_BLACK as i32 as u8;
                break;
            } else {
                if k < 2 as i32 {
                    break;
                }
                if da[(k - 1 as i32) as usize] as i32 == 0 as i32 {
                    let mut w: *mut rb_node = (*pa[(k - 1 as i32) as usize])
                        .rb_link[1 as i32 as usize];
                    if (*w).rb_color as i32 == rb_color::RB_RED as i32 {
                        (*w).rb_color = rb_color::RB_BLACK as i32 as u8;
                        (*pa[(k - 1 as i32) as usize]).rb_color = rb_color::RB_RED as i32
                            as u8;
                        (*pa[(k - 1 as i32) as usize]).rb_link[1 as i32 as usize] = (*w)
                            .rb_link[0 as i32 as usize];
                        (*w).rb_link[0 as i32 as usize] = pa[(k - 1 as i32) as usize];
                        (*pa[(k - 2 as i32) as usize])
                            .rb_link[da[(k - 2 as i32) as usize] as usize] = w;
                        pa[k as usize] = pa[(k - 1 as i32) as usize];
                        da[k as usize] = 0 as i32 as u8;
                        pa[(k - 1 as i32) as usize] = w;
                        k += 1;
                        k;
                        w = (*pa[(k - 1 as i32) as usize]).rb_link[1 as i32 as usize];
                    }
                    if (((*w).rb_link[0 as i32 as usize]).is_null()
                        || (*(*w).rb_link[0 as i32 as usize]).rb_color as i32
                            == rb_color::RB_BLACK as i32)
                        && (((*w).rb_link[1 as i32 as usize]).is_null()
                            || (*(*w).rb_link[1 as i32 as usize]).rb_color as i32
                                == rb_color::RB_BLACK as i32)
                    {
                        (*w).rb_color = rb_color::RB_RED as i32 as u8;
                    } else {
                        if ((*w).rb_link[1 as i32 as usize]).is_null()
                            || (*(*w).rb_link[1 as i32 as usize]).rb_color as i32
                                == rb_color::RB_BLACK as i32
                        {
                            let mut y: *mut rb_node = (*w).rb_link[0 as i32 as usize];
                            (*y).rb_color = rb_color::RB_BLACK as i32 as u8;
                            (*w).rb_color = rb_color::RB_RED as i32 as u8;
                            (*w).rb_link[0 as i32 as usize] = (*y)
                                .rb_link[1 as i32 as usize];
                            (*y).rb_link[1 as i32 as usize] = w;
                            (*pa[(k - 1 as i32) as usize]).rb_link[1 as i32 as usize] = y;
                            w = (*pa[(k - 1 as i32) as usize])
                                .rb_link[1 as i32 as usize];
                        }
                        (*w).rb_color = (*pa[(k - 1 as i32) as usize]).rb_color;
                        (*pa[(k - 1 as i32) as usize]).rb_color = rb_color::RB_BLACK
                            as i32 as u8;
                        (*(*w).rb_link[1 as i32 as usize]).rb_color = rb_color::RB_BLACK
                            as i32 as u8;
                        (*pa[(k - 1 as i32) as usize]).rb_link[1 as i32 as usize] = (*w)
                            .rb_link[0 as i32 as usize];
                        (*w).rb_link[0 as i32 as usize] = pa[(k - 1 as i32) as usize];
                        (*pa[(k - 2 as i32) as usize])
                            .rb_link[da[(k - 2 as i32) as usize] as usize] = w;
                        break;
                    }
                } else {
                    let mut w_0: *mut rb_node = (*pa[(k - 1 as i32) as usize])
                        .rb_link[0 as i32 as usize];
                    if (*w_0).rb_color as i32 == rb_color::RB_RED as i32 {
                        (*w_0).rb_color = rb_color::RB_BLACK as i32 as u8;
                        (*pa[(k - 1 as i32) as usize]).rb_color = rb_color::RB_RED as i32
                            as u8;
                        (*pa[(k - 1 as i32) as usize]).rb_link[0 as i32 as usize] = (*w_0)
                            .rb_link[1 as i32 as usize];
                        (*w_0).rb_link[1 as i32 as usize] = pa[(k - 1 as i32) as usize];
                        (*pa[(k - 2 as i32) as usize])
                            .rb_link[da[(k - 2 as i32) as usize] as usize] = w_0;
                        pa[k as usize] = pa[(k - 1 as i32) as usize];
                        da[k as usize] = 1 as i32 as u8;
                        pa[(k - 1 as i32) as usize] = w_0;
                        k += 1;
                        k;
                        w_0 = (*pa[(k - 1 as i32) as usize]).rb_link[0 as i32 as usize];
                    }
                    if (((*w_0).rb_link[0 as i32 as usize]).is_null()
                        || (*(*w_0).rb_link[0 as i32 as usize]).rb_color as i32
                            == rb_color::RB_BLACK as i32)
                        && (((*w_0).rb_link[1 as i32 as usize]).is_null()
                            || (*(*w_0).rb_link[1 as i32 as usize]).rb_color as i32
                                == rb_color::RB_BLACK as i32)
                    {
                        (*w_0).rb_color = rb_color::RB_RED as i32 as u8;
                    } else {
                        if ((*w_0).rb_link[0 as i32 as usize]).is_null()
                            || (*(*w_0).rb_link[0 as i32 as usize]).rb_color as i32
                                == rb_color::RB_BLACK as i32
                        {
                            let mut y_0: *mut rb_node = (*w_0)
                                .rb_link[1 as i32 as usize];
                            (*y_0).rb_color = rb_color::RB_BLACK as i32 as u8;
                            (*w_0).rb_color = rb_color::RB_RED as i32 as u8;
                            (*w_0).rb_link[1 as i32 as usize] = (*y_0)
                                .rb_link[0 as i32 as usize];
                            (*y_0).rb_link[0 as i32 as usize] = w_0;
                            (*pa[(k - 1 as i32) as usize]).rb_link[0 as i32 as usize] = y_0;
                            w_0 = (*pa[(k - 1 as i32) as usize])
                                .rb_link[0 as i32 as usize];
                        }
                        (*w_0).rb_color = (*pa[(k - 1 as i32) as usize]).rb_color;
                        (*pa[(k - 1 as i32) as usize]).rb_color = rb_color::RB_BLACK
                            as i32 as u8;
                        (*(*w_0).rb_link[0 as i32 as usize]).rb_color = rb_color::RB_BLACK
                            as i32 as u8;
                        (*pa[(k - 1 as i32) as usize]).rb_link[0 as i32 as usize] = (*w_0)
                            .rb_link[1 as i32 as usize];
                        (*w_0).rb_link[1 as i32 as usize] = pa[(k - 1 as i32) as usize];
                        (*pa[(k - 2 as i32) as usize])
                            .rb_link[da[(k - 2 as i32) as usize] as usize] = w_0;
                        break;
                    }
                }
                k -= 1;
                k;
            }
        }
    }
    ((*(*table).rb_alloc).free)
        .expect("non-null function pointer")(p as *mut libc::c_void, (*table).rb_param);
    (*table).rb_count = ((*table).rb_count).wrapping_sub(1);
    (*table).rb_count;
    (*table).rb_generation = ((*table).rb_generation).wrapping_add(1);
    (*table).rb_generation;
    return item as *mut libc::c_void;
}
unsafe extern "C" fn rb_t_init(
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> i32 {
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut table: *const rb_table = vtable as *const rb_table;
    (*trav).rb_table = table;
    (*trav).rb_node = 0 as *mut gsl_bst_rb_node;
    (*trav).rb_height = 0 as i32 as size_t;
    (*trav).rb_generation = (*table).rb_generation;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn rb_t_first(
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const rb_table = vtable as *const rb_table;
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut x: *mut rb_node = 0 as *mut rb_node;
    (*trav).rb_table = table;
    (*trav).rb_height = 0 as i32 as size_t;
    (*trav).rb_generation = (*table).rb_generation;
    x = (*table).rb_root;
    if !x.is_null() {
        while !((*x).rb_link[0 as i32 as usize]).is_null() {
            if (*trav).rb_height >= 48 as i32 as u64 {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                    b"rb.c\0" as *const u8 as *const i8,
                    532 as i32,
                    GSL_ETABLE as i32,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh5 = (*trav).rb_height;
            (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
            (*trav).rb_stack[fresh5 as usize] = x;
            x = (*x).rb_link[0 as i32 as usize];
        }
    }
    (*trav).rb_node = x;
    return if !x.is_null() { (*x).rb_data } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn rb_t_last(
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const rb_table = vtable as *const rb_table;
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut x: *mut rb_node = 0 as *mut rb_node;
    (*trav).rb_table = table;
    (*trav).rb_height = 0 as i32 as size_t;
    (*trav).rb_generation = (*table).rb_generation;
    x = (*table).rb_root;
    if !x.is_null() {
        while !((*x).rb_link[1 as i32 as usize]).is_null() {
            if (*trav).rb_height >= 48 as i32 as u64 {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                    b"rb.c\0" as *const u8 as *const i8,
                    566 as i32,
                    GSL_ETABLE as i32,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh6 = (*trav).rb_height;
            (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
            (*trav).rb_stack[fresh6 as usize] = x;
            x = (*x).rb_link[1 as i32 as usize];
        }
    }
    (*trav).rb_node = x;
    return if !x.is_null() { (*x).rb_data } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn rb_t_find(
    mut item: *const libc::c_void,
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const rb_table = vtable as *const rb_table;
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut p: *mut rb_node = 0 as *mut rb_node;
    let mut q: *mut rb_node = 0 as *mut rb_node;
    (*trav).rb_table = table;
    (*trav).rb_height = 0 as i32 as size_t;
    (*trav).rb_generation = (*table).rb_generation;
    p = (*table).rb_root;
    while !p.is_null() {
        let mut cmp: i32 = ((*table).rb_compare)
            .expect("non-null function pointer")(item, (*p).rb_data, (*table).rb_param);
        if cmp < 0 as i32 {
            q = (*p).rb_link[0 as i32 as usize];
        } else if cmp > 0 as i32 {
            q = (*p).rb_link[1 as i32 as usize];
        } else {
            (*trav).rb_node = p;
            return (*p).rb_data;
        }
        if (*trav).rb_height >= 48 as i32 as u64 {
            gsl_error(
                b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                b"rb.c\0" as *const u8 as *const i8,
                610 as i32,
                GSL_ETABLE as i32,
            );
            return 0 as *mut libc::c_void;
        }
        let fresh7 = (*trav).rb_height;
        (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
        (*trav).rb_stack[fresh7 as usize] = p;
        p = q;
    }
    (*trav).rb_height = 0 as i32 as size_t;
    (*trav).rb_node = 0 as *mut gsl_bst_rb_node;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn rb_t_insert(
    mut item: *mut libc::c_void,
    mut vtrav: *mut libc::c_void,
    mut vtable: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut table: *mut rb_table = vtable as *mut rb_table;
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut p: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    p = rb_probe(item, table);
    if !p.is_null() {
        (*trav).rb_table = table;
        (*trav).rb_node = (p as *mut i8).offset(-(16 as u64 as isize)) as *mut rb_node;
        (*trav).rb_generation = ((*table).rb_generation).wrapping_sub(1 as i32 as u64);
        return *p;
    } else {
        rb_t_init(vtrav, vtable);
        return 0 as *mut libc::c_void;
    };
}
unsafe extern "C" fn rb_t_copy(
    mut vtrav: *mut libc::c_void,
    mut vsrc: *const libc::c_void,
) -> *mut libc::c_void {
    let mut src: *const rb_traverser = vsrc as *const rb_traverser;
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    if trav != src as *mut rb_traverser {
        (*trav).rb_table = (*src).rb_table;
        (*trav).rb_node = (*src).rb_node;
        (*trav).rb_generation = (*src).rb_generation;
        if (*trav).rb_generation == (*(*trav).rb_table).rb_generation {
            (*trav).rb_height = (*src).rb_height;
            memcpy(
                ((*trav).rb_stack).as_mut_ptr() as *mut libc::c_void,
                ((*src).rb_stack).as_ptr() as *const libc::c_void,
                (::core::mem::size_of::<*mut gsl_bst_rb_node>() as u64)
                    .wrapping_mul((*trav).rb_height),
            );
        }
    }
    return if !((*trav).rb_node).is_null() {
        (*(*trav).rb_node).rb_data
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn rb_t_next(mut vtrav: *mut libc::c_void) -> *mut libc::c_void {
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut x: *mut rb_node = 0 as *mut rb_node;
    if (*trav).rb_generation != (*(*trav).rb_table).rb_generation {
        rb_trav_refresh(trav);
    }
    x = (*trav).rb_node;
    if x.is_null() {
        return rb_t_first(vtrav, (*trav).rb_table as *const libc::c_void)
    } else if !((*x).rb_link[1 as i32 as usize]).is_null() {
        if (*trav).rb_height >= 48 as i32 as u64 {
            gsl_error(
                b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                b"rb.c\0" as *const u8 as *const i8,
                695 as i32,
                GSL_ETABLE as i32,
            );
            return 0 as *mut libc::c_void;
        }
        let fresh8 = (*trav).rb_height;
        (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
        (*trav).rb_stack[fresh8 as usize] = x;
        x = (*x).rb_link[1 as i32 as usize];
        while !((*x).rb_link[0 as i32 as usize]).is_null() {
            if (*trav).rb_height >= 48 as i32 as u64 {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                    b"rb.c\0" as *const u8 as *const i8,
                    705 as i32,
                    GSL_ETABLE as i32,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh9 = (*trav).rb_height;
            (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
            (*trav).rb_stack[fresh9 as usize] = x;
            x = (*x).rb_link[0 as i32 as usize];
        }
    } else {
        let mut y: *mut rb_node = 0 as *mut rb_node;
        loop {
            if (*trav).rb_height == 0 as i32 as u64 {
                (*trav).rb_node = 0 as *mut gsl_bst_rb_node;
                return 0 as *mut libc::c_void;
            }
            y = x;
            (*trav).rb_height = ((*trav).rb_height).wrapping_sub(1);
            x = (*trav).rb_stack[(*trav).rb_height as usize];
            if !(y == (*x).rb_link[1 as i32 as usize]) {
                break;
            }
        }
    }
    (*trav).rb_node = x;
    return (*x).rb_data;
}
unsafe extern "C" fn rb_t_prev(mut vtrav: *mut libc::c_void) -> *mut libc::c_void {
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut x: *mut rb_node = 0 as *mut rb_node;
    if (*trav).rb_generation != (*(*trav).rb_table).rb_generation {
        rb_trav_refresh(trav);
    }
    x = (*trav).rb_node;
    if x.is_null() {
        return rb_t_last(vtrav, (*trav).rb_table as *const libc::c_void)
    } else if !((*x).rb_link[0 as i32 as usize]).is_null() {
        if (*trav).rb_height >= 48 as i32 as u64 {
            gsl_error(
                b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                b"rb.c\0" as *const u8 as *const i8,
                756 as i32,
                GSL_ETABLE as i32,
            );
            return 0 as *mut libc::c_void;
        }
        let fresh10 = (*trav).rb_height;
        (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
        (*trav).rb_stack[fresh10 as usize] = x;
        x = (*x).rb_link[0 as i32 as usize];
        while !((*x).rb_link[1 as i32 as usize]).is_null() {
            if (*trav).rb_height >= 48 as i32 as u64 {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                    b"rb.c\0" as *const u8 as *const i8,
                    766 as i32,
                    GSL_ETABLE as i32,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh11 = (*trav).rb_height;
            (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
            (*trav).rb_stack[fresh11 as usize] = x;
            x = (*x).rb_link[1 as i32 as usize];
        }
    } else {
        let mut y: *mut rb_node = 0 as *mut rb_node;
        loop {
            if (*trav).rb_height == 0 as i32 as u64 {
                (*trav).rb_node = 0 as *mut gsl_bst_rb_node;
                return 0 as *mut libc::c_void;
            }
            y = x;
            (*trav).rb_height = ((*trav).rb_height).wrapping_sub(1);
            x = (*trav).rb_stack[(*trav).rb_height as usize];
            if !(y == (*x).rb_link[0 as i32 as usize]) {
                break;
            }
        }
    }
    (*trav).rb_node = x;
    return (*x).rb_data;
}
unsafe extern "C" fn rb_t_cur(mut vtrav: *const libc::c_void) -> *mut libc::c_void {
    let mut trav: *const rb_traverser = vtrav as *const rb_traverser;
    return if !((*trav).rb_node).is_null() {
        (*(*trav).rb_node).rb_data
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn rb_t_replace(
    mut vtrav: *mut libc::c_void,
    mut new_item: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut trav: *mut rb_traverser = vtrav as *mut rb_traverser;
    let mut old: *mut libc::c_void = 0 as *mut libc::c_void;
    old = (*(*trav).rb_node).rb_data;
    (*(*trav).rb_node).rb_data = new_item;
    return old;
}
unsafe extern "C" fn rb_trav_refresh(mut trav: *mut rb_traverser) {
    (*trav).rb_generation = (*(*trav).rb_table).rb_generation;
    if !((*trav).rb_node).is_null() {
        let mut cmp: Option<gsl_bst_cmp_function> = (*(*trav).rb_table).rb_compare;
        let mut param: *mut libc::c_void = (*(*trav).rb_table).rb_param;
        let mut node: *mut rb_node = (*trav).rb_node;
        let mut i: *mut rb_node = 0 as *mut rb_node;
        (*trav).rb_height = 0 as i32 as size_t;
        i = (*(*trav).rb_table).rb_root;
        while i != node {
            if (*trav).rb_height >= 48 as i32 as u64 {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8 as *const i8,
                    b"rb.c\0" as *const u8 as *const i8,
                    957 as i32,
                    GSL_ETABLE as i32,
                );
                return;
            }
            let fresh12 = (*trav).rb_height;
            (*trav).rb_height = ((*trav).rb_height).wrapping_add(1);
            (*trav).rb_stack[fresh12 as usize] = i;
            i = (*i)
                .rb_link[(cmp
                .expect(
                    "non-null function pointer",
                )((*node).rb_data, (*i).rb_data, param) > 0 as i32) as i32 as usize];
        }
    }
}
static mut rb_tree_type: gsl_bst_type = unsafe {
    {
        let mut init = gsl_bst_type {
            name: b"red-black\0" as *const u8 as *const i8,
            node_size: ::core::mem::size_of::<rb_node>() as u64,
            init: Some(
                rb_init
                    as unsafe extern "C" fn(
                        *const gsl_bst_allocator,
                        Option<gsl_bst_cmp_function>,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> i32,
            ),
            nodes: Some(rb_nodes as unsafe extern "C" fn(*const libc::c_void) -> size_t),
            insert: Some(
                rb_insert
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            find: Some(
                rb_find
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            remove: Some(
                rb_remove
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            empty: Some(rb_empty as unsafe extern "C" fn(*mut libc::c_void) -> i32),
            trav_init: Some(
                rb_t_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
            trav_first: Some(
                rb_t_first
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_last: Some(
                rb_t_last
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_find: Some(
                rb_t_find
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_insert: Some(
                rb_t_insert
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_copy: Some(
                rb_t_copy
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_next: Some(
                rb_t_next as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            trav_prev: Some(
                rb_t_prev as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            trav_cur: Some(
                rb_t_cur
                    as unsafe extern "C" fn(*const libc::c_void) -> *mut libc::c_void,
            ),
            trav_replace: Some(
                rb_t_replace
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_bst_rb: *const gsl_bst_type = unsafe {
    &rb_tree_type as *const gsl_bst_type
};