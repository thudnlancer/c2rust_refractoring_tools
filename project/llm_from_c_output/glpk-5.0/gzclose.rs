// gzclose.rs -- zlib gzclose() function
// Copyright (C) 2004, 2010 Mark Adler
// For conditions of distribution and use, see copyright notice in zlib.h

use crate::gzguts::*;

/// gzclose() is in a separate file so that it is linked in only if it is used.
/// That way the other gzclose functions can be used instead to avoid linking in
/// unneeded compression or decompression routines.
pub fn gzclose(file: Option<gzFile>) -> Result<i32, i32> {
    #[cfg(not(feature = "no_gzcompress"))]
    {
        let state = match file {
            Some(f) => f,
            None => return Err(Z_STREAM_ERROR),
        };

        if state.mode == GZ_READ {
            gzclose_r(Some(state))
        } else {
            gzclose_w(Some(state))
        }
    }

    #[cfg(feature = "no_gzcompress"))]
    {
        gzclose_r(file)
    }
}