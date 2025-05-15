/*!
Compute message digests using the Linux kernel crypto API.

This module provides functions for computing message digests using the Linux kernel
crypto API when available. The kernel API gives access to specialized crypto
instructions or devices.

For a more complete set of facilities that use the Linux kernel crypto API,
consider using libkcapi.
*/

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::ptr;
use std::slice;

use libc::{
    accept4, bind, close, fstat, lseek, read, send, sendfile, socket, sockaddr, sockaddr_alg,
    socklen_t, AF_ALG, EAFNOSUPPORT, EINVAL, EIO, MSG_MORE, SEEK_CUR, SEEK_SET, SOCK_CLOEXEC,
    SOCK_SEQPACKET, S_ISREG,
};

const BLOCKSIZE: usize = 32768;

/// Supported hash algorithms and their output lengths.
pub enum HashAlgorithm {
    Md5,    // 16 bytes
    Sha1,   // 20 bytes
    Sha224, // 28 bytes
    Sha256, // 32 bytes
    Sha384, // 48 bytes
    Sha512, // 64 bytes
}

impl HashAlgorithm {
    fn as_str(&self) -> &'static str {
        match self {
            HashAlgorithm::Md5 => "md5",
            HashAlgorithm::Sha1 => "sha1",
            HashAlgorithm::Sha224 => "sha224",
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha384 => "sha384",
            HashAlgorithm::Sha512 => "sha512",
        }
    }

    fn output_len(&self) -> usize {
        match self {
            HashAlgorithm::Md5 => 16,
            HashAlgorithm::Sha1 => 20,
            HashAlgorithm::Sha224 => 28,
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha384 => 48,
            HashAlgorithm::Sha512 => 64,
        }
    }
}

/// Error type for cryptographic operations.
#[derive(Debug)]
pub enum CryptoError {
    Unsupported,
    InvalidInput,
    IoError(io::Error),
    KernelError(i32),
}

impl From<io::Error> for CryptoError {
    fn from(err: io::Error) -> Self {
        CryptoError::IoError(err)
    }
}

/// Create a socket for the given algorithm.
fn alg_socket(alg: &str) -> Result<RawFd, CryptoError> {
    let mut salg = sockaddr_alg {
        salg_family: AF_ALG as u16,
        salg_type: [0; 14],
        salg_name: [0; 64],
        salg_feat: 0,
        salg_mask: 0,
        salg_pid: 0,
    };

    // SAFETY: We're writing to fixed-size arrays with proper bounds checking
    unsafe {
        ptr::copy_nonoverlapping(
            b"hash\0".as_ptr(),
            salg.salg_type.as_mut_ptr(),
            "hash".len().min(13),
        );
    }

    if alg.len() >= 64 {
        return Err(CryptoError::InvalidInput);
    }

    // SAFETY: We've checked the length and are copying into a properly sized buffer
    unsafe {
        ptr::copy_nonoverlapping(
            alg.as_ptr(),
            salg.salg_name.as_mut_ptr(),
            alg.len().min(63),
        );
    }

    // SAFETY: socket is a standard syscall
    let cfd = unsafe { socket(AF_ALG, SOCK_SEQPACKET | SOCK_CLOEXEC, 0) };
    if cfd < 0 {
        return Err(CryptoError::Unsupported);
    }

    // SAFETY: bind is a standard syscall with properly initialized struct
    let bind_res = unsafe { bind(cfd, &salg as *const _ as *const sockaddr, std::mem::size_of::<sockaddr_alg>() as socklen_t) };

    let ofd = if bind_res == 0 {
        // SAFETY: accept4 is a standard syscall
        unsafe { accept4(cfd, ptr::null_mut(), ptr::null_mut(), SOCK_CLOEXEC) }
    } else {
        -1
    };

    // SAFETY: close is a standard syscall
    unsafe { close(cfd) };

    if ofd < 0 {
        Err(CryptoError::Unsupported)
    } else {
        Ok(ofd)
    }
}

