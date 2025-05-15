use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    fn mt_basename(filename: *const i8) -> *const i8;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn wcscmp(_: *const i32, _: *const i32) -> i32;
    fn iswcntrl(__wc: wint_t) -> i32;
    fn iswlower(__wc: wint_t) -> i32;
    fn iswupper(__wc: wint_t) -> i32;
    fn towupper(__wc: wint_t) -> wint_t;
    static mut mtools_no_vfat: u32;
    static mut mtools_ignore_short_case: u32;
    fn autorename_short(_: *mut dos_name_t, _: i32);
    fn dos_to_wchar(
        fromDos: *mut doscp_t,
        dos: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
    ) -> size_t;
    fn wchar_to_dos(
        toDos: *mut doscp_t,
        wchar: *mut wchar_t,
        dos: *mut i8,
        len: size_t,
        mangled: *mut i32,
    );
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut i8,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
    fn native_to_wchar(
        native: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const i8,
        mangled: *mut i32,
    ) -> size_t;
}
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type wchar_t = i32;
pub type wint_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dos_name_t {
    pub base: [i8; 8],
    pub ext: [i8; 3],
    pub sentinel: i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum Case_l {
    LOWER = 2,
    UPPER = 1,
    NONE = 0,
}
impl Case_l {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            Case_l::LOWER => 2,
            Case_l::UPPER => 1,
            Case_l::NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> Case_l {
        match value {
            2 => Case_l::LOWER,
            1 => Case_l::UPPER,
            0 => Case_l::NONE,
            _ => panic!("Invalid value for Case_l: {}", value),
        }
    }
}
impl AddAssign<u32> for Case_l {
    fn add_assign(&mut self, rhs: u32) {
        *self = Case_l::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for Case_l {
    fn sub_assign(&mut self, rhs: u32) {
        *self = Case_l::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for Case_l {
    fn mul_assign(&mut self, rhs: u32) {
        *self = Case_l::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for Case_l {
    fn div_assign(&mut self, rhs: u32) {
        *self = Case_l::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for Case_l {
    fn rem_assign(&mut self, rhs: u32) {
        *self = Case_l::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for Case_l {
    type Output = Case_l;
    fn add(self, rhs: u32) -> Case_l {
        Case_l::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for Case_l {
    type Output = Case_l;
    fn sub(self, rhs: u32) -> Case_l {
        Case_l::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for Case_l {
    type Output = Case_l;
    fn mul(self, rhs: u32) -> Case_l {
        Case_l::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for Case_l {
    type Output = Case_l;
    fn div(self, rhs: u32) -> Case_l {
        Case_l::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for Case_l {
    type Output = Case_l;
    fn rem(self, rhs: u32) -> Case_l {
        Case_l::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type Case_t = Case_l;
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_tolower(mut ch: i8) -> i8 {
    return ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = ch as u8 as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(ch as u8 as i32);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(ch as u8 as i32 as isize);
        }
        __res
    }) as i8;
}
#[inline]
unsafe extern "C" fn ch_towupper(mut ch: wchar_t) -> wchar_t {
    return towupper(ch as wint_t) as wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn unix_normalize(
    mut cp: *mut doscp_t,
    mut ans: *mut i8,
    mut dn: *mut dos_name_t,
    mut ans_size: size_t,
) -> *mut i8 {
    let mut buffer: [i8; 13] = [0; 13];
    let mut wbuffer: [wchar_t; 13] = [0; 13];
    let mut a: *mut i8 = 0 as *mut i8;
    let mut j: i32 = 0;
    a = buffer.as_mut_ptr();
    j = 0 as i32;
    while j < 8 as i32 && (*dn).base[j as usize] as i32 > ' ' as i32 {
        *a = (*dn).base[j as usize];
        j += 1;
        j;
        a = a.offset(1);
        a;
    }
    if (*dn).ext[0 as i32 as usize] as i32 > ' ' as i32 {
        let fresh0 = a;
        a = a.offset(1);
        *fresh0 = '.' as i32 as i8;
        j = 0 as i32;
        while j < 3 as i32 && (*dn).ext[j as usize] as i32 > ' ' as i32 {
            *a = (*dn).ext[j as usize];
            j += 1;
            j;
            a = a.offset(1);
            a;
        }
    }
    let fresh1 = a;
    a = a.offset(1);
    *fresh1 = '\0' as i32 as i8;
    dos_to_wchar(cp, buffer.as_mut_ptr(), wbuffer.as_mut_ptr(), 13 as i32 as size_t);
    wchar_to_native(wbuffer.as_mut_ptr(), ans, 13 as i32 as size_t, ans_size);
    return ans;
}
unsafe extern "C" fn TranslateToDos(
    mut toDos: *mut doscp_t,
    mut in_0: *const i8,
    mut out: *mut i8,
    mut count: size_t,
    mut end: *mut i8,
    mut Case: *mut Case_t,
    mut mangled: *mut i32,
) {
    let mut buffer: [wchar_t; 12] = [0; 12];
    let mut s: *mut wchar_t = buffer.as_mut_ptr();
    let mut t_idx: size_t = 0 as i32 as size_t;
    native_to_wchar(in_0, buffer.as_mut_ptr(), count, end, mangled);
    buffer[count as usize] = '\0' as i32;
    *Case = Case_l::NONE;
    while *s != 0 {
        if *s == ' ' as i32 || *s == '.' as i32 {
            *mangled |= 3 as i32;
        } else {
            if iswcntrl(*s as wint_t) != 0 {
                *mangled |= 3 as i32;
                buffer[t_idx as usize] = '_' as i32;
            } else if iswlower(*s as wint_t) != 0 {
                buffer[t_idx as usize] = ch_towupper(*s);
                if *Case as u32 == Case_l::UPPER as i32 as u32 && mtools_no_vfat == 0 {
                    *mangled |= 1 as i32;
                } else {
                    *Case = Case_l::LOWER;
                }
            } else if iswupper(*s as wint_t) != 0 {
                buffer[t_idx as usize] = *s;
                if *Case as u32 == Case_l::LOWER as i32 as u32 && mtools_no_vfat == 0 {
                    *mangled |= 1 as i32;
                } else {
                    *Case = Case_l::UPPER;
                }
            } else {
                buffer[t_idx as usize] = *s;
            }
            t_idx = t_idx.wrapping_add(1);
            t_idx;
        }
        s = s.offset(1);
        s;
    }
    wchar_to_dos(toDos, buffer.as_mut_ptr(), out, t_idx, mangled);
}
#[no_mangle]
pub unsafe extern "C" fn dos_name(
    mut toDos: *mut doscp_t,
    mut name: *const i8,
    mut verbose: i32,
    mut mangled: *mut i32,
    mut dn: *mut dos_name_t,
) {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut ext: *mut i8 = 0 as *mut i8;
    let mut i: size_t = 0;
    let mut BaseCase: Case_t = Case_l::NONE;
    let mut ExtCase: Case_t = Case_l::UPPER;
    *mangled = 0 as i32;
    if *name.offset(0 as i32 as isize) as i32 != 0
        && *name.offset(1 as i32 as isize) as i32 == ':' as i32
    {
        name = &*name.offset(2 as i32 as isize) as *const i8;
    }
    name = mt_basename(name);
    s = strrchr(name, '\\' as i32);
    if !s.is_null() {
        name = s.offset(1 as i32 as isize);
    }
    memset(dn as *mut libc::c_void, ' ' as i32, 11 as i32 as u64);
    i = strspn(name, b". \0" as *const u8 as *const i8);
    if i != 0 {
        name = name.offset(i as isize);
        *mangled = 3 as i32;
    }
    ext = strrchr(name, '.' as i32);
    TranslateToDos(
        toDos,
        name,
        ((*dn).base).as_mut_ptr(),
        8 as i32 as size_t,
        ext,
        &mut BaseCase,
        mangled,
    );
    if !ext.is_null() {
        TranslateToDos(
            toDos,
            ext.offset(1 as i32 as isize),
            ((*dn).ext).as_mut_ptr(),
            3 as i32 as size_t,
            0 as *mut i8,
            &mut ExtCase,
            mangled,
        );
    }
    if *mangled & 2 as i32 != 0 {
        autorename_short(dn, 0 as i32);
    }
    if *mangled == 0 {
        if BaseCase as u32 == Case_l::LOWER as i32 as u32 {
            *mangled |= 0x8 as i32;
        }
        if ExtCase as u32 == Case_l::LOWER as i32 as u32 {
            *mangled |= 0x10 as i32;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn unix_name(
    mut dosCp: *mut doscp_t,
    mut base: *const i8,
    mut ext: *const i8,
    mut Case: uint8_t,
    mut ret: *mut wchar_t,
) -> *mut wchar_t {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut tname: [i8; 9] = [0; 9];
    let mut text: [i8; 4] = [0; 4];
    let mut ans: [i8; 13] = [0; 13];
    let mut i: i32 = 0;
    strncpy(tname.as_mut_ptr(), base, 8 as i32 as u64);
    tname[8 as i32 as usize] = '\0' as i32 as i8;
    s = strchr(tname.as_mut_ptr(), ' ' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as i8;
    }
    if tname[0 as i32 as usize] as i32 == '\u{5}' as i32 {
        tname[0 as i32 as usize] = -27i32 as i8;
    }
    if Case as i32 & (0x8 as i32 | 0x10 as i32) == 0 && mtools_ignore_short_case != 0 {
        Case = (Case as i32 | (0x8 as i32 | 0x10 as i32)) as uint8_t;
    }
    if Case as i32 & 0x8 as i32 != 0 {
        i = 0 as i32;
        while i < 8 as i32 && tname[i as usize] as i32 != 0 {
            tname[i as usize] = ch_tolower(tname[i as usize]);
            i += 1;
            i;
        }
    }
    strncpy(text.as_mut_ptr(), ext, 3 as i32 as u64);
    text[3 as i32 as usize] = '\0' as i32 as i8;
    s = strchr(text.as_mut_ptr(), ' ' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as i8;
    }
    if Case as i32 & 0x10 as i32 != 0 {
        i = 0 as i32;
        while i < 3 as i32 && text[i as usize] as i32 != 0 {
            text[i as usize] = ch_tolower(text[i as usize]);
            i += 1;
            i;
        }
    }
    if *text.as_mut_ptr() != 0 {
        strcpy(ans.as_mut_ptr(), tname.as_mut_ptr());
        strcat(ans.as_mut_ptr(), b".\0" as *const u8 as *const i8);
        strcat(ans.as_mut_ptr(), text.as_mut_ptr());
    } else {
        strcpy(ans.as_mut_ptr(), tname.as_mut_ptr());
    }
    dos_to_wchar(dosCp, ans.as_mut_ptr(), ret, 12 as i32 as size_t);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn isSpecial(mut name: *const i8) -> i32 {
    if *name.offset(0 as i32 as isize) as i32 == '\0' as i32 {
        return 1 as i32;
    }
    if strcmp(name, b".\0" as *const u8 as *const i8) == 0 {
        return 1 as i32;
    }
    if strcmp(name, b"..\0" as *const u8 as *const i8) == 0 {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn isSpecialW(mut name: *const wchar_t) -> i32 {
    if *name.offset(0 as i32 as isize) == '\0' as i32 {
        return 1 as i32;
    }
    if wcscmp(
        name,
        (*::core::mem::transmute::<&[u8; 8], &[i32; 2]>(b".\0\0\0\0\0\0\0")).as_ptr(),
    ) == 0
    {
        return 1 as i32;
    }
    if wcscmp(
        name,
        (*::core::mem::transmute::<&[u8; 12], &[i32; 3]>(b".\0\0\0.\0\0\0\0\0\0\0"))
            .as_ptr(),
    ) == 0
    {
        return 1 as i32;
    }
    return 0 as i32;
}