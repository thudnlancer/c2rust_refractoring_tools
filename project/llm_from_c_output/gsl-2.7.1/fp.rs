// Rust equivalent using conditional compilation

#[cfg(all(target_arch = "sparc", target_vendor = "unknown"))]
mod fp {
    include!("fp-gnusparc.rs");
}

#[cfg(all(target_arch = "m68k", target_vendor = "unknown"))]
mod fp {
    include!("fp-gnum68k.rs");
}

#[cfg(all(target_arch = "powerpc", target_vendor = "unknown"))]
mod fp {
    include!("fp-gnuppc.rs");
}

#[cfg(all(target_arch = "x86", target_vendor = "unknown"))]
mod fp {
    include!("fp-gnux86.rs");
}

#[cfg(all(target_os = "hpux", target_version = "11"))]
mod fp {
    include!("fp-hpux11.rs");
}

#[cfg(target_os = "hpux")]
mod fp {
    include!("fp-hpux.rs");
}

#[cfg(target_os = "solaris")]
mod fp {
    include!("fp-solaris.rs");
}

#[cfg(target_os = "irix")]
mod fp {
    include!("fp-irix.rs");
}

#[cfg(target_os = "aix")]
mod fp {
    include!("fp-aix.rs");
}

#[cfg(target_os = "tru64")]
mod fp {
    include!("fp-tru64.rs");
}

#[cfg(target_os = "freebsd")]
mod fp {
    include!("fp-freebsd.rs");
}

#[cfg(target_os = "netbsd")]
mod fp {
    include!("fp-netbsd.rs");
}

#[cfg(target_os = "openbsd")]
mod fp {
    include!("fp-openbsd.rs");
}

#[cfg(all(target_os = "macos", target_arch = "x86"))]
mod fp {
    include!("fp-darwin86.rs");
}

#[cfg(all(target_os = "macos", not(target_arch = "x86")))]
mod fp {
    include!("fp-darwin.rs");
}

#[cfg(any(
    feature = "feenableexcept",
    feature = "fesettrapenable"
))]
mod fp {
    include!("fp-gnuc99.rs");
}

#[cfg(not(any(
    target_arch = "sparc",
    target_arch = "m68k",
    target_arch = "powerpc",
    target_arch = "x86",
    target_os = "hpux",
    target_os = "solaris",
    target_os = "irix",
    target_os = "aix",
    target_os = "tru64",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "macos",
    feature = "feenableexcept",
    feature = "fesettrapenable"
)))]
mod fp {
    include!("fp-unknown.rs");
}