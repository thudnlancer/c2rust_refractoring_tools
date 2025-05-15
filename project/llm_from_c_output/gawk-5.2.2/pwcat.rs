/*
 * pwcat.rs
 *
 * Generate a printable version of the password database.
 */
/*
 * Arnold Robbins, arnold@skeeve.com, May 1993
 * Public Domain
 * December 2010, move to ANSI C definition for main().
 * Translated to Rust while maintaining functionality and safety.
 */

use std::io::{self, Write};
use users::{all_users, User};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for user in all_users() {
        let user: User = user;
        writeln!(
            handle,
            "{}:{}:{}:{}:{}:{}:{}",
            user.name().to_string_lossy(),
            user.password().to_string_lossy(),
            user.uid(),
            user.primary_group_id(),
            user.gecos().to_string_lossy(),
            user.home_dir().to_string_lossy(),
            user.shell().to_string_lossy()
        )?;
    }

    Ok(())
}