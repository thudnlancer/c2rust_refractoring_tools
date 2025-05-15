/* global.rs - global variables and initial values for cpio.
   Copyright (C) 1990-2023 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public
   License along with this program; if not, write to the Free
   Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
   Boston, MA 02110-1301 USA.  */

use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicUsize, Ordering};

/* If true, reset access times after reading files (-a).  */
static RESET_TIME_FLAG: AtomicBool = AtomicBool::new(false);

/* Block size value, initially 512.  -B sets to 5120.  */
static IO_BLOCK_SIZE: AtomicI32 = AtomicI32::new(512);

/* The header format to recognize and produce.  */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Unknown,
    // Other variants would go here
}
static ARCHIVE_FORMAT: std::sync::Mutex<ArchiveFormat> = std::sync::Mutex::new(ArchiveFormat::Unknown);

/* If true, create directories as needed. (-d with -i or -p) */
static CREATE_DIR_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, interactively rename files. (-r) */
static RENAME_FLAG: AtomicBool = AtomicBool::new(false);

/* If non-NULL, the name of a file that will be read to
   rename all of the files in the archive.  --rename-batch-file.  */
static RENAME_BATCH_FILE: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/* If true, print a table of contents of input. (-t) */
static TABLE_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, copy unconditionally (older replaces newer). (-u) */
static UNCONDITIONAL_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, list the files processed, or ls -l style output with -t. (-v) */
static VERBOSE_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, print a . for each file processed. (-V) */
static DOT_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, link files whenever possible.  Used with -p option. (-l) */
static LINK_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, retain previous file modification time. (-m) */
static RETAIN_TIME_FLAG: AtomicBool = AtomicBool::new(false);

/* Set true if crc_flag is true and we are doing a cpio -i.  Used
   by copy_files so it knows whether to compute the crc.  */
static CRC_I_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, append to end of archive. (-A) */
static APPEND_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, swap bytes of each file during cpio -i.  */
static SWAP_BYTES_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, swap halfwords of each file during cpio -i.  */
static SWAP_HALFWORDS_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, we are swapping halfwords on the current file.  */
static SWAPPING_HALFWORDS: AtomicBool = AtomicBool::new(false);

/* If true, we are swapping bytes on the current file.  */
static SWAPPING_BYTES: AtomicBool = AtomicBool::new(false);

/* Umask for creating new directories */
static NEWDIR_UMASK: std::sync::Mutex<u32> = std::sync::Mutex::new(0o022);

/* If true, set ownership of all files to UID `set_owner'.  */
static SET_OWNER_FLAG: AtomicBool = AtomicBool::new(false);
static SET_OWNER: std::sync::Mutex<u32> = std::sync::Mutex::new(0);

/* If true, set group ownership of all files to GID `set_group'.  */
static SET_GROUP_FLAG: AtomicBool = AtomicBool::new(false);
static SET_GROUP: std::sync::Mutex<u32> = std::sync::Mutex::new(0);

/* If true, do not chown the files.  */
static NO_CHOWN_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, try to write sparse ("holey") files.  */
static SPARSE_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, don't report number of blocks copied.  */
static QUIET_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, only read the archive and verify the files' CRC's, don't
   actually extract the files. */
static ONLY_VERIFY_CRC_FLAG: AtomicBool = AtomicBool::new(false);

/* If true, don't use any absolute paths, prefix them by `./'.  */
static NO_ABS_PATHS_FLAG: AtomicBool = AtomicBool::new(false);

/* File position of last header read.  Only used during -A to determine
   where the old TRAILER!!! record started.  */
static LAST_HEADER_START: std::sync::Mutex<i64> = std::sync::Mutex::new(0);

/* With -i; if true, copy only files that match any of the given patterns;
   if false, copy only files that do not match any of the patterns. (-f) */
static COPY_MATCHING_FILES: AtomicBool = AtomicBool::new(true);

/* With -itv; if true, list numeric uid and gid instead of translating them
   into names.  */
static NUMERIC_UID: AtomicBool = AtomicBool::new(false);

/* Name of file containing additional patterns (-E).  */
static PATTERN_FILE_NAME: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/* Message to print when end of medium is reached (-M).  */
static NEW_MEDIA_MESSAGE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/* With -M with %d, message to print when end of medium is reached.  */
static NEW_MEDIA_MESSAGE_WITH_NUMBER: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static NEW_MEDIA_MESSAGE_AFTER_NUMBER: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/* File descriptor containing the archive.  */
static ARCHIVE_DES: std::sync::Mutex<Option<std::fs::File>> = std::sync::Mutex::new(None);

/* Name of file containing the archive, if known; NULL if stdin/out.  */
static ARCHIVE_NAME: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/* Name of the remote shell command, if known; NULL otherwise.  */
static RSH_COMMAND_OPTION: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/* CRC checksum.  */
static CRC: AtomicU32 = AtomicU32::new(0);

/* Input and output buffers.  */
static INPUT_BUFFER: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
static OUTPUT_BUFFER: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);

/* The size of the input buffer.  */
static INPUT_BUFFER_SIZE: AtomicUsize = AtomicUsize::new(0);

/* Current locations in `input_buffer' and `output_buffer'.  */
static IN_BUFF: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
static OUT_BUFF: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);

/* Current number of bytes stored at `input_buff' and `output_buff'.  */
static INPUT_SIZE: AtomicUsize = AtomicUsize::new(0);
static OUTPUT_SIZE: AtomicUsize = AtomicUsize::new(0);

static INPUT_BYTES: std::sync::Mutex<i64> = std::sync::Mutex::new(0);
static OUTPUT_BYTES: std::sync::Mutex<i64> = std::sync::Mutex::new(0);

/* Saving of argument values for later reference.  */
static DIRECTORY_NAME: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);
static SAVE_PATTERNS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static NUM_PATTERNS: AtomicUsize = AtomicUsize::new(0);

/* Character that terminates file names read from stdin.  */
static NAME_END: std::sync::Mutex<char> = std::sync::Mutex::new('\n');

/* true if input (cpio -i) or output (cpio -o) is a device node.  */
static INPUT_IS_SPECIAL: AtomicBool = AtomicBool::new(false);
static OUTPUT_IS_SPECIAL: AtomicBool = AtomicBool::new(false);

/* true if lseek works on the input.  */
static INPUT_IS_SEEKABLE: AtomicBool = AtomicBool::new(false);

/* true if lseek works on the output.  */
static OUTPUT_IS_SEEKABLE: AtomicBool = AtomicBool::new(false);

/* Print extra warning messages */
static WARN_OPTION: AtomicUsize = AtomicUsize::new(0);

/* Extract to standard output? */
static TO_STDOUT_OPTION: AtomicBool = AtomicBool::new(false);

/* Which copy operation to perform. (-i, -o, -p) */
static COPY_FUNCTION: std::sync::Mutex<Option<fn()>>> = std::sync::Mutex::new(None);

static CHANGE_DIRECTORY_OPTION: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

static RENUMBER_INODES_OPTION: AtomicBool = AtomicBool::new(false);
static IGNORE_DEVNO_OPTION: AtomicBool = AtomicBool::new(false);
static IGNORE_DIRNLINK_OPTION: AtomicBool = AtomicBool::new(false);