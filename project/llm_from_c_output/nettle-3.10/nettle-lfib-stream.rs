/* lfib-stream.rs

   Generates a pseudorandom stream, using the Knuth lfib
   (non-cryptographic) pseudorandom generator.

   Copyright (C) 2003 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

use knuth_lfib::KnuthLfibCtx;
use std::env;
use std::io::{self, Write};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

const BUFSIZE: usize = 500;

fn usage() {
    eprintln!("Usage: lfib-stream [SEED]");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let seed = match args.len() {
        1 => SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .as_secs() as u32,
        2 => args[1]
            .parse()
            .map_err(|_| {
                usage();
                io::Error::new(io::ErrorKind::InvalidInput, "Invalid seed value")
            })?,
        _ => {
            usage();
            process::exit(1);
        }
    };

    let mut ctx = KnuthLfibCtx::new(seed);

    loop {
        let mut buffer = [0u8; BUFSIZE];
        ctx.random(&mut buffer);

        io::stdout().write_all(&buffer)?;
        io::stdout().flush()?;
    }
}