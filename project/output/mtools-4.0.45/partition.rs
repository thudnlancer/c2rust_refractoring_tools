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
    pub type doscp_t;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static mut mtools_skip_check: u32;
    fn printOom();
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn limitSizeToOffT(len: *mut size_t, maxLen: mt_off_t);
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type mt_off_t = off_t;
pub type smt_off_t = mt_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsc {
    pub byte0: u8,
    pub head: u8,
    pub sector: u8,
    pub cyl: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partition {
    pub start: hsc,
    pub end: hsc,
    pub start_sect: [u8; 4],
    pub nr_sects: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Partition_t {
    pub head: Stream_t,
    pub offset: mt_off_t,
    pub size: mt_off_t,
    pub nbSect: uint32_t,
    pub pos: uint8_t,
    pub sectors: uint8_t,
    pub heads: uint8_t,
    pub cyclinders: uint16_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn print_hsc(mut h: *mut hsc) {
    printf(
        b" h=%d s=%d c=%d\n\0" as *const u8 as *const i8,
        (*h).head as i32,
        ((*h).sector as i32 & 0x3f as i32) as uint8_t as i32,
        ((*h).cyl as i32 | ((*h).sector as i32 & 0xc0 as i32) << 2 as i32) as uint16_t
            as i32,
    );
}
unsafe extern "C" fn overlapCheck(
    mut partTable: *mut partition,
    mut i: u32,
    mut start: uint32_t,
    mut end: uint32_t,
) -> i32 {
    let mut partition: *mut partition = &mut *partTable.offset(i as isize)
        as *mut partition;
    if (*partition).end.byte0 == 0 {
        return 0 as i32;
    }
    if end
        > (((*partition).start_sect[0 as i32 as usize] as i32
            + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32
            + (((*((*partition).start_sect)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t
        && (start
            < ((((*partition).start_sect[0 as i32 as usize] as i32
                + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t)
                .wrapping_add(
                    (((*partition).nr_sects[0 as i32 as usize] as i32
                        + (((*partition).nr_sects[1 as i32 as usize] as i32)
                            << 8 as i32)) as uint16_t as i32
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(0 as i32 as isize) as i32
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                            as i32) << 16 as i32)) as uint32_t,
                )
            || ((((*partition).start_sect[0 as i32 as usize] as i32
                + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t)
                .wrapping_add(
                    (((*partition).nr_sects[0 as i32 as usize] as i32
                        + (((*partition).nr_sects[1 as i32 as usize] as i32)
                            << 8 as i32)) as uint16_t as i32
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(0 as i32 as isize) as i32
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                            as i32) << 16 as i32)) as uint32_t,
                )
                < (((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t)
    {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn findOverlap(
    mut partTable: *mut partition,
    mut until: u32,
    mut start: uint32_t,
    mut end: uint32_t,
) -> u32 {
    let mut i: u32 = 0;
    i = 1 as i32 as u32;
    while i <= until {
        if overlapCheck(partTable, i, start, end) != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn consistencyCheck(
    mut partTable: *mut partition,
    mut doprint: i32,
    mut verbose: i32,
    mut has_activated: *mut i32,
    mut tot_sectors: uint32_t,
    mut used_dev: *mut device,
    mut target_partition: u32,
) -> i32 {
    let mut i: u32 = 0;
    let mut inconsistency: bool = false;
    inconsistency = 0 as i32 != 0;
    *has_activated = 0 as i32;
    i = 1 as i32 as u32;
    while i <= 4 as i32 as u32 {
        let mut j: u32 = 0;
        let mut partition: *mut partition = &mut *partTable.offset(i as isize)
            as *mut partition;
        if !((*partition).end.byte0 == 0) {
            if (*partition).start.byte0 != 0 {
                *has_activated += 1;
                *has_activated;
            }
            if ((((*partition).start_sect[0 as i32 as usize] as i32
                + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*partition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t)
                .wrapping_add(
                    (((*partition).nr_sects[0 as i32 as usize] as i32
                        + (((*partition).nr_sects[1 as i32 as usize] as i32)
                            << 8 as i32)) as uint16_t as i32
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(0 as i32 as isize) as i32
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                            as i32) << 16 as i32)) as uint32_t,
                )
                < (((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t
            {
                fprintf(
                    stderr,
                    b"End of partition %d before its begin\n\0" as *const u8
                        as *const i8,
                    i,
                );
            }
            j = findOverlap(
                partTable,
                i.wrapping_sub(1 as i32 as u32),
                (((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t,
                ((((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t)
                    .wrapping_add(
                        (((*partition).nr_sects[0 as i32 as usize] as i32
                            + (((*partition).nr_sects[1 as i32 as usize] as i32)
                                << 8 as i32)) as uint16_t as i32
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(0 as i32 as isize) as i32
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                                as i32) << 16 as i32)) as uint32_t,
                    ),
            );
            if j != 0 {
                fprintf(
                    stderr,
                    b"Partitions %d and %d overlap\n\0" as *const u8 as *const i8,
                    j,
                    i,
                );
                inconsistency = 1 as i32 != 0;
            }
            if tot_sectors != 0
                && ((((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t)
                    .wrapping_add(
                        (((*partition).nr_sects[0 as i32 as usize] as i32
                            + (((*partition).nr_sects[1 as i32 as usize] as i32)
                                << 8 as i32)) as uint16_t as i32
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(0 as i32 as isize) as i32
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                                as i32) << 16 as i32)) as uint32_t,
                    ) > tot_sectors
            {
                fprintf(
                    stderr,
                    b"Partition %d extends beyond end of disk\n\0" as *const u8
                        as *const i8,
                    i,
                );
            }
            if doprint != 0 && verbose != 0 {
                if i == target_partition {
                    putchar('*' as i32);
                } else {
                    putchar(' ' as i32);
                }
                printf(b"Partition %d\n\0" as *const u8 as *const i8, i);
                printf(
                    b"  active=%x\n\0" as *const u8 as *const i8,
                    (*partition).start.byte0 as i32,
                );
                printf(b"  start:\0" as *const u8 as *const i8);
                print_hsc(&mut (*partition).start);
                printf(
                    b"  type=0x%x\n\0" as *const u8 as *const i8,
                    (*partition).end.byte0 as i32,
                );
                printf(b"  end:\0" as *const u8 as *const i8);
                print_hsc(&mut (*partition).end);
                printf(
                    b"  start=%d\n\0" as *const u8 as *const i8,
                    (((*partition).start_sect[0 as i32 as usize] as i32
                        + (((*partition).start_sect[1 as i32 as usize] as i32)
                            << 8 as i32)) as uint16_t as i32
                        + (((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(0 as i32 as isize) as i32
                            + ((*((*partition).start_sect)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                            as i32) << 16 as i32)) as uint32_t,
                );
                printf(
                    b"  nr=%d\n\0" as *const u8 as *const i8,
                    (((*partition).nr_sects[0 as i32 as usize] as i32
                        + (((*partition).nr_sects[1 as i32 as usize] as i32)
                            << 8 as i32)) as uint16_t as i32
                        + (((*((*partition).nr_sects)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(0 as i32 as isize) as i32
                            + ((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                            as i32) << 16 as i32)) as uint32_t,
                );
                printf(b"\n\0" as *const u8 as *const i8);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return inconsistency as i32;
}
unsafe extern "C" fn limit_size(
    mut This: *mut Partition_t,
    mut start: mt_off_t,
    mut len: *mut size_t,
) -> i32 {
    if start > (*This).size {
        return -(1 as i32);
    }
    limitSizeToOffT(len, (*This).size - start);
    return 0 as i32;
}
unsafe extern "C" fn partition_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if limit_size(This, start, &mut len) < 0 as i32 {
        return -(1 as i32) as ssize_t;
    }
    return ((*(*(*This).head.Next).Class).pread)
        .expect(
            "non-null function pointer",
        )((*This).head.Next, buf, start + (*This).offset, len);
}
unsafe extern "C" fn partition_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if limit_size(This, start, &mut len) < 0 as i32 {
        return -(1 as i32) as ssize_t;
    }
    return ((*(*(*This).head.Next).Class).pwrite)
        .expect(
            "non-null function pointer",
        )((*This).head.Next, buf, start + (*This).offset, len);
}
unsafe extern "C" fn partition_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut uint32_t,
) -> i32 {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if !date.is_null() || !type_0.is_null() || !address.is_null() {
        let mut ret: i32 = ((*(*(*This).head.Next).Class).get_data)
            .expect(
                "non-null function pointer",
            )((*This).head.Next, date, 0 as *mut mt_off_t, type_0, address);
        if ret < 0 as i32 {
            return ret;
        }
    }
    if !size.is_null() {
        *size = (*This).size * 512 as i32 as i64;
    }
    return 0 as i32;
}
unsafe extern "C" fn partition_geom(
    mut Stream: *mut Stream_t,
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> i32 {
    let mut This: *mut Partition_t = Stream as *mut Partition_t;
    if (*dev).tot_sectors == 0 {
        (*dev).tot_sectors = (*This).nbSect;
    }
    return 0 as i32;
}
static mut PartitionClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                partition_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                partition_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: None,
            set_geom: Some(
                partition_geom
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device,
                        *mut device,
                    ) -> i32,
            ),
            get_data: Some(
                partition_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut i32,
                        *mut uint32_t,
                    ) -> i32,
            ),
            pre_allocate: None,
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn OpenPartition(
    mut Next: *mut Stream_t,
    mut dev: *mut device,
    mut errmsg: *mut i8,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut Partition_t = 0 as *mut Partition_t;
    let mut has_activated: i32 = 0;
    let mut buf: [u8; 2048] = [0; 2048];
    let mut partTable: *mut partition = buf.as_mut_ptr().offset(0x1ae as i32 as isize)
        as *mut partition;
    let mut partOff: uint32_t = 0;
    let mut partition: *mut partition = 0 as *mut partition;
    if dev.is_null() || (*dev).partition > 4 as i32 as u32
        || (*dev).partition <= 0 as i32 as u32
    {
        fprintf(
            stderr,
            b"Invalid partition %d (must be between 1 and 4), ignoring it\n\0"
                as *const u8 as *const i8,
            (*dev).partition,
        );
        return 0 as *mut Stream_t;
    }
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Partition_t>() as u64)
        as *mut Partition_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<Partition_t>() as u64,
    );
    init_head(&mut (*This).head, &mut PartitionClass, Next);
    if !(force_pread(
        (*This).head.Next,
        buf.as_mut_ptr() as *mut i8,
        0 as i32 as mt_off_t,
        512 as i32 as size_t,
    ) != 512 as i32 as i64)
    {
        if (*buf.as_mut_ptr().offset(510 as i32 as isize).offset(0 as i32 as isize)
            as i32
            + ((*buf.as_mut_ptr().offset(510 as i32 as isize).offset(1 as i32 as isize)
                as i32) << 8 as i32)) as uint16_t as i32 != 0xaa55 as i32
        {
            if !errmsg.is_null() {
                sprintf(
                    errmsg,
                    b"Device does not have a BIOS partition table\n\0" as *const u8
                        as *const i8,
                );
            }
        } else {
            partition = &mut *partTable.offset((*dev).partition as isize)
                as *mut partition;
            if (*partition).end.byte0 == 0 {
                if !errmsg.is_null() {
                    sprintf(
                        errmsg,
                        b"Partition %d does not exist\n\0" as *const u8 as *const i8,
                        (*dev).partition,
                    );
                }
            } else {
                partOff = (((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t;
                if !maxSize.is_null() {
                    if partOff as i64 > *maxSize >> 9 as i32 {
                        if !errmsg.is_null() {
                            sprintf(
                                errmsg,
                                b"init: Big disks not supported\0" as *const u8 as *const i8,
                            );
                        }
                        current_block = 10885379428911626165;
                    } else {
                        *maxSize -= (partOff << 9 as i32) as i64;
                        if *maxSize
                            > ((((*partition).nr_sects[0 as i32 as usize] as i32
                                + (((*partition).nr_sects[1 as i32 as usize] as i32)
                                    << 8 as i32)) as uint16_t as i32
                                + (((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    .offset(0 as i32 as isize) as i32
                                    + ((*((*partition).nr_sects)
                                        .as_mut_ptr()
                                        .offset(2 as i32 as isize)
                                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                                    as i32) << 16 as i32)) as uint32_t as mt_off_t) << 9 as i32
                        {
                            *maxSize = ((((*partition).nr_sects[0 as i32 as usize] as i32
                                + (((*partition).nr_sects[1 as i32 as usize] as i32)
                                    << 8 as i32)) as uint16_t as i32
                                + (((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    .offset(0 as i32 as isize) as i32
                                    + ((*((*partition).nr_sects)
                                        .as_mut_ptr()
                                        .offset(2 as i32 as isize)
                                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                                    as i32) << 16 as i32)) as uint32_t as mt_off_t) << 9 as i32;
                        }
                        current_block = 11057878835866523405;
                    }
                } else {
                    current_block = 11057878835866523405;
                }
                match current_block {
                    10885379428911626165 => {}
                    _ => {
                        (*This).offset = (partOff as mt_off_t) << 9 as i32;
                        if mtools_skip_check == 0
                            && consistencyCheck(
                                buf.as_mut_ptr().offset(0x1ae as i32 as isize)
                                    as *mut partition,
                                0 as i32,
                                0 as i32,
                                &mut has_activated,
                                (*dev).tot_sectors,
                                dev,
                                0 as i32 as u32,
                            ) != 0
                        {
                            fprintf(
                                stderr,
                                b"Warning: inconsistent partition table\n\0" as *const u8
                                    as *const i8,
                            );
                            fprintf(
                                stderr,
                                b"Possibly unpartitioned device\n\0" as *const u8
                                    as *const i8,
                            );
                            fprintf(
                                stderr,
                                b"\n*** Maybe try without partition=%d in device definition ***\n\n\0"
                                    as *const u8 as *const i8,
                                (*dev).partition,
                            );
                            fprintf(
                                stderr,
                                b"If this is a PCMCIA card, or a disk partitioned on another computer, this message may be in error: add mtools_skip_check=1 to your .mtoolsrc file to suppress this warning\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        (*This).nbSect = (((*partition).nr_sects[0 as i32 as usize]
                            as i32
                            + (((*partition).nr_sects[1 as i32 as usize] as i32)
                                << 8 as i32)) as uint16_t as i32
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(0 as i32 as isize) as i32
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                                as i32) << 16 as i32)) as uint32_t;
                        (*dev).tot_sectors = (*This).nbSect;
                        (*This).size = ((*This).nbSect as mt_off_t) << 9 as i32;
                        return &mut (*This).head;
                    }
                }
            }
        }
    }
    free(This as *mut i8 as *mut libc::c_void);
    return 0 as *mut Stream_t;
}