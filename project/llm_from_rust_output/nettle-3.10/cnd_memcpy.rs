use std::ptr;

pub type size_t = usize;

#[no_mangle]
pub fn nettle_cnd_memcpy(cnd: i32, dst: *mut libc::c_void, src: *const libc::c_void, n: size_t) {
    let sp = src as *const u8;
    let dp = dst as *mut u8;
    let m = if cnd != 0 { u8::MAX } else { 0 };

    for i in 0..n {
        unsafe {
            let src_byte = ptr::read_volatile(sp.add(i));
            let dst_byte = ptr::read_volatile(dp.add(i));
            let c = (src_byte & m) | (dst_byte & !m);
            ptr::write_volatile(dp.add(i), c);
        }
    }
}