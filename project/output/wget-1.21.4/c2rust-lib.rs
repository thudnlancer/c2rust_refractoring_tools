#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]
#[macro_use]
extern crate c2rust_bitfields;
#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod lib {
    pub mod af_alg;
    pub mod asnprintf;
    pub mod base32;
    pub mod basename;
    pub mod basename_lgpl;
    pub mod binary_io;
    pub mod bitrotate;
    pub mod btowc;
    pub mod c_ctype;
    pub mod c_strcasecmp;
    pub mod c_strcasestr;
    pub mod c_strncasecmp;
    pub mod canonicalize;
    pub mod chdir_long;
    pub mod cloexec;
    pub mod concat_filename;
    pub mod dirname;
    pub mod dirname_lgpl;
    pub mod dup_safer;
    pub mod dup_safer_flag;
    pub mod exitfail;
    pub mod fatal_signal;
    pub mod fcntl;
    pub mod fd_hook;
    pub mod fd_safer;
    pub mod fd_safer_flag;
    pub mod fflush;
    pub mod file_set;
    pub mod filenamecat_lgpl;
    pub mod findprog_in;
    pub mod fopen;
    pub mod fpurge;
    pub mod freading;
    pub mod free;
    pub mod fseek;
    pub mod fseeko;
    pub mod getprogname;
    pub mod gettime;
    pub mod glthread {
        pub mod lock;
        pub mod threadlib;
    }
    pub mod hard_locale;
    pub mod hash;
    pub mod hash_pjw;
    pub mod hash_triple_simple;
    pub mod ialloc;
    pub mod ioctl;
    pub mod localcharset;
    pub mod malloc {
        pub mod dynarray_at_failure;
        pub mod dynarray_emplace_enlarge;
        pub mod dynarray_finalize;
        pub mod dynarray_resize;
        pub mod dynarray_resize_clear;
        pub mod scratch_buffer_grow;
        pub mod scratch_buffer_grow_preserve;
        pub mod scratch_buffer_set_array_size;
    }
    pub mod malloca;
    pub mod mbchar;
    pub mod mbiter;
    pub mod mbrtowc;
    pub mod mbsrtowcs;
    pub mod mbsrtowcs_state;
    pub mod md2;
    pub mod md2_stream;
    pub mod md4;
    pub mod md4_stream;
    pub mod md5;
    pub mod md5_stream;
    pub mod mktime;
    pub mod nanosleep;
    pub mod openat_die;
    pub mod openat_proc;
    pub mod pipe2;
    pub mod pipe2_safer;
    pub mod pipe_safer;
    pub mod printf_args;
    pub mod printf_parse;
    pub mod quotearg;
    pub mod r#u64;
    pub mod regex;
    pub mod save_cwd;
    pub mod setlocale_null;
    pub mod sha1;
    pub mod sha1_stream;
    pub mod sha256;
    pub mod sha256_stream;
    pub mod sha512;
    pub mod sha512_stream;
    pub mod sig_handler;
    pub mod sockets;
    pub mod spawn;
    pub mod spawn_faction_addchdir;
    pub mod spawn_faction_addclose;
    pub mod spawn_faction_adddup2;
    pub mod spawn_faction_addopen;
    pub mod spawn_faction_destroy;
    pub mod spawn_faction_init;
    pub mod spawn_pipe;
    pub mod spawnattr_destroy;
    pub mod spawnattr_init;
    pub mod spawnattr_setflags;
    pub mod spawnattr_setpgroup;
    pub mod spawnattr_setsigmask;
    pub mod spawni;
    pub mod spawnp;
    pub mod stat_time;
    pub mod strerror_r;
    pub mod stripslash;
    pub mod strnlen1;
    pub mod strtol;
    pub mod strtoll;
    pub mod sys_socket;
    pub mod tempname;
    pub mod timegm;
    pub mod timespec;
    pub mod tmpdir;
    pub mod unicase {
        pub mod cased;
        pub mod empty_prefix_context;
        pub mod empty_suffix_context;
        pub mod ignorable;
        pub mod special_casing;
        pub mod tolower;
        pub mod u8_casemap;
        pub mod u8_tolower;
    }
    pub mod unictype {
        pub mod combiningclass;
        pub mod pr_soft_dotted;
    }
    pub mod uninorm {
        pub mod decompose_internal;
        pub mod u8_normalize;
    }
    pub mod unistd;
    pub mod unistr {
        pub mod u8_cpy;
        pub mod u8_mbtouc_unsafe;
        pub mod u8_mbtouc_unsafe_aux;
        pub mod u8_strlen;
        pub mod u8_uctomb;
        pub mod u8_uctomb_aux;
    }
    pub mod uniwidth {
        pub mod width;
    }
    pub mod utimens;
    pub mod vasnprintf;
    pub mod wait_process;
    pub mod wctype_h;
    pub mod xalloc_die;
    pub mod xmalloc;
    pub mod xmemdup0;
    pub mod xsize;
    pub mod xstrndup;
}
pub mod src {
    pub mod build_info;
    pub mod connect;
    pub mod convert;
    pub mod cookies;
    pub mod css_;
    pub mod css_url;
    pub mod exits;
    pub mod ftp;
    pub mod ftp_basic;
    pub mod ftp_ls;
    pub mod ftp_opie;
    pub mod gnutls;
    pub mod hash;
    pub mod host;
    pub mod hsts;
    pub mod html_parse;
    pub mod html_url;
    pub mod http;
    pub mod http_ntlm;
    pub mod init;
    pub mod iri;
    pub mod log;
    pub mod main;
    pub mod netrc;
    pub mod progress;
    pub mod ptimer;
    pub mod recur;
    pub mod res;
    pub mod retr;
    pub mod spider;
    pub mod url;
    pub mod utils;
    pub mod version;
    pub mod warc;
    pub mod xattr;
}