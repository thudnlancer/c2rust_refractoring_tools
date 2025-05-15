use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __mode_t = u32;
pub type __off_t = i64;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = u64;
pub type uint32_t = __uint32_t;
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
#[no_mangle]
pub static mut reset_time_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut io_block_size: i32 = 512 as i32;
#[no_mangle]
pub static mut archive_format: archive_format = archive_format::arf_unknown;
#[no_mangle]
pub static mut create_dir_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut rename_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut rename_batch_file: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut table_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut unconditional_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut verbose_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut dot_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut link_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut retain_time_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut crc_i_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut append_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut swap_bytes_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut swap_halfwords_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut swapping_halfwords: i32 = 0 as i32;
#[no_mangle]
pub static mut swapping_bytes: i32 = 0 as i32;
#[no_mangle]
pub static mut newdir_umask: mode_t = 0;
#[no_mangle]
pub static mut set_owner_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut set_owner: uid_t = 0;
#[no_mangle]
pub static mut set_group_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut set_group: gid_t = 0;
#[no_mangle]
pub static mut no_chown_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut sparse_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut quiet_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut only_verify_crc_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut no_abs_paths_flag: i32 = 0 as i32;
#[no_mangle]
pub static mut last_header_start: off_t = 0 as i32 as off_t;
#[no_mangle]
pub static mut copy_matching_files: i32 = 1 as i32;
#[no_mangle]
pub static mut numeric_uid: i32 = 0 as i32;
#[no_mangle]
pub static mut pattern_file_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut new_media_message: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut new_media_message_with_number: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut new_media_message_after_number: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut archive_des: i32 = 0;
#[no_mangle]
pub static mut archive_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut rsh_command_option: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut crc: uint32_t = 0;
#[no_mangle]
pub static mut input_buffer: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut output_buffer: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut input_buffer_size: size_t = 0;
#[no_mangle]
pub static mut in_buff: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut out_buff: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut input_size: size_t = 0;
#[no_mangle]
pub static mut output_size: size_t = 0;
#[no_mangle]
pub static mut input_bytes: off_t = 0;
#[no_mangle]
pub static mut output_bytes: off_t = 0;
#[no_mangle]
pub static mut directory_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut save_patterns: *mut *mut i8 = 0 as *const *mut i8 as *mut *mut i8;
#[no_mangle]
pub static mut num_patterns: i32 = 0;
#[no_mangle]
pub static mut name_end: i8 = '\n' as i32 as i8;
#[no_mangle]
pub static mut input_is_special: i8 = 0 as i32 as i8;
#[no_mangle]
pub static mut output_is_special: i8 = 0 as i32 as i8;
#[no_mangle]
pub static mut input_is_seekable: i8 = 0 as i32 as i8;
#[no_mangle]
pub static mut output_is_seekable: i8 = 0 as i32 as i8;
#[no_mangle]
pub static mut warn_option: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut to_stdout_option: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut xstat: Option<unsafe extern "C" fn() -> i32> = None;
#[no_mangle]
pub static mut copy_function: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut change_directory_option: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut renumber_inodes_option: i32 = 0;
#[no_mangle]
pub static mut ignore_devno_option: i32 = 0;
#[no_mangle]
pub static mut ignore_dirnlink_option: i32 = 0;