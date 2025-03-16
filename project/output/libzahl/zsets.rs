#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm, label_break_value)]
use core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: libc::c_int,
    pub padding__: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [zahl; 1];
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISdigit = 2048,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
}

#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17254579315488500390;
        }
        19 => {
            current_block_42 = 17254579315488500390;
        }
        18 => {
            current_block_42 = 16864574447450575806;
        }
        17 => {
            current_block_42 = 4745558310823793980;
        }
        16 => {
            current_block_42 = 3609958504996442327;
        }
        15 => {
            current_block_42 = 7591989803176764182;
        }
        14 => {
            current_block_42 = 18018901114339951972;
        }
        13 => {
            current_block_42 = 12689878998560279573;
        }
        12 => {
            current_block_42 = 5286769970458876423;
        }
        11 => {
            current_block_42 = 7754037464575168929;
        }
        10 => {
            current_block_42 = 3450148331727835525;
        }
        9 => {
            current_block_42 = 17513148302838498461;
        }
        8 => {
            current_block_42 = 5033545425852514390;
        }
        7 => {
            current_block_42 = 7822129706622160761;
        }
        6 => {
            current_block_42 = 14314526103220412958;
        }
        5 => {
            current_block_42 = 11474465627670557597;
        }
        4 => {
            current_block_42 = 13771482776398185125;
        }
        3 => {
            current_block_42 = 519160363748337264;
        }
        2 => {
            current_block_42 = 17126217794288990979;
        }
        1 => {
            current_block_42 = 11647970138234258413;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n    shlq $3, {3}\n    addq {1}, {3}\n 1:\n    movq 0({2}), {0}\n    movq {0}, 0({1})\n    movq 8({2}), {0}\n    movq {0}, 8({1})\n    movq 16({2}), {0}\n    movq {0}, 16({1})\n    movq 24({2}), {0}\n    movq {0}, 24({1})\n    addq $32, {2}\n    addq $32, {1}\n    cmpq {3}, {1}\n    jl 1b",
                lateout(reg) t, inlateout(reg) d, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17254579315488500390 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16864574447450575806;
        }
        _ => {}
    }
    match current_block_42 {
        16864574447450575806 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4745558310823793980;
        }
        _ => {}
    }
    match current_block_42 {
        4745558310823793980 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3609958504996442327;
        }
        _ => {}
    }
    match current_block_42 {
        3609958504996442327 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7591989803176764182;
        }
        _ => {}
    }
    match current_block_42 {
        7591989803176764182 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 18018901114339951972;
        }
        _ => {}
    }
    match current_block_42 {
        18018901114339951972 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12689878998560279573;
        }
        _ => {}
    }
    match current_block_42 {
        12689878998560279573 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5286769970458876423;
        }
        _ => {}
    }
    match current_block_42 {
        5286769970458876423 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7754037464575168929;
        }
        _ => {}
    }
    match current_block_42 {
        7754037464575168929 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3450148331727835525;
        }
        _ => {}
    }
    match current_block_42 {
        3450148331727835525 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17513148302838498461;
        }
        _ => {}
    }
    match current_block_42 {
        17513148302838498461 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5033545425852514390;
        }
        _ => {}
    }
    match current_block_42 {
        5033545425852514390 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7822129706622160761;
        }
        _ => {}
    }
    match current_block_42 {
        7822129706622160761 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14314526103220412958;
        }
        _ => {}
    }
    match current_block_42 {
        14314526103220412958 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11474465627670557597;
        }
        _ => {}
    }
    match current_block_42 {
        11474465627670557597 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13771482776398185125;
        }
        _ => {}
    }
    match current_block_42 {
        13771482776398185125 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 519160363748337264;
        }
        _ => {}
    }
    match current_block_42 {
        519160363748337264 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17126217794288990979;
        }
        _ => {}
    }
    match current_block_42 {
        17126217794288990979 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11647970138234258413;
        }
        _ => {}
    }
    match current_block_42 {
        11647970138234258413 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: libc::c_int = 0;
    let mut c_sign: libc::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
}
#[no_mangle]
pub unsafe extern "C" fn zsets(
    mut a: *mut zahl,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut temp: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut neg: libc::c_int = (*str as libc::c_int == '-' as i32) as libc::c_int;
    let mut str_end: *const libc::c_char = 0 as *const libc::c_char;
    str = str
        .offset((neg != 0 || *str as libc::c_int == '+' as i32) as libc::c_int as isize);
    if (*str == 0) as libc::c_int as libc::c_long != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    str_end = str;
    while *str_end != 0 {
        if (*(*__ctype_b_loc()).offset(*str_end as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0)
            as libc::c_int as libc::c_long != 0
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        str_end = str_end.offset(1);
        str_end;
    }
    (*a).sign = 0 as libc::c_int;
    zset(libzahl_tmp_str_num.as_mut_ptr(), libzahl_const_1e19.as_mut_ptr());
    's_183: {
        let mut current_block_36: u64;
        match str_end.offset_from(str) as libc::c_long
            % 19 as libc::c_int as libc::c_long
        {
            0 => {
                current_block_36 = 9281452336402489912;
            }
            18 => {
                current_block_36 = 9113683985054832900;
            }
            17 => {
                current_block_36 = 15988712159247554322;
            }
            16 => {
                current_block_36 = 5976146557728084725;
            }
            15 => {
                current_block_36 = 14218857524381082706;
            }
            14 => {
                current_block_36 = 9294588496553486658;
            }
            13 => {
                current_block_36 = 13790796995684902264;
            }
            12 => {
                current_block_36 = 1869334804241041290;
            }
            11 => {
                current_block_36 = 5516530427166507445;
            }
            10 => {
                current_block_36 = 10703124919230875173;
            }
            9 => {
                current_block_36 = 5736421875614469061;
            }
            8 => {
                current_block_36 = 6246763041535604849;
            }
            7 => {
                current_block_36 = 696810026292259735;
            }
            6 => {
                current_block_36 = 18195282705153904920;
            }
            5 => {
                current_block_36 = 16051936835507880021;
            }
            4 => {
                current_block_36 = 14928940103145537714;
            }
            3 => {
                current_block_36 = 12999328127362432425;
            }
            2 => {
                current_block_36 = 2430319949834955361;
            }
            1 => {
                current_block_36 = 14180127615403059768;
            }
            _ => {
                current_block_36 = 10692455896603418738;
            }
        }
        loop {
            match current_block_36 {
                10692455896603418738 => {
                    break 's_183;
                }
                9281452336402489912 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh0 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh0 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 9113683985054832900;
                }
                9113683985054832900 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh1 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh1 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 15988712159247554322;
                }
                15988712159247554322 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh2 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh2 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 5976146557728084725;
                }
                5976146557728084725 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh3 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh3 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 14218857524381082706;
                }
                14218857524381082706 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh4 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh4 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 9294588496553486658;
                }
                9294588496553486658 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh5 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh5 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 13790796995684902264;
                }
                13790796995684902264 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh6 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh6 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 1869334804241041290;
                }
                1869334804241041290 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh7 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh7 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 5516530427166507445;
                }
                5516530427166507445 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh8 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh8 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 10703124919230875173;
                }
                10703124919230875173 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh9 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh9 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 5736421875614469061;
                }
                5736421875614469061 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh10 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh10 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 6246763041535604849;
                }
                6246763041535604849 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh11 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh11 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 696810026292259735;
                }
                696810026292259735 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh12 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh12 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 18195282705153904920;
                }
                18195282705153904920 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh13 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh13 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 16051936835507880021;
                }
                16051936835507880021 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh14 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh14 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 14928940103145537714;
                }
                14928940103145537714 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh15 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh15 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 12999328127362432425;
                }
                12999328127362432425 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh16 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh16 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 2430319949834955361;
                }
                2430319949834955361 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh17 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh17 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 14180127615403059768;
                }
                _ => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh18 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh18 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    if !(temp == 0) {
                        *((*libzahl_tmp_str_num.as_mut_ptr()).chars)
                            .offset(0 as libc::c_int as isize) = temp as zahl_char_t;
                        zadd(a, a, libzahl_tmp_str_num.as_mut_ptr());
                    }
                    if !(*str != 0) {
                        current_block_36 = 10692455896603418738;
                        continue;
                    }
                    zmul(a, a, libzahl_const_1e19.as_mut_ptr());
                    temp = 0 as libc::c_int as libc::c_ulonglong;
                    current_block_36 = 9281452336402489912;
                }
            }
        }
    }
    if (neg != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = -zsignum(a);
    }
    return 0 as libc::c_int;
}
