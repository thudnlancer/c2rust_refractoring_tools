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
    fn xzalloc(s: size_t) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cflow_depmap {
    pub nrows: size_t,
    pub rowlen: size_t,
    pub r: [u32; 1],
}
pub type cflow_depmap_t = *mut cflow_depmap;
unsafe extern "C" fn transitive_closure(mut R: *mut u32, mut n: i32) {
    let mut rowsize: size_t = 0;
    let mut mask: u32 = 0;
    let mut rowj: *mut u32 = 0 as *mut u32;
    let mut rp: *mut u32 = 0 as *mut u32;
    let mut rend: *mut u32 = 0 as *mut u32;
    let mut ccol: *mut u32 = 0 as *mut u32;
    let mut relend: *mut u32 = 0 as *mut u32;
    let mut cword: *mut u32 = 0 as *mut u32;
    let mut rowi: *mut u32 = 0 as *mut u32;
    rowsize = (n as u64)
        .wrapping_add(
            (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(
            (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_mul(::core::mem::size_of::<u32>() as u64);
    relend = (R as *mut i8).offset((n as u64).wrapping_mul(rowsize) as isize)
        as *mut u32;
    cword = R;
    mask = 1 as i32 as u32;
    rowi = R;
    while rowi < relend {
        ccol = cword;
        rowj = R;
        while rowj < relend {
            if *ccol & mask != 0 {
                rp = rowi;
                rend = (rowj as *mut i8).offset(rowsize as isize) as *mut u32;
                while rowj < rend {
                    let fresh0 = rp;
                    rp = rp.offset(1);
                    let fresh1 = rowj;
                    rowj = rowj.offset(1);
                    *fresh1 |= *fresh0;
                }
            } else {
                rowj = (rowj as *mut i8).offset(rowsize as isize) as *mut u32;
            }
            ccol = (ccol as *mut i8).offset(rowsize as isize) as *mut u32;
        }
        mask <<= 1 as i32;
        if mask == 0 as i32 as u32 {
            mask = 1 as i32 as u32;
            cword = cword.offset(1);
            cword;
        }
        rowi = (rowi as *mut i8).offset(rowsize as isize) as *mut u32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn depmap_alloc(mut count: size_t) -> cflow_depmap_t {
    let mut size: size_t = count
        .wrapping_add(
            (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(
            (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
        );
    let mut dmap: cflow_depmap_t = xzalloc(
        (::core::mem::size_of::<cflow_depmap>() as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_add(
                count
                    .wrapping_mul(size)
                    .wrapping_mul(::core::mem::size_of::<u32>() as u64),
            ),
    ) as cflow_depmap_t;
    (*dmap).nrows = count;
    (*dmap).rowlen = size;
    return dmap;
}
unsafe extern "C" fn depmap_rowptr(
    mut dmap: cflow_depmap_t,
    mut row: size_t,
) -> *mut u32 {
    return ((*dmap).r).as_mut_ptr().offset(((*dmap).rowlen).wrapping_mul(row) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn depmap_set(
    mut dmap: cflow_depmap_t,
    mut row: size_t,
    mut col: size_t,
) {
    let mut rptr: *mut u32 = depmap_rowptr(dmap, row);
    *rptr
        .offset(
            col
                .wrapping_div(
                    (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
                ) as isize,
        )
        |= ((1 as i32)
            << col
                .wrapping_rem(
                    (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
                )) as u32;
}
#[no_mangle]
pub unsafe extern "C" fn depmap_isset(
    mut dmap: cflow_depmap_t,
    mut row: size_t,
    mut col: size_t,
) -> i32 {
    let mut rptr: *mut u32 = depmap_rowptr(dmap, row);
    return (*rptr
        .offset(
            col
                .wrapping_div(
                    (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
                ) as isize,
        )
        & ((1 as i32)
            << col
                .wrapping_rem(
                    (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
                )) as u32 != 0 as i32 as u32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn depmap_tc(mut dmap: cflow_depmap_t) {
    transitive_closure(((*dmap).r).as_mut_ptr(), (*dmap).nrows as i32);
}