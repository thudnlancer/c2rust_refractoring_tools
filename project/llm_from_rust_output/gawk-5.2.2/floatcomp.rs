use std::mem;

type Uintmax = u64;

fn count_trailing_zeros(n: Uintmax) -> i32 {
    n.trailing_zeros() as i32
}

#[no_mangle]
pub extern "C" fn adjust_uint(n: Uintmax) -> Uintmax {
    let wordbits = (8 * mem::size_of::<Uintmax>()) as i32;
    
    let float_bits = if mem::size_of::<f64>() == mem::size_of::<u128>() {
        64
    } else if mem::size_of::<f64>() == mem::size_of::<f64>() {
        53
    } else {
        24
    };
    
    let factor = if 2 == 2 { 1 } else { 4 };
    
    if float_bits * factor < wordbits {
        let one: Uintmax = 1;
        let sentinel = one << (wordbits - float_bits * factor);
        let shift = count_trailing_zeros(n | sentinel);
        let mask = ((one << float_bits) - 1) as Uintmax;
        n & (mask << shift)
    } else {
        n
    }
}