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
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncat(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn cpio_set_c_name(file_hdr: *mut cpio_file_stat, name: *mut i8);
    static mut archive_format: archive_format;
    static mut numeric_uid: i32;
    fn warn_junk_bytes(bytes_skipped: i64);
    fn to_ascii(
        where_0: *mut i8,
        v: uintmax_t,
        digits: size_t,
        logbase: u32,
        nul: bool,
    ) -> i32;
    fn field_width_error(
        filename: *const i8,
        fieldname: *const i8,
        value: uintmax_t,
        width: size_t,
        nul: bool,
    );
    fn tape_buffered_write(in_buf: *mut i8, out_des: i32, num_bytes: off_t);
    fn tape_buffered_read(in_buf: *mut i8, in_des: i32, num_bytes: off_t);
    fn getgidbyname(group: *mut i8) -> *mut gid_t;
    fn getuidbyname(user: *mut i8) -> *mut uid_t;
    fn getuser(uid: uid_t) -> *mut i8;
    fn getgroup(gid: gid_t) -> *mut i8;
    fn from_ascii(where_0: *const i8, digs: size_t, logbase: u32) -> uintmax_t;
}
pub type __uint32_t = u32;
pub type __uintmax_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __off_t = i64;
pub type __time_t = i64;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: u32,
    pub c_dev_min: u32,
    pub c_rdev_maj: u32,
    pub c_rdev_min: u32,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut i8,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum archive_format {
    arf_unknown,
    arf_binary,
    arf_oldascii,
    arf_newascii,
    arf_crcascii,
    arf_tar,
    arf_ustar,
    arf_hpoldascii,
    arf_hpbinary,
}
impl archive_format {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            archive_format::arf_unknown => 0,
            archive_format::arf_binary => 1,
            archive_format::arf_oldascii => 2,
            archive_format::arf_newascii => 3,
            archive_format::arf_crcascii => 4,
            archive_format::arf_tar => 5,
            archive_format::arf_ustar => 6,
            archive_format::arf_hpoldascii => 7,
            archive_format::arf_hpbinary => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> archive_format {
        match value {
            0 => archive_format::arf_unknown,
            1 => archive_format::arf_binary,
            2 => archive_format::arf_oldascii,
            3 => archive_format::arf_newascii,
            4 => archive_format::arf_crcascii,
            5 => archive_format::arf_tar,
            6 => archive_format::arf_ustar,
            7 => archive_format::arf_hpoldascii,
            8 => archive_format::arf_hpbinary,
            _ => panic!("Invalid value for archive_format: {}", value),
        }
    }
}
impl AddAssign<u32> for archive_format {
    fn add_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for archive_format {
    fn sub_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for archive_format {
    fn mul_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for archive_format {
    fn div_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for archive_format {
    fn rem_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for archive_format {
    type Output = archive_format;
    fn add(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for archive_format {
    type Output = archive_format;
    fn sub(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for archive_format {
    type Output = archive_format;
    fn mul(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for archive_format {
    type Output = archive_format;
    fn div(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for archive_format {
    type Output = archive_format;
    fn rem(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union tar_record {
    pub header: tar_header,
    pub buffer: [i8; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_header {
    pub name: [i8; 100],
    pub mode: [i8; 8],
    pub uid: [i8; 8],
    pub gid: [i8; 8],
    pub size: [i8; 12],
    pub mtime: [i8; 12],
    pub chksum: [i8; 8],
    pub typeflag: i8,
    pub linkname: [i8; 100],
    pub magic: [i8; 6],
    pub version: [i8; 2],
    pub uname: [i8; 32],
    pub gname: [i8; 32],
    pub devmajor: [i8; 8],
    pub devminor: [i8; 8],
    pub prefix: [i8; 155],
}
unsafe extern "C" fn stash_tar_linkname(mut linkname: *mut i8) -> *mut i8 {
    static mut hold_tar_linkname: [i8; 101] = [0; 101];
    strncpy(hold_tar_linkname.as_mut_ptr(), linkname, 100 as i32 as u64);
    hold_tar_linkname[100 as i32 as usize] = '\0' as i32 as i8;
    return hold_tar_linkname.as_mut_ptr();
}
unsafe extern "C" fn split_long_name(mut name: *const i8, mut length: size_t) -> size_t {
    let mut i: size_t = 0;
    if length > 155 as i32 as u64 {
        length = (155 as i32 + 2 as i32) as size_t;
    }
    i = length.wrapping_sub(1 as i32 as u64);
    while i > 0 as i32 as u64 {
        if *name.offset(i as isize) as i32 == '/' as i32 {
            break;
        }
        i = i.wrapping_sub(1);
        i;
    }
    return i;
}
unsafe extern "C" fn stash_tar_filename(
    mut prefix: *mut i8,
    mut filename: *mut i8,
) -> *mut i8 {
    static mut hold_tar_filename: [i8; 257] = [0; 257];
    if prefix.is_null() || *prefix as i32 == '\0' as i32 {
        strncpy(hold_tar_filename.as_mut_ptr(), filename, 100 as i32 as u64);
        hold_tar_filename[100 as i32 as usize] = '\0' as i32 as i8;
    } else {
        strncpy(hold_tar_filename.as_mut_ptr(), prefix, 155 as i32 as u64);
        hold_tar_filename[155 as i32 as usize] = '\0' as i32 as i8;
        strcat(hold_tar_filename.as_mut_ptr(), b"/\0" as *const u8 as *const i8);
        strncat(hold_tar_filename.as_mut_ptr(), filename, 100 as i32 as u64);
        hold_tar_filename[(155 as i32 + 100 as i32) as usize] = '\0' as i32 as i8;
    }
    return hold_tar_filename.as_mut_ptr();
}
unsafe extern "C" fn to_oct_or_error(
    mut value: uintmax_t,
    mut digits: size_t,
    mut where_0: *mut i8,
    mut field: *const i8,
    mut file: *const i8,
) -> i32 {
    if to_ascii(where_0, value, digits, 3 as i32 as u32, 1 as i32 != 0) != 0 {
        field_width_error(file, field, value, digits, 1 as i32 != 0);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn tar_checksum(mut tar_hdr: *mut tar_header) -> u32 {
    let mut sum: u32 = 0 as i32 as u32;
    let mut p: *mut i8 = tar_hdr as *mut i8;
    let mut q: *mut i8 = p.offset(512 as i32 as isize);
    let mut i: i32 = 0;
    while p < ((*tar_hdr).chksum).as_mut_ptr() {
        let fresh0 = p;
        p = p.offset(1);
        sum = sum.wrapping_add((*fresh0 as i32 & 0xff as i32) as u32);
    }
    i = 0 as i32;
    while i < 8 as i32 {
        sum = sum.wrapping_add(' ' as i32 as u32);
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    while p < q {
        let fresh1 = p;
        p = p.offset(1);
        sum = sum.wrapping_add((*fresh1 as i32 & 0xff as i32) as u32);
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn write_out_tar_header(
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: i32,
) -> i32 {
    let mut name_len: i32 = 0;
    let mut tar_rec: tar_record = tar_record {
        header: tar_header {
            name: [0; 100],
            mode: [0; 8],
            uid: [0; 8],
            gid: [0; 8],
            size: [0; 12],
            mtime: [0; 12],
            chksum: [0; 8],
            typeflag: 0,
            linkname: [0; 100],
            magic: [0; 6],
            version: [0; 2],
            uname: [0; 32],
            gname: [0; 32],
            devmajor: [0; 8],
            devminor: [0; 8],
            prefix: [0; 155],
        },
    };
    let mut tar_hdr: *mut tar_header = &mut tar_rec as *mut tar_record
        as *mut tar_header;
    memset(
        &mut tar_rec as *mut tar_record as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<tar_record>() as u64,
    );
    name_len = strlen((*file_hdr).c_name) as i32;
    if name_len <= 100 as i32 {
        strncpy(((*tar_hdr).name).as_mut_ptr(), (*file_hdr).c_name, name_len as u64);
    } else {
        let mut prefix_len: i32 = split_long_name((*file_hdr).c_name, name_len as size_t)
            as i32;
        strncpy(((*tar_hdr).prefix).as_mut_ptr(), (*file_hdr).c_name, prefix_len as u64);
        strncpy(
            ((*tar_hdr).name).as_mut_ptr(),
            ((*file_hdr).c_name).offset(prefix_len as isize).offset(1 as i32 as isize),
            (name_len - prefix_len - 1 as i32) as u64,
        );
    }
    if to_oct_or_error(
        ((*file_hdr).c_mode
            & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                    | 0o100 as i32 >> 3 as i32 >> 3 as i32
                    | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                        | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32)
            as uintmax_t,
        8 as i32 as size_t,
        ((*tar_hdr).mode).as_mut_ptr(),
        b"mode\0" as *const u8 as *const i8,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as i32;
    }
    if to_oct_or_error(
        (*file_hdr).c_uid as uintmax_t,
        8 as i32 as size_t,
        ((*tar_hdr).uid).as_mut_ptr(),
        b"uid\0" as *const u8 as *const i8,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as i32;
    }
    if to_oct_or_error(
        (*file_hdr).c_gid as uintmax_t,
        8 as i32 as size_t,
        ((*tar_hdr).gid).as_mut_ptr(),
        b"gid\0" as *const u8 as *const i8,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as i32;
    }
    if to_oct_or_error(
        (*file_hdr).c_filesize as uintmax_t,
        12 as i32 as size_t,
        ((*tar_hdr).size).as_mut_ptr(),
        b"size\0" as *const u8 as *const i8,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as i32;
    }
    if to_oct_or_error(
        (*file_hdr).c_mtime as uintmax_t,
        12 as i32 as size_t,
        ((*tar_hdr).mtime).as_mut_ptr(),
        b"mtime\0" as *const u8 as *const i8,
        (*file_hdr).c_name,
    ) != 0
    {
        return 1 as i32;
    }
    match (*file_hdr).c_mode & 0o170000 as i32 as u32 {
        32768 => {
            if !((*file_hdr).c_tar_linkname).is_null() {
                strncpy(
                    ((*tar_hdr).linkname).as_mut_ptr(),
                    (*file_hdr).c_tar_linkname,
                    100 as i32 as u64,
                );
                (*tar_hdr).typeflag = '1' as i32 as i8;
                to_ascii(
                    ((*tar_hdr).size).as_mut_ptr(),
                    0 as i32 as uintmax_t,
                    12 as i32 as size_t,
                    3 as i32 as u32,
                    1 as i32 != 0,
                );
            } else {
                (*tar_hdr).typeflag = '0' as i32 as i8;
            }
        }
        16384 => {
            (*tar_hdr).typeflag = '5' as i32 as i8;
        }
        8192 => {
            (*tar_hdr).typeflag = '3' as i32 as i8;
        }
        24576 => {
            (*tar_hdr).typeflag = '4' as i32 as i8;
        }
        4096 => {
            (*tar_hdr).typeflag = '6' as i32 as i8;
        }
        40960 => {
            (*tar_hdr).typeflag = '2' as i32 as i8;
            strncpy(
                ((*tar_hdr).linkname).as_mut_ptr(),
                (*file_hdr).c_tar_linkname,
                100 as i32 as u64,
            );
            to_ascii(
                ((*tar_hdr).size).as_mut_ptr(),
                0 as i32 as uintmax_t,
                12 as i32 as size_t,
                3 as i32 as u32,
                1 as i32 != 0,
            );
        }
        _ => {}
    }
    if archive_format as u32 == archive_format::arf_ustar as i32 as u32 {
        let mut name: *mut i8 = 0 as *mut i8;
        strncpy(
            ((*tar_hdr).magic).as_mut_ptr(),
            b"ustar\0" as *const u8 as *const i8,
            6 as i32 as u64,
        );
        strncpy(
            ((*tar_hdr).version).as_mut_ptr(),
            b"00\0" as *const u8 as *const i8,
            2 as i32 as u64,
        );
        name = getuser((*file_hdr).c_uid);
        if !name.is_null() {
            strcpy(((*tar_hdr).uname).as_mut_ptr(), name);
        }
        name = getgroup((*file_hdr).c_gid);
        if !name.is_null() {
            strcpy(((*tar_hdr).gname).as_mut_ptr(), name);
        }
        if to_oct_or_error(
            (*file_hdr).c_rdev_maj as uintmax_t,
            8 as i32 as size_t,
            ((*tar_hdr).devmajor).as_mut_ptr(),
            b"devmajor\0" as *const u8 as *const i8,
            (*file_hdr).c_name,
        ) != 0
        {
            return 1 as i32;
        }
        if to_oct_or_error(
            (*file_hdr).c_rdev_min as uintmax_t,
            8 as i32 as size_t,
            ((*tar_hdr).devminor).as_mut_ptr(),
            b"devminor\0" as *const u8 as *const i8,
            (*file_hdr).c_name,
        ) != 0
        {
            return 1 as i32;
        }
    }
    to_ascii(
        ((*tar_hdr).chksum).as_mut_ptr(),
        tar_checksum(tar_hdr) as uintmax_t,
        8 as i32 as size_t,
        3 as i32 as u32,
        1 as i32 != 0,
    );
    tape_buffered_write(
        &mut tar_rec as *mut tar_record as *mut i8,
        out_des,
        512 as i32 as off_t,
    );
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn null_block(mut block: *mut i64, mut size: i32) -> i32 {
    let mut p: *mut i64 = block;
    let mut i: i32 = (size as u64).wrapping_div(::core::mem::size_of::<i64>() as u64)
        as i32;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = p;
        p = p.offset(1);
        if *fresh3 != 0 {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn read_in_tar_header(
    mut file_hdr: *mut cpio_file_stat,
    mut in_des: i32,
) {
    let mut bytes_skipped: i64 = 0 as i32 as i64;
    let mut warned: i32 = 0 as i32;
    let mut tar_rec: tar_record = tar_record {
        header: tar_header {
            name: [0; 100],
            mode: [0; 8],
            uid: [0; 8],
            gid: [0; 8],
            size: [0; 12],
            mtime: [0; 12],
            chksum: [0; 8],
            typeflag: 0,
            linkname: [0; 100],
            magic: [0; 6],
            version: [0; 2],
            uname: [0; 32],
            gname: [0; 32],
            devmajor: [0; 8],
            devminor: [0; 8],
            prefix: [0; 155],
        },
    };
    let mut tar_hdr: *mut tar_header = &mut tar_rec as *mut tar_record
        as *mut tar_header;
    let mut uidp: *mut uid_t = 0 as *mut uid_t;
    let mut gidp: *mut gid_t = 0 as *mut gid_t;
    tape_buffered_read(
        &mut tar_rec as *mut tar_record as *mut i8,
        in_des,
        512 as i32 as off_t,
    );
    if null_block(&mut tar_rec as *mut tar_record as *mut i64, 512 as i32) != 0 {
        cpio_set_c_name(file_hdr, b"TRAILER!!!\0" as *const u8 as *const i8 as *mut i8);
        return;
    }
    loop {
        (*file_hdr).c_chksum = from_ascii(
            ((*tar_hdr).chksum).as_mut_ptr(),
            ::core::mem::size_of::<[i8; 8]>() as u64,
            3 as i32 as u32,
        ) as uint32_t;
        if (*file_hdr).c_chksum != tar_checksum(tar_hdr) {
            if warned == 0 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid header: checksum error\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                warned = 1 as i32;
            }
            memmove(
                &mut tar_rec as *mut tar_record as *mut libc::c_void,
                (&mut tar_rec as *mut tar_record as *mut i8).offset(1 as i32 as isize)
                    as *const libc::c_void,
                (512 as i32 - 1 as i32) as u64,
            );
            tape_buffered_read(
                (&mut tar_rec as *mut tar_record as *mut i8)
                    .offset((512 as i32 - 1 as i32) as isize),
                in_des,
                1 as i32 as off_t,
            );
            bytes_skipped += 1;
            bytes_skipped;
        } else {
            if archive_format as u32 != archive_format::arf_ustar as i32 as u32 {
                cpio_set_c_name(
                    file_hdr,
                    stash_tar_filename(0 as *mut i8, ((*tar_hdr).name).as_mut_ptr()),
                );
            } else {
                cpio_set_c_name(
                    file_hdr,
                    stash_tar_filename(
                        ((*tar_hdr).prefix).as_mut_ptr(),
                        ((*tar_hdr).name).as_mut_ptr(),
                    ),
                );
            }
            (*file_hdr).c_nlink = 1 as i32 as size_t;
            (*file_hdr).c_mode = from_ascii(
                ((*tar_hdr).mode).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 8]>() as u64,
                3 as i32 as u32,
            ) as mode_t;
            (*file_hdr).c_mode = (*file_hdr).c_mode & 0o7777 as i32 as u32;
            if archive_format as u32 == archive_format::arf_ustar as i32 as u32
                && numeric_uid == 0
                && {
                    uidp = getuidbyname(((*tar_hdr).uname).as_mut_ptr());
                    !uidp.is_null()
                }
            {
                (*file_hdr).c_uid = *uidp;
            } else {
                (*file_hdr).c_uid = from_ascii(
                    ((*tar_hdr).uid).as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 8]>() as u64,
                    3 as i32 as u32,
                ) as uid_t;
            }
            if archive_format as u32 == archive_format::arf_ustar as i32 as u32
                && numeric_uid == 0
                && {
                    gidp = getgidbyname(((*tar_hdr).gname).as_mut_ptr());
                    !gidp.is_null()
                }
            {
                (*file_hdr).c_gid = *gidp;
            } else {
                (*file_hdr).c_gid = from_ascii(
                    ((*tar_hdr).gid).as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 8]>() as u64,
                    3 as i32 as u32,
                ) as gid_t;
            }
            (*file_hdr).c_filesize = from_ascii(
                ((*tar_hdr).size).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 12]>() as u64,
                3 as i32 as u32,
            ) as off_t;
            (*file_hdr).c_mtime = from_ascii(
                ((*tar_hdr).mtime).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 12]>() as u64,
                3 as i32 as u32,
            ) as time_t;
            (*file_hdr).c_rdev_maj = from_ascii(
                ((*tar_hdr).devmajor).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 8]>() as u64,
                3 as i32 as u32,
            ) as u32;
            (*file_hdr).c_rdev_min = from_ascii(
                ((*tar_hdr).devminor).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 8]>() as u64,
                3 as i32 as u32,
            ) as u32;
            (*file_hdr).c_tar_linkname = 0 as *const i8;
            match (*tar_hdr).typeflag as i32 {
                53 => {
                    (*file_hdr).c_mode |= 0o40000 as i32 as u32;
                }
                51 => {
                    (*file_hdr).c_mode |= 0o20000 as i32 as u32;
                    (*file_hdr).c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as i32 as off_t;
                }
                52 => {
                    (*file_hdr).c_mode |= 0o60000 as i32 as u32;
                    (*file_hdr).c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as i32 as off_t;
                }
                54 => {
                    (*file_hdr).c_mode |= 0o10000 as i32 as u32;
                    (*file_hdr).c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as i32 as off_t;
                }
                50 => {
                    (*file_hdr).c_mode |= 0o120000 as i32 as u32;
                    (*file_hdr).c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as i32 as off_t;
                }
                49 => {
                    (*file_hdr).c_mode |= 0o100000 as i32 as u32;
                    (*file_hdr).c_tar_linkname = stash_tar_linkname(
                        ((*tar_hdr).linkname).as_mut_ptr(),
                    );
                    (*file_hdr).c_filesize = 0 as i32 as off_t;
                }
                0 => {
                    if *((*file_hdr).c_name)
                        .offset(
                            ((*file_hdr).c_namesize).wrapping_sub(1 as i32 as u64)
                                as isize,
                        ) as i32 == '/' as i32
                    {
                        (*file_hdr).c_mode |= 0o40000 as i32 as u32;
                    } else {
                        (*file_hdr).c_mode |= 0o100000 as i32 as u32;
                    }
                }
                48 | 55 | _ => {
                    (*file_hdr).c_mode |= 0o100000 as i32 as u32;
                }
            }
            break;
        }
    }
    if bytes_skipped > 0 as i32 as i64 {
        warn_junk_bytes(bytes_skipped);
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_tar_header(mut buf: *mut i8) -> i32 {
    let mut tar_hdr: *mut tar_header = buf as *mut tar_header;
    let mut chksum: u64 = 0;
    chksum = from_ascii(
        ((*tar_hdr).chksum).as_mut_ptr(),
        ::core::mem::size_of::<[i8; 8]>() as u64,
        3 as i32 as u32,
    );
    if chksum != tar_checksum(tar_hdr) as u64 {
        return 0 as i32;
    }
    if strncmp(
        ((*tar_hdr).magic).as_mut_ptr(),
        b"ustar\0" as *const u8 as *const i8,
        (6 as i32 - 1 as i32) as u64,
    ) == 0
    {
        return 2 as i32;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_tar_filename_too_long(mut name: *mut i8) -> i32 {
    let mut whole_name_len: i32 = 0;
    let mut prefix_name_len: i32 = 0;
    whole_name_len = strlen(name) as i32;
    if whole_name_len <= 100 as i32 {
        return 0 as i32;
    }
    if archive_format as u32 != archive_format::arf_ustar as i32 as u32 {
        return 1 as i32;
    }
    if whole_name_len > 100 as i32 + 155 as i32 + 1 as i32 {
        return 1 as i32;
    }
    prefix_name_len = split_long_name(name, whole_name_len as size_t) as i32;
    if prefix_name_len == 0 as i32
        || whole_name_len - prefix_name_len - 1 as i32 > 100 as i32
    {
        return 1 as i32;
    }
    return 0 as i32;
}