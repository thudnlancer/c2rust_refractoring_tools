use std::io::{self, Read, Write, Seek, SeekFrom};
use std::sync::Arc;

type MtOff = i64;
type SizeT = usize;
type SSizeT = i64;

trait StreamOps {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<SSizeT>;
    fn write(&mut self, buf: &[u8]) -> io::Result<SSizeT>;
    fn pread(&mut self, buf: &mut [u8], offset: MtOff) -> io::Result<SSizeT>;
    fn pwrite(&mut self, buf: &[u8], offset: MtOff) -> io::Result<SSizeT>;
    fn flush(&mut self) -> io::Result<()>;
}

struct Stream {
    ops: Arc<dyn StreamOps + Send + Sync>,
}

impl Stream {
    fn force_pio<F>(
        &mut self,
        mut buf: &mut [u8],
        mut start: MtOff,
        io_op: F,
    ) -> io::Result<SSizeT>
    where
        F: Fn(&mut dyn StreamOps, &mut [u8], MtOff) -> io::Result<SSizeT>,
    {
        let mut done = 0;
        let mut ops = Arc::get_mut(&mut self.ops).expect("Failed to get mutable reference");
        
        while !buf.is_empty() {
            let ret = io_op(&mut *ops, buf, start)?;
            if ret <= 0 {
                if done > 0 {
                    return Ok(done);
                }
                return Ok(ret);
            }

            let ret = ret as SizeT;
            assert!(ret <= buf.len(), "ret value exceeds buffer length");

            start += ret as MtOff;
            done += ret as SSizeT;
            buf = &mut buf[ret..];
        }

        Ok(done)
    }

    fn force_write(&mut self, buf: &mut [u8]) -> io::Result<SSizeT> {
        self.force_pio(buf, 0, |ops, buf, _| ops.write(buf))
    }

    fn force_pwrite(&mut self, buf: &mut [u8], start: MtOff) -> io::Result<SSizeT> {
        self.force_pio(buf, start, |ops, buf, offset| ops.pwrite(buf, offset))
    }

    fn force_pread(&mut self, buf: &mut [u8], start: MtOff) -> io::Result<SSizeT> {
        self.force_pio(buf, start, |ops, buf, offset| ops.pread(buf, offset))
    }
}

// Example implementation of StreamOps
struct FileStream {
    file: std::fs::File,
}

impl StreamOps for FileStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<SSizeT> {
        self.file.read(buf).map(|n| n as SSizeT)
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<SSizeT> {
        self.file.write(buf).map(|n| n as SSizeT)
    }

    fn pread(&mut self, buf: &mut [u8], offset: MtOff) -> io::Result<SSizeT> {
        use std::os::unix::fs::FileExt;
        self.file.read_at(buf, offset as u64).map(|n| n as SSizeT)
    }

    fn pwrite(&mut self, buf: &[u8], offset: MtOff) -> io::Result<SSizeT> {
        use std::os::unix::fs::FileExt;
        self.file.write_at(buf, offset as u64).map(|n| n as SSizeT)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}