/// Compute message digest of a memory buffer.
pub fn afalg_buffer(buffer: &[u8], alg: HashAlgorithm) -> Result<Vec<u8>, CryptoError> {
    // On Linux < 4.9, empty buffers produce incorrect results
    if buffer.is_empty() {
        return Err(CryptoError::Unsupported);
    }

    let ofd = alg_socket(alg.as_str())?;
    let mut result = vec![0u8; alg.output_len()];

    let mut remaining = buffer;
    let mut success = false;

    while !remaining.is_empty() {
        let chunk_size = remaining.len().min(BLOCKSIZE);
        let chunk = &remaining[..chunk_size];

        // SAFETY: send is a standard syscall with properly aligned buffer
        let sent = unsafe { send(ofd, chunk.as_ptr() as *const _, chunk_size, MSG_MORE) };
        if sent != chunk_size as isize {
            break;
        }

        remaining = &remaining[chunk_size..];
        if remaining.is_empty() {
            // SAFETY: read is a standard syscall with properly sized buffer
            let read_bytes = unsafe { read(ofd, result.as_mut_ptr() as *mut _, alg.output_len()) };
            if read_bytes == alg.output_len() as isize {
                success = true;
            }
            break;
        }
    }

    // SAFETY: close is a standard syscall
    unsafe { close(ofd) };

    if success {
        Ok(result)
    } else {
        Err(CryptoError::Unsupported)
    }
}

/// Compute message digest of data read from a stream.
pub fn afalg_stream(stream: &mut File, alg: HashAlgorithm) -> Result<Vec<u8>, CryptoError> {
    let ofd = alg_socket(alg.as_str())?;
    let mut result = vec![0u8; alg.output_len()];
    let fd = stream.as_raw_fd();

    // Try to use sendfile for efficiency if possible
    let mut stat = unsafe {
        let mut stat = std::mem::zeroed();
        if fstat(fd, &mut stat) != 0 {
            std::mem::zeroed()
        } else {
            stat
        }
    };

    let pos = stream.stream_position()?;
    let is_regular = unsafe { S_ISREG(stat.st_mode) } != 0;

    if is_regular && pos < stat.st_size {
        // SAFETY: sendfile is a standard syscall
        let mut offset = pos;
        let nbytes = stat.st_size - pos;
        let sent = unsafe { sendfile(ofd, fd, &mut offset, nbytes as usize) };

        if sent == nbytes {
            // SAFETY: read is a standard syscall
            let read_bytes = unsafe { read(ofd, result.as_mut_ptr() as *mut _, alg.output_len()) };
            if read_bytes == alg.output_len() as isize {
                // Reset the file position
                stream.seek(SeekFrom::Start(offset))?;
                // SAFETY: close is a standard syscall
                unsafe { close(ofd) };
                return Ok(result);
            }
        }
    }

    // Fall back to read-write loop
    let mut nseek: i64 = 0;
    let mut success = false;

    loop {
        let mut buf = [0u8; BLOCKSIZE];
        let blocksize = if nseek == 0 { 1 } else { BLOCKSIZE };
        let size = stream.read(&mut buf[..blocksize])?;

        if size == 0 {
            success = nseek != 0;
            break;
        }

        nseek -= size as i64;

        // SAFETY: send is a standard syscall
        let sent = unsafe { send(ofd, buf.as_ptr() as *const _, size, MSG_MORE) };
        if sent != size as isize {
            if nseek == -1 {
                // Push back one byte if possible
                let mut pushback_buf = [0u8; 1];
                pushback_buf[0] = buf[0];
                stream.seek(SeekFrom::Current(-1))?;
            } else if nseek != 0 {
                stream.seek(SeekFrom::Current(nseek))?;
            }
            break;
        }

        if stream.stream_position()? >= stat.st_size as u64 {
            success = true;
            break;
        }
    }

    if success {
        // SAFETY: read is a standard syscall
        let read_bytes = unsafe { read(ofd, result.as_mut_ptr() as *mut _, alg.output_len()) };
        if read_bytes != alg.output_len() as isize {
            if nseek != 0 {
                stream.seek(SeekFrom::Current(nseek))?;
            }
            success = false;
        }
    }

    // SAFETY: close is a standard syscall
    unsafe { close(ofd) };

    if success {
        Ok(result)
    } else {
        Err(CryptoError::Unsupported)
    }
}