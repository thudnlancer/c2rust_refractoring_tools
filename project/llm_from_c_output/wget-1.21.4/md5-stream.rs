use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use md5::{Md5, Digest};

const BLOCKSIZE: usize = 32768;

/// Compute MD5 message digest for bytes read from a file.
/// The resulting message digest will be written into the 16-byte array.
pub fn md5_stream<P: AsRef<Path>>(path: P, resblock: &mut [u8; 16]) -> io::Result<()> {
    // Verify BLOCKSIZE is valid
    if BLOCKSIZE % 64 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid BLOCKSIZE",
        ));
    }

    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; BLOCKSIZE + 72];
    let mut hasher = Md5::new();
    let mut sum = 0;

    // Iterate over full file contents
    loop {
        // Read block, handling partial reads
        sum = 0;
        loop {
            if sum == BLOCKSIZE {
                break;
            }

            match file.read(&mut buffer[sum..BLOCKSIZE])? {
                0 => {
                    if sum > 0 {
                        hasher.update(&buffer[..sum]);
                    }
                    let result = hasher.finalize();
                    resblock.copy_from_slice(&result);
                    return Ok(());
                }
                n => {
                    sum += n;
                }
            }
        }

        // Process full block
        hasher.update(&buffer[..BLOCKSIZE]);
    }
}

/*
 * Hey Emacs!
 * Local Variables:
 * coding: utf-8
 * End:
 */