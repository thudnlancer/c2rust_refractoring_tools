use ::libc;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_qrng_type {
    pub name: *const libc::c_char,
    pub max_dimension: libc::c_uint,
    pub state_size: Option::<unsafe extern "C" fn(libc::c_uint) -> size_t>,
    pub init_state: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> libc::c_int,
    >,
    pub get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_uint,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct halton_state_t {
    pub sequence_count: libc::c_uint,
}
static mut halton_type: gsl_qrng_type = unsafe {
    {
        let mut init = gsl_qrng_type {
            name: b"halton\0" as *const u8 as *const libc::c_char,
            max_dimension: 1229 as libc::c_int as libc::c_uint,
            state_size: Some(
                halton_state_size as unsafe extern "C" fn(libc::c_uint) -> size_t,
            ),
            init_state: Some(
                halton_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> libc::c_int,
            ),
            get: Some(
                halton_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                        *mut libc::c_double,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_qrng_halton: *const gsl_qrng_type = unsafe {
    &halton_type as *const gsl_qrng_type
};
static mut prime_numbers: [libc::c_int; 1229] = [
    2 as libc::c_int,
    3 as libc::c_int,
    5 as libc::c_int,
    7 as libc::c_int,
    11 as libc::c_int,
    13 as libc::c_int,
    17 as libc::c_int,
    19 as libc::c_int,
    23 as libc::c_int,
    29 as libc::c_int,
    31 as libc::c_int,
    37 as libc::c_int,
    41 as libc::c_int,
    43 as libc::c_int,
    47 as libc::c_int,
    53 as libc::c_int,
    59 as libc::c_int,
    61 as libc::c_int,
    67 as libc::c_int,
    71 as libc::c_int,
    73 as libc::c_int,
    79 as libc::c_int,
    83 as libc::c_int,
    89 as libc::c_int,
    97 as libc::c_int,
    101 as libc::c_int,
    103 as libc::c_int,
    107 as libc::c_int,
    109 as libc::c_int,
    113 as libc::c_int,
    127 as libc::c_int,
    131 as libc::c_int,
    137 as libc::c_int,
    139 as libc::c_int,
    149 as libc::c_int,
    151 as libc::c_int,
    157 as libc::c_int,
    163 as libc::c_int,
    167 as libc::c_int,
    173 as libc::c_int,
    179 as libc::c_int,
    181 as libc::c_int,
    191 as libc::c_int,
    193 as libc::c_int,
    197 as libc::c_int,
    199 as libc::c_int,
    211 as libc::c_int,
    223 as libc::c_int,
    227 as libc::c_int,
    229 as libc::c_int,
    233 as libc::c_int,
    239 as libc::c_int,
    241 as libc::c_int,
    251 as libc::c_int,
    257 as libc::c_int,
    263 as libc::c_int,
    269 as libc::c_int,
    271 as libc::c_int,
    277 as libc::c_int,
    281 as libc::c_int,
    283 as libc::c_int,
    293 as libc::c_int,
    307 as libc::c_int,
    311 as libc::c_int,
    313 as libc::c_int,
    317 as libc::c_int,
    331 as libc::c_int,
    337 as libc::c_int,
    347 as libc::c_int,
    349 as libc::c_int,
    353 as libc::c_int,
    359 as libc::c_int,
    367 as libc::c_int,
    373 as libc::c_int,
    379 as libc::c_int,
    383 as libc::c_int,
    389 as libc::c_int,
    397 as libc::c_int,
    401 as libc::c_int,
    409 as libc::c_int,
    419 as libc::c_int,
    421 as libc::c_int,
    431 as libc::c_int,
    433 as libc::c_int,
    439 as libc::c_int,
    443 as libc::c_int,
    449 as libc::c_int,
    457 as libc::c_int,
    461 as libc::c_int,
    463 as libc::c_int,
    467 as libc::c_int,
    479 as libc::c_int,
    487 as libc::c_int,
    491 as libc::c_int,
    499 as libc::c_int,
    503 as libc::c_int,
    509 as libc::c_int,
    521 as libc::c_int,
    523 as libc::c_int,
    541 as libc::c_int,
    547 as libc::c_int,
    557 as libc::c_int,
    563 as libc::c_int,
    569 as libc::c_int,
    571 as libc::c_int,
    577 as libc::c_int,
    587 as libc::c_int,
    593 as libc::c_int,
    599 as libc::c_int,
    601 as libc::c_int,
    607 as libc::c_int,
    613 as libc::c_int,
    617 as libc::c_int,
    619 as libc::c_int,
    631 as libc::c_int,
    641 as libc::c_int,
    643 as libc::c_int,
    647 as libc::c_int,
    653 as libc::c_int,
    659 as libc::c_int,
    661 as libc::c_int,
    673 as libc::c_int,
    677 as libc::c_int,
    683 as libc::c_int,
    691 as libc::c_int,
    701 as libc::c_int,
    709 as libc::c_int,
    719 as libc::c_int,
    727 as libc::c_int,
    733 as libc::c_int,
    739 as libc::c_int,
    743 as libc::c_int,
    751 as libc::c_int,
    757 as libc::c_int,
    761 as libc::c_int,
    769 as libc::c_int,
    773 as libc::c_int,
    787 as libc::c_int,
    797 as libc::c_int,
    809 as libc::c_int,
    811 as libc::c_int,
    821 as libc::c_int,
    823 as libc::c_int,
    827 as libc::c_int,
    829 as libc::c_int,
    839 as libc::c_int,
    853 as libc::c_int,
    857 as libc::c_int,
    859 as libc::c_int,
    863 as libc::c_int,
    877 as libc::c_int,
    881 as libc::c_int,
    883 as libc::c_int,
    887 as libc::c_int,
    907 as libc::c_int,
    911 as libc::c_int,
    919 as libc::c_int,
    929 as libc::c_int,
    937 as libc::c_int,
    941 as libc::c_int,
    947 as libc::c_int,
    953 as libc::c_int,
    967 as libc::c_int,
    971 as libc::c_int,
    977 as libc::c_int,
    983 as libc::c_int,
    991 as libc::c_int,
    997 as libc::c_int,
    1009 as libc::c_int,
    1013 as libc::c_int,
    1019 as libc::c_int,
    1021 as libc::c_int,
    1031 as libc::c_int,
    1033 as libc::c_int,
    1039 as libc::c_int,
    1049 as libc::c_int,
    1051 as libc::c_int,
    1061 as libc::c_int,
    1063 as libc::c_int,
    1069 as libc::c_int,
    1087 as libc::c_int,
    1091 as libc::c_int,
    1093 as libc::c_int,
    1097 as libc::c_int,
    1103 as libc::c_int,
    1109 as libc::c_int,
    1117 as libc::c_int,
    1123 as libc::c_int,
    1129 as libc::c_int,
    1151 as libc::c_int,
    1153 as libc::c_int,
    1163 as libc::c_int,
    1171 as libc::c_int,
    1181 as libc::c_int,
    1187 as libc::c_int,
    1193 as libc::c_int,
    1201 as libc::c_int,
    1213 as libc::c_int,
    1217 as libc::c_int,
    1223 as libc::c_int,
    1229 as libc::c_int,
    1231 as libc::c_int,
    1237 as libc::c_int,
    1249 as libc::c_int,
    1259 as libc::c_int,
    1277 as libc::c_int,
    1279 as libc::c_int,
    1283 as libc::c_int,
    1289 as libc::c_int,
    1291 as libc::c_int,
    1297 as libc::c_int,
    1301 as libc::c_int,
    1303 as libc::c_int,
    1307 as libc::c_int,
    1319 as libc::c_int,
    1321 as libc::c_int,
    1327 as libc::c_int,
    1361 as libc::c_int,
    1367 as libc::c_int,
    1373 as libc::c_int,
    1381 as libc::c_int,
    1399 as libc::c_int,
    1409 as libc::c_int,
    1423 as libc::c_int,
    1427 as libc::c_int,
    1429 as libc::c_int,
    1433 as libc::c_int,
    1439 as libc::c_int,
    1447 as libc::c_int,
    1451 as libc::c_int,
    1453 as libc::c_int,
    1459 as libc::c_int,
    1471 as libc::c_int,
    1481 as libc::c_int,
    1483 as libc::c_int,
    1487 as libc::c_int,
    1489 as libc::c_int,
    1493 as libc::c_int,
    1499 as libc::c_int,
    1511 as libc::c_int,
    1523 as libc::c_int,
    1531 as libc::c_int,
    1543 as libc::c_int,
    1549 as libc::c_int,
    1553 as libc::c_int,
    1559 as libc::c_int,
    1567 as libc::c_int,
    1571 as libc::c_int,
    1579 as libc::c_int,
    1583 as libc::c_int,
    1597 as libc::c_int,
    1601 as libc::c_int,
    1607 as libc::c_int,
    1609 as libc::c_int,
    1613 as libc::c_int,
    1619 as libc::c_int,
    1621 as libc::c_int,
    1627 as libc::c_int,
    1637 as libc::c_int,
    1657 as libc::c_int,
    1663 as libc::c_int,
    1667 as libc::c_int,
    1669 as libc::c_int,
    1693 as libc::c_int,
    1697 as libc::c_int,
    1699 as libc::c_int,
    1709 as libc::c_int,
    1721 as libc::c_int,
    1723 as libc::c_int,
    1733 as libc::c_int,
    1741 as libc::c_int,
    1747 as libc::c_int,
    1753 as libc::c_int,
    1759 as libc::c_int,
    1777 as libc::c_int,
    1783 as libc::c_int,
    1787 as libc::c_int,
    1789 as libc::c_int,
    1801 as libc::c_int,
    1811 as libc::c_int,
    1823 as libc::c_int,
    1831 as libc::c_int,
    1847 as libc::c_int,
    1861 as libc::c_int,
    1867 as libc::c_int,
    1871 as libc::c_int,
    1873 as libc::c_int,
    1877 as libc::c_int,
    1879 as libc::c_int,
    1889 as libc::c_int,
    1901 as libc::c_int,
    1907 as libc::c_int,
    1913 as libc::c_int,
    1931 as libc::c_int,
    1933 as libc::c_int,
    1949 as libc::c_int,
    1951 as libc::c_int,
    1973 as libc::c_int,
    1979 as libc::c_int,
    1987 as libc::c_int,
    1993 as libc::c_int,
    1997 as libc::c_int,
    1999 as libc::c_int,
    2003 as libc::c_int,
    2011 as libc::c_int,
    2017 as libc::c_int,
    2027 as libc::c_int,
    2029 as libc::c_int,
    2039 as libc::c_int,
    2053 as libc::c_int,
    2063 as libc::c_int,
    2069 as libc::c_int,
    2081 as libc::c_int,
    2083 as libc::c_int,
    2087 as libc::c_int,
    2089 as libc::c_int,
    2099 as libc::c_int,
    2111 as libc::c_int,
    2113 as libc::c_int,
    2129 as libc::c_int,
    2131 as libc::c_int,
    2137 as libc::c_int,
    2141 as libc::c_int,
    2143 as libc::c_int,
    2153 as libc::c_int,
    2161 as libc::c_int,
    2179 as libc::c_int,
    2203 as libc::c_int,
    2207 as libc::c_int,
    2213 as libc::c_int,
    2221 as libc::c_int,
    2237 as libc::c_int,
    2239 as libc::c_int,
    2243 as libc::c_int,
    2251 as libc::c_int,
    2267 as libc::c_int,
    2269 as libc::c_int,
    2273 as libc::c_int,
    2281 as libc::c_int,
    2287 as libc::c_int,
    2293 as libc::c_int,
    2297 as libc::c_int,
    2309 as libc::c_int,
    2311 as libc::c_int,
    2333 as libc::c_int,
    2339 as libc::c_int,
    2341 as libc::c_int,
    2347 as libc::c_int,
    2351 as libc::c_int,
    2357 as libc::c_int,
    2371 as libc::c_int,
    2377 as libc::c_int,
    2381 as libc::c_int,
    2383 as libc::c_int,
    2389 as libc::c_int,
    2393 as libc::c_int,
    2399 as libc::c_int,
    2411 as libc::c_int,
    2417 as libc::c_int,
    2423 as libc::c_int,
    2437 as libc::c_int,
    2441 as libc::c_int,
    2447 as libc::c_int,
    2459 as libc::c_int,
    2467 as libc::c_int,
    2473 as libc::c_int,
    2477 as libc::c_int,
    2503 as libc::c_int,
    2521 as libc::c_int,
    2531 as libc::c_int,
    2539 as libc::c_int,
    2543 as libc::c_int,
    2549 as libc::c_int,
    2551 as libc::c_int,
    2557 as libc::c_int,
    2579 as libc::c_int,
    2591 as libc::c_int,
    2593 as libc::c_int,
    2609 as libc::c_int,
    2617 as libc::c_int,
    2621 as libc::c_int,
    2633 as libc::c_int,
    2647 as libc::c_int,
    2657 as libc::c_int,
    2659 as libc::c_int,
    2663 as libc::c_int,
    2671 as libc::c_int,
    2677 as libc::c_int,
    2683 as libc::c_int,
    2687 as libc::c_int,
    2689 as libc::c_int,
    2693 as libc::c_int,
    2699 as libc::c_int,
    2707 as libc::c_int,
    2711 as libc::c_int,
    2713 as libc::c_int,
    2719 as libc::c_int,
    2729 as libc::c_int,
    2731 as libc::c_int,
    2741 as libc::c_int,
    2749 as libc::c_int,
    2753 as libc::c_int,
    2767 as libc::c_int,
    2777 as libc::c_int,
    2789 as libc::c_int,
    2791 as libc::c_int,
    2797 as libc::c_int,
    2801 as libc::c_int,
    2803 as libc::c_int,
    2819 as libc::c_int,
    2833 as libc::c_int,
    2837 as libc::c_int,
    2843 as libc::c_int,
    2851 as libc::c_int,
    2857 as libc::c_int,
    2861 as libc::c_int,
    2879 as libc::c_int,
    2887 as libc::c_int,
    2897 as libc::c_int,
    2903 as libc::c_int,
    2909 as libc::c_int,
    2917 as libc::c_int,
    2927 as libc::c_int,
    2939 as libc::c_int,
    2953 as libc::c_int,
    2957 as libc::c_int,
    2963 as libc::c_int,
    2969 as libc::c_int,
    2971 as libc::c_int,
    2999 as libc::c_int,
    3001 as libc::c_int,
    3011 as libc::c_int,
    3019 as libc::c_int,
    3023 as libc::c_int,
    3037 as libc::c_int,
    3041 as libc::c_int,
    3049 as libc::c_int,
    3061 as libc::c_int,
    3067 as libc::c_int,
    3079 as libc::c_int,
    3083 as libc::c_int,
    3089 as libc::c_int,
    3109 as libc::c_int,
    3119 as libc::c_int,
    3121 as libc::c_int,
    3137 as libc::c_int,
    3163 as libc::c_int,
    3167 as libc::c_int,
    3169 as libc::c_int,
    3181 as libc::c_int,
    3187 as libc::c_int,
    3191 as libc::c_int,
    3203 as libc::c_int,
    3209 as libc::c_int,
    3217 as libc::c_int,
    3221 as libc::c_int,
    3229 as libc::c_int,
    3251 as libc::c_int,
    3253 as libc::c_int,
    3257 as libc::c_int,
    3259 as libc::c_int,
    3271 as libc::c_int,
    3299 as libc::c_int,
    3301 as libc::c_int,
    3307 as libc::c_int,
    3313 as libc::c_int,
    3319 as libc::c_int,
    3323 as libc::c_int,
    3329 as libc::c_int,
    3331 as libc::c_int,
    3343 as libc::c_int,
    3347 as libc::c_int,
    3359 as libc::c_int,
    3361 as libc::c_int,
    3371 as libc::c_int,
    3373 as libc::c_int,
    3389 as libc::c_int,
    3391 as libc::c_int,
    3407 as libc::c_int,
    3413 as libc::c_int,
    3433 as libc::c_int,
    3449 as libc::c_int,
    3457 as libc::c_int,
    3461 as libc::c_int,
    3463 as libc::c_int,
    3467 as libc::c_int,
    3469 as libc::c_int,
    3491 as libc::c_int,
    3499 as libc::c_int,
    3511 as libc::c_int,
    3517 as libc::c_int,
    3527 as libc::c_int,
    3529 as libc::c_int,
    3533 as libc::c_int,
    3539 as libc::c_int,
    3541 as libc::c_int,
    3547 as libc::c_int,
    3557 as libc::c_int,
    3559 as libc::c_int,
    3571 as libc::c_int,
    3581 as libc::c_int,
    3583 as libc::c_int,
    3593 as libc::c_int,
    3607 as libc::c_int,
    3613 as libc::c_int,
    3617 as libc::c_int,
    3623 as libc::c_int,
    3631 as libc::c_int,
    3637 as libc::c_int,
    3643 as libc::c_int,
    3659 as libc::c_int,
    3671 as libc::c_int,
    3673 as libc::c_int,
    3677 as libc::c_int,
    3691 as libc::c_int,
    3697 as libc::c_int,
    3701 as libc::c_int,
    3709 as libc::c_int,
    3719 as libc::c_int,
    3727 as libc::c_int,
    3733 as libc::c_int,
    3739 as libc::c_int,
    3761 as libc::c_int,
    3767 as libc::c_int,
    3769 as libc::c_int,
    3779 as libc::c_int,
    3793 as libc::c_int,
    3797 as libc::c_int,
    3803 as libc::c_int,
    3821 as libc::c_int,
    3823 as libc::c_int,
    3833 as libc::c_int,
    3847 as libc::c_int,
    3851 as libc::c_int,
    3853 as libc::c_int,
    3863 as libc::c_int,
    3877 as libc::c_int,
    3881 as libc::c_int,
    3889 as libc::c_int,
    3907 as libc::c_int,
    3911 as libc::c_int,
    3917 as libc::c_int,
    3919 as libc::c_int,
    3923 as libc::c_int,
    3929 as libc::c_int,
    3931 as libc::c_int,
    3943 as libc::c_int,
    3947 as libc::c_int,
    3967 as libc::c_int,
    3989 as libc::c_int,
    4001 as libc::c_int,
    4003 as libc::c_int,
    4007 as libc::c_int,
    4013 as libc::c_int,
    4019 as libc::c_int,
    4021 as libc::c_int,
    4027 as libc::c_int,
    4049 as libc::c_int,
    4051 as libc::c_int,
    4057 as libc::c_int,
    4073 as libc::c_int,
    4079 as libc::c_int,
    4091 as libc::c_int,
    4093 as libc::c_int,
    4099 as libc::c_int,
    4111 as libc::c_int,
    4127 as libc::c_int,
    4129 as libc::c_int,
    4133 as libc::c_int,
    4139 as libc::c_int,
    4153 as libc::c_int,
    4157 as libc::c_int,
    4159 as libc::c_int,
    4177 as libc::c_int,
    4201 as libc::c_int,
    4211 as libc::c_int,
    4217 as libc::c_int,
    4219 as libc::c_int,
    4229 as libc::c_int,
    4231 as libc::c_int,
    4241 as libc::c_int,
    4243 as libc::c_int,
    4253 as libc::c_int,
    4259 as libc::c_int,
    4261 as libc::c_int,
    4271 as libc::c_int,
    4273 as libc::c_int,
    4283 as libc::c_int,
    4289 as libc::c_int,
    4297 as libc::c_int,
    4327 as libc::c_int,
    4337 as libc::c_int,
    4339 as libc::c_int,
    4349 as libc::c_int,
    4357 as libc::c_int,
    4363 as libc::c_int,
    4373 as libc::c_int,
    4391 as libc::c_int,
    4397 as libc::c_int,
    4409 as libc::c_int,
    4421 as libc::c_int,
    4423 as libc::c_int,
    4441 as libc::c_int,
    4447 as libc::c_int,
    4451 as libc::c_int,
    4457 as libc::c_int,
    4463 as libc::c_int,
    4481 as libc::c_int,
    4483 as libc::c_int,
    4493 as libc::c_int,
    4507 as libc::c_int,
    4513 as libc::c_int,
    4517 as libc::c_int,
    4519 as libc::c_int,
    4523 as libc::c_int,
    4547 as libc::c_int,
    4549 as libc::c_int,
    4561 as libc::c_int,
    4567 as libc::c_int,
    4583 as libc::c_int,
    4591 as libc::c_int,
    4597 as libc::c_int,
    4603 as libc::c_int,
    4621 as libc::c_int,
    4637 as libc::c_int,
    4639 as libc::c_int,
    4643 as libc::c_int,
    4649 as libc::c_int,
    4651 as libc::c_int,
    4657 as libc::c_int,
    4663 as libc::c_int,
    4673 as libc::c_int,
    4679 as libc::c_int,
    4691 as libc::c_int,
    4703 as libc::c_int,
    4721 as libc::c_int,
    4723 as libc::c_int,
    4729 as libc::c_int,
    4733 as libc::c_int,
    4751 as libc::c_int,
    4759 as libc::c_int,
    4783 as libc::c_int,
    4787 as libc::c_int,
    4789 as libc::c_int,
    4793 as libc::c_int,
    4799 as libc::c_int,
    4801 as libc::c_int,
    4813 as libc::c_int,
    4817 as libc::c_int,
    4831 as libc::c_int,
    4861 as libc::c_int,
    4871 as libc::c_int,
    4877 as libc::c_int,
    4889 as libc::c_int,
    4903 as libc::c_int,
    4909 as libc::c_int,
    4919 as libc::c_int,
    4931 as libc::c_int,
    4933 as libc::c_int,
    4937 as libc::c_int,
    4943 as libc::c_int,
    4951 as libc::c_int,
    4957 as libc::c_int,
    4967 as libc::c_int,
    4969 as libc::c_int,
    4973 as libc::c_int,
    4987 as libc::c_int,
    4993 as libc::c_int,
    4999 as libc::c_int,
    5003 as libc::c_int,
    5009 as libc::c_int,
    5011 as libc::c_int,
    5021 as libc::c_int,
    5023 as libc::c_int,
    5039 as libc::c_int,
    5051 as libc::c_int,
    5059 as libc::c_int,
    5077 as libc::c_int,
    5081 as libc::c_int,
    5087 as libc::c_int,
    5099 as libc::c_int,
    5101 as libc::c_int,
    5107 as libc::c_int,
    5113 as libc::c_int,
    5119 as libc::c_int,
    5147 as libc::c_int,
    5153 as libc::c_int,
    5167 as libc::c_int,
    5171 as libc::c_int,
    5179 as libc::c_int,
    5189 as libc::c_int,
    5197 as libc::c_int,
    5209 as libc::c_int,
    5227 as libc::c_int,
    5231 as libc::c_int,
    5233 as libc::c_int,
    5237 as libc::c_int,
    5261 as libc::c_int,
    5273 as libc::c_int,
    5279 as libc::c_int,
    5281 as libc::c_int,
    5297 as libc::c_int,
    5303 as libc::c_int,
    5309 as libc::c_int,
    5323 as libc::c_int,
    5333 as libc::c_int,
    5347 as libc::c_int,
    5351 as libc::c_int,
    5381 as libc::c_int,
    5387 as libc::c_int,
    5393 as libc::c_int,
    5399 as libc::c_int,
    5407 as libc::c_int,
    5413 as libc::c_int,
    5417 as libc::c_int,
    5419 as libc::c_int,
    5431 as libc::c_int,
    5437 as libc::c_int,
    5441 as libc::c_int,
    5443 as libc::c_int,
    5449 as libc::c_int,
    5471 as libc::c_int,
    5477 as libc::c_int,
    5479 as libc::c_int,
    5483 as libc::c_int,
    5501 as libc::c_int,
    5503 as libc::c_int,
    5507 as libc::c_int,
    5519 as libc::c_int,
    5521 as libc::c_int,
    5527 as libc::c_int,
    5531 as libc::c_int,
    5557 as libc::c_int,
    5563 as libc::c_int,
    5569 as libc::c_int,
    5573 as libc::c_int,
    5581 as libc::c_int,
    5591 as libc::c_int,
    5623 as libc::c_int,
    5639 as libc::c_int,
    5641 as libc::c_int,
    5647 as libc::c_int,
    5651 as libc::c_int,
    5653 as libc::c_int,
    5657 as libc::c_int,
    5659 as libc::c_int,
    5669 as libc::c_int,
    5683 as libc::c_int,
    5689 as libc::c_int,
    5693 as libc::c_int,
    5701 as libc::c_int,
    5711 as libc::c_int,
    5717 as libc::c_int,
    5737 as libc::c_int,
    5741 as libc::c_int,
    5743 as libc::c_int,
    5749 as libc::c_int,
    5779 as libc::c_int,
    5783 as libc::c_int,
    5791 as libc::c_int,
    5801 as libc::c_int,
    5807 as libc::c_int,
    5813 as libc::c_int,
    5821 as libc::c_int,
    5827 as libc::c_int,
    5839 as libc::c_int,
    5843 as libc::c_int,
    5849 as libc::c_int,
    5851 as libc::c_int,
    5857 as libc::c_int,
    5861 as libc::c_int,
    5867 as libc::c_int,
    5869 as libc::c_int,
    5879 as libc::c_int,
    5881 as libc::c_int,
    5897 as libc::c_int,
    5903 as libc::c_int,
    5923 as libc::c_int,
    5927 as libc::c_int,
    5939 as libc::c_int,
    5953 as libc::c_int,
    5981 as libc::c_int,
    5987 as libc::c_int,
    6007 as libc::c_int,
    6011 as libc::c_int,
    6029 as libc::c_int,
    6037 as libc::c_int,
    6043 as libc::c_int,
    6047 as libc::c_int,
    6053 as libc::c_int,
    6067 as libc::c_int,
    6073 as libc::c_int,
    6079 as libc::c_int,
    6089 as libc::c_int,
    6091 as libc::c_int,
    6101 as libc::c_int,
    6113 as libc::c_int,
    6121 as libc::c_int,
    6131 as libc::c_int,
    6133 as libc::c_int,
    6143 as libc::c_int,
    6151 as libc::c_int,
    6163 as libc::c_int,
    6173 as libc::c_int,
    6197 as libc::c_int,
    6199 as libc::c_int,
    6203 as libc::c_int,
    6211 as libc::c_int,
    6217 as libc::c_int,
    6221 as libc::c_int,
    6229 as libc::c_int,
    6247 as libc::c_int,
    6257 as libc::c_int,
    6263 as libc::c_int,
    6269 as libc::c_int,
    6271 as libc::c_int,
    6277 as libc::c_int,
    6287 as libc::c_int,
    6299 as libc::c_int,
    6301 as libc::c_int,
    6311 as libc::c_int,
    6317 as libc::c_int,
    6323 as libc::c_int,
    6329 as libc::c_int,
    6337 as libc::c_int,
    6343 as libc::c_int,
    6353 as libc::c_int,
    6359 as libc::c_int,
    6361 as libc::c_int,
    6367 as libc::c_int,
    6373 as libc::c_int,
    6379 as libc::c_int,
    6389 as libc::c_int,
    6397 as libc::c_int,
    6421 as libc::c_int,
    6427 as libc::c_int,
    6449 as libc::c_int,
    6451 as libc::c_int,
    6469 as libc::c_int,
    6473 as libc::c_int,
    6481 as libc::c_int,
    6491 as libc::c_int,
    6521 as libc::c_int,
    6529 as libc::c_int,
    6547 as libc::c_int,
    6551 as libc::c_int,
    6553 as libc::c_int,
    6563 as libc::c_int,
    6569 as libc::c_int,
    6571 as libc::c_int,
    6577 as libc::c_int,
    6581 as libc::c_int,
    6599 as libc::c_int,
    6607 as libc::c_int,
    6619 as libc::c_int,
    6637 as libc::c_int,
    6653 as libc::c_int,
    6659 as libc::c_int,
    6661 as libc::c_int,
    6673 as libc::c_int,
    6679 as libc::c_int,
    6689 as libc::c_int,
    6691 as libc::c_int,
    6701 as libc::c_int,
    6703 as libc::c_int,
    6709 as libc::c_int,
    6719 as libc::c_int,
    6733 as libc::c_int,
    6737 as libc::c_int,
    6761 as libc::c_int,
    6763 as libc::c_int,
    6779 as libc::c_int,
    6781 as libc::c_int,
    6791 as libc::c_int,
    6793 as libc::c_int,
    6803 as libc::c_int,
    6823 as libc::c_int,
    6827 as libc::c_int,
    6829 as libc::c_int,
    6833 as libc::c_int,
    6841 as libc::c_int,
    6857 as libc::c_int,
    6863 as libc::c_int,
    6869 as libc::c_int,
    6871 as libc::c_int,
    6883 as libc::c_int,
    6899 as libc::c_int,
    6907 as libc::c_int,
    6911 as libc::c_int,
    6917 as libc::c_int,
    6947 as libc::c_int,
    6949 as libc::c_int,
    6959 as libc::c_int,
    6961 as libc::c_int,
    6967 as libc::c_int,
    6971 as libc::c_int,
    6977 as libc::c_int,
    6983 as libc::c_int,
    6991 as libc::c_int,
    6997 as libc::c_int,
    7001 as libc::c_int,
    7013 as libc::c_int,
    7019 as libc::c_int,
    7027 as libc::c_int,
    7039 as libc::c_int,
    7043 as libc::c_int,
    7057 as libc::c_int,
    7069 as libc::c_int,
    7079 as libc::c_int,
    7103 as libc::c_int,
    7109 as libc::c_int,
    7121 as libc::c_int,
    7127 as libc::c_int,
    7129 as libc::c_int,
    7151 as libc::c_int,
    7159 as libc::c_int,
    7177 as libc::c_int,
    7187 as libc::c_int,
    7193 as libc::c_int,
    7207 as libc::c_int,
    7211 as libc::c_int,
    7213 as libc::c_int,
    7219 as libc::c_int,
    7229 as libc::c_int,
    7237 as libc::c_int,
    7243 as libc::c_int,
    7247 as libc::c_int,
    7253 as libc::c_int,
    7283 as libc::c_int,
    7297 as libc::c_int,
    7307 as libc::c_int,
    7309 as libc::c_int,
    7321 as libc::c_int,
    7331 as libc::c_int,
    7333 as libc::c_int,
    7349 as libc::c_int,
    7351 as libc::c_int,
    7369 as libc::c_int,
    7393 as libc::c_int,
    7411 as libc::c_int,
    7417 as libc::c_int,
    7433 as libc::c_int,
    7451 as libc::c_int,
    7457 as libc::c_int,
    7459 as libc::c_int,
    7477 as libc::c_int,
    7481 as libc::c_int,
    7487 as libc::c_int,
    7489 as libc::c_int,
    7499 as libc::c_int,
    7507 as libc::c_int,
    7517 as libc::c_int,
    7523 as libc::c_int,
    7529 as libc::c_int,
    7537 as libc::c_int,
    7541 as libc::c_int,
    7547 as libc::c_int,
    7549 as libc::c_int,
    7559 as libc::c_int,
    7561 as libc::c_int,
    7573 as libc::c_int,
    7577 as libc::c_int,
    7583 as libc::c_int,
    7589 as libc::c_int,
    7591 as libc::c_int,
    7603 as libc::c_int,
    7607 as libc::c_int,
    7621 as libc::c_int,
    7639 as libc::c_int,
    7643 as libc::c_int,
    7649 as libc::c_int,
    7669 as libc::c_int,
    7673 as libc::c_int,
    7681 as libc::c_int,
    7687 as libc::c_int,
    7691 as libc::c_int,
    7699 as libc::c_int,
    7703 as libc::c_int,
    7717 as libc::c_int,
    7723 as libc::c_int,
    7727 as libc::c_int,
    7741 as libc::c_int,
    7753 as libc::c_int,
    7757 as libc::c_int,
    7759 as libc::c_int,
    7789 as libc::c_int,
    7793 as libc::c_int,
    7817 as libc::c_int,
    7823 as libc::c_int,
    7829 as libc::c_int,
    7841 as libc::c_int,
    7853 as libc::c_int,
    7867 as libc::c_int,
    7873 as libc::c_int,
    7877 as libc::c_int,
    7879 as libc::c_int,
    7883 as libc::c_int,
    7901 as libc::c_int,
    7907 as libc::c_int,
    7919 as libc::c_int,
    7927 as libc::c_int,
    7933 as libc::c_int,
    7937 as libc::c_int,
    7949 as libc::c_int,
    7951 as libc::c_int,
    7963 as libc::c_int,
    7993 as libc::c_int,
    8009 as libc::c_int,
    8011 as libc::c_int,
    8017 as libc::c_int,
    8039 as libc::c_int,
    8053 as libc::c_int,
    8059 as libc::c_int,
    8069 as libc::c_int,
    8081 as libc::c_int,
    8087 as libc::c_int,
    8089 as libc::c_int,
    8093 as libc::c_int,
    8101 as libc::c_int,
    8111 as libc::c_int,
    8117 as libc::c_int,
    8123 as libc::c_int,
    8147 as libc::c_int,
    8161 as libc::c_int,
    8167 as libc::c_int,
    8171 as libc::c_int,
    8179 as libc::c_int,
    8191 as libc::c_int,
    8209 as libc::c_int,
    8219 as libc::c_int,
    8221 as libc::c_int,
    8231 as libc::c_int,
    8233 as libc::c_int,
    8237 as libc::c_int,
    8243 as libc::c_int,
    8263 as libc::c_int,
    8269 as libc::c_int,
    8273 as libc::c_int,
    8287 as libc::c_int,
    8291 as libc::c_int,
    8293 as libc::c_int,
    8297 as libc::c_int,
    8311 as libc::c_int,
    8317 as libc::c_int,
    8329 as libc::c_int,
    8353 as libc::c_int,
    8363 as libc::c_int,
    8369 as libc::c_int,
    8377 as libc::c_int,
    8387 as libc::c_int,
    8389 as libc::c_int,
    8419 as libc::c_int,
    8423 as libc::c_int,
    8429 as libc::c_int,
    8431 as libc::c_int,
    8443 as libc::c_int,
    8447 as libc::c_int,
    8461 as libc::c_int,
    8467 as libc::c_int,
    8501 as libc::c_int,
    8513 as libc::c_int,
    8521 as libc::c_int,
    8527 as libc::c_int,
    8537 as libc::c_int,
    8539 as libc::c_int,
    8543 as libc::c_int,
    8563 as libc::c_int,
    8573 as libc::c_int,
    8581 as libc::c_int,
    8597 as libc::c_int,
    8599 as libc::c_int,
    8609 as libc::c_int,
    8623 as libc::c_int,
    8627 as libc::c_int,
    8629 as libc::c_int,
    8641 as libc::c_int,
    8647 as libc::c_int,
    8663 as libc::c_int,
    8669 as libc::c_int,
    8677 as libc::c_int,
    8681 as libc::c_int,
    8689 as libc::c_int,
    8693 as libc::c_int,
    8699 as libc::c_int,
    8707 as libc::c_int,
    8713 as libc::c_int,
    8719 as libc::c_int,
    8731 as libc::c_int,
    8737 as libc::c_int,
    8741 as libc::c_int,
    8747 as libc::c_int,
    8753 as libc::c_int,
    8761 as libc::c_int,
    8779 as libc::c_int,
    8783 as libc::c_int,
    8803 as libc::c_int,
    8807 as libc::c_int,
    8819 as libc::c_int,
    8821 as libc::c_int,
    8831 as libc::c_int,
    8837 as libc::c_int,
    8839 as libc::c_int,
    8849 as libc::c_int,
    8861 as libc::c_int,
    8863 as libc::c_int,
    8867 as libc::c_int,
    8887 as libc::c_int,
    8893 as libc::c_int,
    8923 as libc::c_int,
    8929 as libc::c_int,
    8933 as libc::c_int,
    8941 as libc::c_int,
    8951 as libc::c_int,
    8963 as libc::c_int,
    8969 as libc::c_int,
    8971 as libc::c_int,
    8999 as libc::c_int,
    9001 as libc::c_int,
    9007 as libc::c_int,
    9011 as libc::c_int,
    9013 as libc::c_int,
    9029 as libc::c_int,
    9041 as libc::c_int,
    9043 as libc::c_int,
    9049 as libc::c_int,
    9059 as libc::c_int,
    9067 as libc::c_int,
    9091 as libc::c_int,
    9103 as libc::c_int,
    9109 as libc::c_int,
    9127 as libc::c_int,
    9133 as libc::c_int,
    9137 as libc::c_int,
    9151 as libc::c_int,
    9157 as libc::c_int,
    9161 as libc::c_int,
    9173 as libc::c_int,
    9181 as libc::c_int,
    9187 as libc::c_int,
    9199 as libc::c_int,
    9203 as libc::c_int,
    9209 as libc::c_int,
    9221 as libc::c_int,
    9227 as libc::c_int,
    9239 as libc::c_int,
    9241 as libc::c_int,
    9257 as libc::c_int,
    9277 as libc::c_int,
    9281 as libc::c_int,
    9283 as libc::c_int,
    9293 as libc::c_int,
    9311 as libc::c_int,
    9319 as libc::c_int,
    9323 as libc::c_int,
    9337 as libc::c_int,
    9341 as libc::c_int,
    9343 as libc::c_int,
    9349 as libc::c_int,
    9371 as libc::c_int,
    9377 as libc::c_int,
    9391 as libc::c_int,
    9397 as libc::c_int,
    9403 as libc::c_int,
    9413 as libc::c_int,
    9419 as libc::c_int,
    9421 as libc::c_int,
    9431 as libc::c_int,
    9433 as libc::c_int,
    9437 as libc::c_int,
    9439 as libc::c_int,
    9461 as libc::c_int,
    9463 as libc::c_int,
    9467 as libc::c_int,
    9473 as libc::c_int,
    9479 as libc::c_int,
    9491 as libc::c_int,
    9497 as libc::c_int,
    9511 as libc::c_int,
    9521 as libc::c_int,
    9533 as libc::c_int,
    9539 as libc::c_int,
    9547 as libc::c_int,
    9551 as libc::c_int,
    9587 as libc::c_int,
    9601 as libc::c_int,
    9613 as libc::c_int,
    9619 as libc::c_int,
    9623 as libc::c_int,
    9629 as libc::c_int,
    9631 as libc::c_int,
    9643 as libc::c_int,
    9649 as libc::c_int,
    9661 as libc::c_int,
    9677 as libc::c_int,
    9679 as libc::c_int,
    9689 as libc::c_int,
    9697 as libc::c_int,
    9719 as libc::c_int,
    9721 as libc::c_int,
    9733 as libc::c_int,
    9739 as libc::c_int,
    9743 as libc::c_int,
    9749 as libc::c_int,
    9767 as libc::c_int,
    9769 as libc::c_int,
    9781 as libc::c_int,
    9787 as libc::c_int,
    9791 as libc::c_int,
    9803 as libc::c_int,
    9811 as libc::c_int,
    9817 as libc::c_int,
    9829 as libc::c_int,
    9833 as libc::c_int,
    9839 as libc::c_int,
    9851 as libc::c_int,
    9857 as libc::c_int,
    9859 as libc::c_int,
    9871 as libc::c_int,
    9883 as libc::c_int,
    9887 as libc::c_int,
    9901 as libc::c_int,
    9907 as libc::c_int,
    9923 as libc::c_int,
    9929 as libc::c_int,
    9931 as libc::c_int,
    9941 as libc::c_int,
    9949 as libc::c_int,
    9967 as libc::c_int,
    9973 as libc::c_int,
];
unsafe extern "C" fn halton_state_size(mut dimension: libc::c_uint) -> size_t {
    return ::core::mem::size_of::<halton_state_t>() as libc::c_ulong;
}
unsafe extern "C" fn halton_init(
    mut state: *mut libc::c_void,
    mut dimension: libc::c_uint,
) -> libc::c_int {
    let mut h_state: *mut halton_state_t = state as *mut halton_state_t;
    (*h_state).sequence_count = 0 as libc::c_int as libc::c_uint;
    if dimension < 1 as libc::c_int as libc::c_uint
        || dimension > 1229 as libc::c_int as libc::c_uint
    {
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn vdcorput(mut x: libc::c_int, mut b: libc::c_int) -> libc::c_double {
    let mut r: libc::c_double = 0.0f64;
    let mut v: libc::c_double = 1.0f64;
    let mut binv: libc::c_double = 1.0f64 / b as libc::c_double;
    while x > 0 as libc::c_int {
        v *= binv;
        r += v * (x % b) as libc::c_double;
        x /= b;
    }
    return r;
}
unsafe extern "C" fn halton_get(
    mut state: *mut libc::c_void,
    mut dimension: libc::c_uint,
    mut v: *mut libc::c_double,
) -> libc::c_int {
    let mut h_state: *mut halton_state_t = state as *mut halton_state_t;
    let mut i: libc::c_uint = 0;
    if dimension < 1 as libc::c_int as libc::c_uint
        || dimension > 1229 as libc::c_int as libc::c_uint
    {
        return GSL_EINVAL as libc::c_int;
    }
    (*h_state).sequence_count = ((*h_state).sequence_count).wrapping_add(1);
    (*h_state).sequence_count;
    i = 0 as libc::c_int as libc::c_uint;
    while i < dimension {
        *v
            .offset(
                i as isize,
            ) = vdcorput(
            (*h_state).sequence_count as libc::c_int,
            prime_numbers[i as usize],
        );
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
