#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum archive_format {
    arf_hpbinary,
    arf_hpoldascii,
    arf_ustar,
    arf_tar,
    arf_crcascii,
    arf_newascii,
    arf_oldascii,
    arf_binary,
    arf_unknown,
}  // end of enum

#[no_mangle]
pub static mut reset_time_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut io_block_size: libc::c_int = 512 as libc::c_int;
#[no_mangle]
pub static mut archive_format: archive_format = arf_unknown;
#[no_mangle]
pub static mut create_dir_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut rename_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut rename_batch_file: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut table_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut unconditional_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut verbose_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dot_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut link_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut retain_time_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut crc_i_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut append_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut swap_bytes_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut swap_halfwords_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut swapping_halfwords: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut swapping_bytes: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut newdir_umask: mode_t = 0;
#[no_mangle]
pub static mut set_owner_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut set_owner: uid_t = 0;
#[no_mangle]
pub static mut set_group_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut set_group: gid_t = 0;
#[no_mangle]
pub static mut no_chown_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sparse_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut quiet_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut only_verify_crc_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut no_abs_paths_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut last_header_start: off_t = 0 as libc::c_int as off_t;
#[no_mangle]
pub static mut copy_matching_files: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut numeric_uid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pattern_file_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut new_media_message: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut new_media_message_with_number: *mut libc::c_char = 0
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut new_media_message_after_number: *mut libc::c_char = 0
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut archive_des: libc::c_int = 0;
#[no_mangle]
pub static mut archive_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut rsh_command_option: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut crc: uint32_t = 0;
#[no_mangle]
pub static mut input_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut output_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut input_buffer_size: size_t = 0;
#[no_mangle]
pub static mut in_buff: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut out_buff: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut input_size: size_t = 0;
#[no_mangle]
pub static mut output_size: size_t = 0;
#[no_mangle]
pub static mut input_bytes: off_t = 0;
#[no_mangle]
pub static mut output_bytes: off_t = 0;
#[no_mangle]
pub static mut directory_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut save_patterns: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut num_patterns: libc::c_int = 0;
#[no_mangle]
pub static mut name_end: libc::c_char = '\n' as i32 as libc::c_char;
#[no_mangle]
pub static mut input_is_special: libc::c_char = 0 as libc::c_int as libc::c_char;
#[no_mangle]
pub static mut output_is_special: libc::c_char = 0 as libc::c_int as libc::c_char;
#[no_mangle]
pub static mut input_is_seekable: libc::c_char = 0 as libc::c_int as libc::c_char;
#[no_mangle]
pub static mut output_is_seekable: libc::c_char = 0 as libc::c_int as libc::c_char;
#[no_mangle]
pub static mut warn_option: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut to_stdout_option: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut xstat: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
#[no_mangle]
pub static mut copy_function: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut change_directory_option: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut renumber_inodes_option: libc::c_int = 0;
#[no_mangle]
pub static mut ignore_devno_option: libc::c_int = 0;
#[no_mangle]
pub static mut ignore_dirnlink_option: libc::c_int = 0;
