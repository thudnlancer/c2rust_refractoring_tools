use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::mem;
use std::path::Path;
use std::ptr;
use std::slice;
use std::os::unix::io::{AsRawFd, RawFd};
use libc::{self, c_char, c_int, off_t};
use nix::sys::stat::fstat;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, lseek, Whence};
use tar_sys::{BLOCKSIZE, blocking_factor, record_size, record_start, record_end, current_block, recent_long_name, recent_long_link, records_read, records_written, records_skipped, archive_name_array, archive, current_header, current_stat_info, ignore_zeros_option, write_archive_to_stdout, acting_as_filter, new_record, new_blocks};
use tar_sys::header::{Header, read_header, HeaderStatus};
use tar_sys::name::{Name, name_scan, name_gather, names_notfound, ISFOUND};
use tar_sys::rmt::{rmtioctl, rmtlseek, MTIOCTOP, mtop, MTBSR, MTFSR};
use tar_sys::xheader::xheader_decode;
use tar_sys::utils::{xmalloc, flush_write, flush_archive, flush_read, write_eot, skip_member, set_next_block_after, seek_error_details, truncate_warn};

fn move_archive(count: off_t) -> io::Result<()> {
    if count == 0 {
        return Ok(());
    }

    #[cfg(MTIOCTOP)]
    {
        let operation = if count < 0 {
            mtop {
                mt_op: MTBSR,
                mt_count: -count,
            }
        } else {
            mtop {
                mt_op: MTFSR,
                mt_count: count,
            }
        };

        if rmtioctl(archive, MTIOCTOP, &operation as *const _ as *const c_char).is_ok() {
            return Ok(());
        }

        if io::Error::last_os_error().raw_os_error() == Some(libc::EIO) {
            if rmtioctl(archive, MTIOCTOP, &operation as *const _ as *const c_char).is_ok() {
                return Ok(());
            }
        }
    }

    let position0 = rmtlseek(archive, 0, Whence::SeekCur)?;
    let increment = record_size as off_t * count;
    let position = position0 + increment;

    if increment / count != record_size as off_t
        || (position < position0) != (increment < 0)
    {
        let position = if position < 0 { 0 } else { position };
        if rmtlseek(archive, position, Whence::SeekSet)? != position {
            seek_error_details(&archive_name_array[0], position);
        }
    }

    Ok(())
}

fn write_record(move_back_flag: bool) -> io::Result<()> {
    let save_record = record_start;
    record_start = new_record;

    if acting_as_filter {
        archive = libc::STDOUT_FILENO;
        flush_write()?;
        archive = libc::STDIN_FILENO;
    } else {
        move_archive((records_written + records_skipped) - records_read)?;
        flush_write()?;
    }

    record_start = save_record;

    if move_back_flag && !acting_as_filter {
        move_archive(records_read - (records_written + records_skipped))?;
    }

    new_blocks = 0;
    Ok(())
}

fn write_recent_blocks(h: *const Header, blocks: usize) -> io::Result<()> {
    for i in 0..blocks {
        unsafe {
            ptr::copy_nonoverlapping(h.add(i), new_record.add(new_blocks), 1);
        }
        new_blocks += 1;
        if new_blocks == blocking_factor {
            write_record(true)?;
        }
    }
    Ok(())
}

fn write_recent_bytes(data: *const u8, bytes: usize) -> io::Result<()> {
    let blocks = bytes / BLOCKSIZE;
    let rest = bytes - blocks * BLOCKSIZE;

    write_recent_blocks(data as *const Header, blocks)?;
    unsafe {
        ptr::copy_nonoverlapping(data.add(blocks * BLOCKSIZE), new_record[new_blocks].buffer.as_mut_ptr(), rest);
        ptr::write_bytes(new_record[new_blocks].buffer.as_mut_ptr().add(rest), 0, BLOCKSIZE - rest);
    }
    new_blocks += 1;
    if new_blocks == blocking_factor {
        write_record(true)?;
    }
    Ok(())
}

fn flush_file() -> io::Result<()> {
    set_next_block_after(current_header);
    let mut blocks_to_skip = (current_stat_info.stat.st_size + BLOCKSIZE as u64 - 1) / BLOCKSIZE as u64;

    while (record_end - current_block) as u64 <= blocks_to_skip {
        blocks_to_skip -= (record_end - current_block) as u64;
        flush_archive()?;
    }
    current_block += blocks_to_skip as usize;
    Ok(())
}

