use num_bigint::BigInt;
use num_traits::{Zero, One, FromPrimitive};
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Primality {
    Prime,
    NonPrime,
    ProbablyPrime,
}

fn zptest(witness: &mut Option<BigInt>, n: &BigInt, t: u32) -> Primality {
    /*
     * Miller–Rabin primality test.
     */

    if n <= &BigInt::from_u32(3).unwrap() {
        if n <= &BigInt::one() {
            if let Some(w) = witness {
                *w = n.clone();
            }
            return Primality::NonPrime;
        } else {
            return Primality::Prime;
        }
    }
    if n.is_even() {
        if let Some(w) = witness {
            *w = BigInt::from_u32(2).unwrap();
        }
        return Primality::NonPrime;
    }

    let n1 = n - BigInt::one();
    let n4 = n - BigInt::from_u32(4).unwrap();

    let r = n1.trailing_zeros().unwrap_or(0) as usize;
    let d = n1 >> r;

    let mut rng = rand::thread_rng();
    let mut a = BigInt::zero();
    let mut x = BigInt::zero();
    let mut t = t;

    while t > 0 {
        a = rng.gen_bigint_range(&BigInt::zero(), &n4);
        a += BigInt::from_u32(2).unwrap();

        x = a.modpow(&d, n);

        if x == BigInt::one() || x == n1 {
            t -= 1;
            continue;
        }

        let mut i = 1;
        while i < r {
            x = x.modpow(&BigInt::from_u32(2).unwrap(), n);
            if x == BigInt::one() {
                if let Some(w) = witness {
                    *w = a.clone();
                }
                return Primality::NonPrime;
            }
            if x == n1 {
                break;
            }
            i += 1;
        }
        if i == r {
            if let Some(w) = witness {
                *w = a.clone();
            }
            return Primality::NonPrime;
        }
        t -= 1;
    }

    Primality::ProbablyPrime
}