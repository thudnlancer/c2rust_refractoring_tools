use std::ffi::CString;
use std::os::raw::{c_char, c_uint, c_int, c_long, c_ulong};

pub type Uint32 = c_uint;
pub type Uid = c_uint;
pub type Gid = c_uint;
pub type Mode = c_uint;
pub type Off = c_long;
pub type Size = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Unknown,
    Binary,
    OldAscii,
    NewAscii,
    CrcAscii,
    Tar,
    Ustar,
    HpOldAscii,
    HpBinary,
}

#[derive(Debug, Default)]
pub struct ArchiveConfig {
    pub reset_time_flag: bool,
    pub io_block_size: i32,
    pub archive_format: ArchiveFormat,
    pub create_dir_flag: bool,
    pub rename_flag: bool,
    pub rename_batch_file: Option<CString>,
    pub table_flag: bool,
    pub unconditional_flag: bool,
    pub verbose_flag: bool,
    pub dot_flag: bool,
    pub link_flag: bool,
    pub retain_time_flag: bool,
    pub crc_i_flag: bool,
    pub append_flag: bool,
    pub swap_bytes_flag: bool,
    pub swap_halfwords_flag: bool,
    pub swapping_halfwords: bool,
    pub swapping_bytes: bool,
    pub newdir_umask: Mode,
    pub set_owner_flag: bool,
    pub set_owner: Uid,
    pub set_group_flag: bool,
    pub set_group: Gid,
    pub no_chown_flag: bool,
    pub sparse_flag: bool,
    pub quiet_flag: bool,
    pub only_verify_crc_flag: bool,
    pub no_abs_paths_flag: bool,
    pub last_header_start: Off,
    pub copy_matching_files: bool,
    pub numeric_uid: bool,
    pub pattern_file_name: Option<CString>,
    pub new_media_message: Option<CString>,
    pub new_media_message_with_number: Option<CString>,
    pub new_media_message_after_number: Option<CString>,
    pub archive_des: i32,
    pub archive_name: Option<CString>,
    pub rsh_command_option: Option<CString>,
    pub crc: Uint32,
    pub input_buffer: Option<CString>,
    pub output_buffer: Option<CString>,
    pub input_buffer_size: Size,
    pub in_buff: Option<CString>,
    pub out_buff: Option<CString>,
    pub input_size: Size,
    pub output_size: Size,
    pub input_bytes: Off,
    pub output_bytes: Off,
    pub directory_name: Option<CString>,
    pub save_patterns: Vec<CString>,
    pub num_patterns: i32,
    pub name_end: char,
    pub input_is_special: bool,
    pub output_is_special: bool,
    pub input_is_seekable: bool,
    pub output_is_seekable: bool,
    pub warn_option: u32,
    pub to_stdout_option: bool,
    pub change_directory_option: Option<CString>,
    pub renumber_inodes_option: i32,
    pub ignore_devno_option: i32,
    pub ignore_dirnlink_option: i32,
}

impl Default for ArchiveFormat {
    fn default() -> Self {
        ArchiveFormat::Unknown
    }
}

lazy_static::lazy_static! {
    pub static ref CONFIG: std::sync::Mutex<ArchiveConfig> = std::sync::Mutex::new(ArchiveConfig::default());
}