pub fn delete_archive_members() -> io::Result<()> {
    let mut logical_status = HeaderStatus::StillUnread;
    let mut previous_status = HeaderStatus::StillUnread;

    name_gather();
    open_archive(ACCESS_UPDATE)?;
    acting_as_filter = archive_name_array[0] == "-";

    loop {
        let status = read_header(&mut current_header, &mut current_stat_info, read_header_x_raw);

        match status {
            HeaderStatus::StillUnread => unreachable!(),
            HeaderStatus::Success => {
                if let Some(name) = name_scan(current_stat_info.file_name) {
                    name.found_count += 1;
                    if !ISFOUND(name) {
                        skip_member()?;
                        continue;
                    }
                } else {
                    skip_member()?;
                    continue;
                }
                logical_status = status;
                break;
            }
            HeaderStatus::SuccessExtended => {
                logical_status = status;
                break;
            }
            HeaderStatus::ZeroBlock => {
                if ignore_zeros_option {
                    set_next_block_after(current_header);
                    continue;
                }
                logical_status = HeaderStatus::EndOfFile;
                break;
            }
            HeaderStatus::EndOfFile => {
                logical_status = HeaderStatus::EndOfFile;
                break;
            }
            HeaderStatus::Failure => {
                set_next_block_after(current_header);
                match previous_status {
                    HeaderStatus::StillUnread => {
                        warn!("This does not look like a tar archive");
                    }
                    HeaderStatus::Success | HeaderStatus::SuccessExtended | HeaderStatus::ZeroBlock => {
                        error!("Skipping to next header");
                    }
                    HeaderStatus::Failure => {}
                    HeaderStatus::EndOfFile => unreachable!(),
                }
            }
        }

        previous_status = status;
    }

    records_skipped = records_read - 1;
    new_record = xmalloc(record_size);

    if logical_status == HeaderStatus::Success || logical_status == HeaderStatus::SuccessExtended {
        write_archive_to_stdout = false;

        new_blocks = (current_block - record_start) as usize;
        if new_blocks > 0 {
            unsafe {
                ptr::copy_nonoverlapping(record_start, new_record, new_blocks * BLOCKSIZE);
            }
        }

        if logical_status == HeaderStatus::Success {
            logical_status = HeaderStatus::StillUnread;
            flush_file()?;
        }

        while logical_status != HeaderStatus::EndOfFile {
            if current_block == record_end {
                flush_archive()?;
            }

            let status = read_header(&mut current_header, &mut current_stat_info, read_header_auto);

            match status {
                HeaderStatus::StillUnread | HeaderStatus::SuccessExtended => unreachable!(),
                HeaderStatus::Success => {
                    xheader_decode(&mut current_stat_info);

                    if let Some(name) = name_scan(current_stat_info.file_name) {
                        name.found_count += 1;
                        if ISFOUND(name) {
                            flush_file()?;
                            continue;
                        }
                    }

                    if current_stat_info.xhdr.size > 0 {
                        write_recent_bytes(current_stat_info.xhdr.buffer.as_ptr(), current_stat_info.xhdr.size)?;
                    } else {
                        write_recent_blocks(recent_long_name, recent_long_name_blocks)?;
                        write_recent_blocks(recent_long_link, recent_long_link_blocks)?;
                    }

                    unsafe {
                        *new_record.add(new_blocks) = *current_header;
                    }
                    new_blocks += 1;
                    let mut blocks_to_keep = (current_stat_info.stat.st_size + BLOCKSIZE as u64 - 1) / BLOCKSIZE as u64;
                    set_next_block_after(current_header);
                    if new_blocks == blocking_factor {
                        write_record(true)?;
                    }

                    let mut kept_blocks_in_record = (record_end - current_block) as u64;
                    if kept_blocks_in_record > blocks_to_keep {
                        kept_blocks_in_record = blocks_to_keep;
                    }

                    while blocks_to_keep > 0 {
                        if current_block == record_end {
                            flush_read()?;
                            current_block = record_start;
                            kept_blocks_in_record = blocking_factor as u64;
                            if kept_blocks_in_record > blocks_to_keep {
                                kept_blocks_in_record = blocks_to_keep;
                            }
                        }

                        let mut count = kept_blocks_in_record as usize;
                        if blocking_factor - new_blocks < count {
                            count = blocking_factor - new_blocks;
                        }

                        if count == 0 {
                            unreachable!();
                        }

                        unsafe {
                            ptr::copy_nonoverlapping(current_block, new_record.add(new_blocks), count * BLOCKSIZE);
                        }
                        new_blocks += count;
                        current_block += count;
                        blocks_to_keep -= count as u64;
                        kept_blocks_in_record -= count as u64;

                        if new_blocks == blocking_factor {
                            write_record(true)?;
                        }
                    }
                }
                HeaderStatus::ZeroBlock => {
                    if ignore_zeros_option {
                        set_next_block_after(current_header);
                    } else {
                        logical_status = HeaderStatus::EndOfFile;
                    }
                }
                HeaderStatus::EndOfFile => {
                    logical_status = HeaderStatus::EndOfFile;
                }
                HeaderStatus::Failure => {
                    error!("Deleting non-header from archive");
                    set_next_block_after(current_header);
                }
            }

            tar_stat_destroy(&mut current_stat_info);
        }

        if logical_status == HeaderStatus::EndOfFile {
            let mut total_zero_blocks = 0;

            loop {
                let zero_blocks = blocking_factor - new_blocks;
                unsafe {
                    ptr::write_bytes(new_record.add(new_blocks), 0, BLOCKSIZE * zero_blocks);
                }
                total_zero_blocks += zero_blocks;
                write_record(total_zero_blocks < 2)?;
                if total_zero_blocks >= 2 {
                    break;
                }
            }
        }

        if !acting_as_filter && !_isrmt(archive) {
            if sys_truncate(archive).is_err() {
                truncate_warn(&archive_name_array[0]);
            }
        }
    }

    unsafe {
        libc::free(new_record as *mut _);
    }

    close_archive()?;
    names_notfound();
    Ok(())
}