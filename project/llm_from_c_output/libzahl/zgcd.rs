use std::cmp::Ordering;

// Assuming the following types and functions are defined elsewhere in the Rust library:
// - z_t: A big integer type
// - zahl_char_t: The underlying character type for big integers
// - zzero: Checks if a big integer is zero
// - znegative2: Checks if either of two big integers is negative
// - zlsb: Gets the least significant bit position
// - zrsh: Right shifts a big integer
// - zcmpmag: Compares the magnitude of two big integers
// - zswap_tainted_unsigned: Swaps two unsigned big integers
// - zsub_positive_assign: Subtracts two big integers and assigns the result
// - zrsh_taint: Right shifts a big integer and marks it as tainted
// - zlsh: Left shifts a big integer
// - SET_SIGNUM: Sets the sign of a big integer
// - libzahl_tmp_gcd_u, libzahl_tmp_gcd_v: Temporary variables for GCD calculation

pub fn zgcd(a: &mut z_t, b: &z_t, c: &z_t) {
    /*
     * Binary GCD algorithm.
     */

    if zzero(b) {
        *a = c.clone();
        return;
    }
    if zzero(c) {
        *a = b.clone();
        return;
    }

    let neg = znegative2(b, c);

    let u_lsb = zlsb(b);
    let v_lsb = zlsb(c);
    let shifts = u_lsb.min(v_lsb);
    
    let mut u = libzahl_tmp_gcd_u.clone();
    let mut v = libzahl_tmp_gcd_v.clone();
    
    zrsh(&mut u, b, u_lsb);
    zrsh(&mut v, c, v_lsb);

    let u_orig = u.chars.clone();
    let v_orig = v.chars.clone();

    loop {
        let cmpmag = zcmpmag(&u, &v);
        if cmpmag >= 0 {
            if cmpmag == 0 {
                break;
            }
            zswap_tainted_unsigned(&mut u, &mut v);
        }
        zsub_positive_assign(&mut v, &u);
        zrsh_taint(&mut v, zlsb(&v));
    }

    zlsh(a, &u, shifts);
    SET_SIGNUM(a, if neg { -1 } else { 1 });

    u.chars = u_orig;
    v.chars = v_orig;
}