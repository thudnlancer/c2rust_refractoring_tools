use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

pub type GslComparisonFn<T> = fn(&T, &T) -> Ordering;

fn downheap<T>(
    p: &mut [usize],
    data: &[T],
    size: usize,
    n: usize,
    mut k: usize,
    compare: GslComparisonFn<T>,
) {
    let pki = p[k];
    while k <= n / 2 {
        let mut j = 2 * k;
        if j < n && compare(&data[p[j]], &data[p[j + 1]]) == Ordering::Less {
            j += 1;
        }
        if compare(&data[pki], &data[p[j]]) != Ordering::Less {
            break;
        }
        p[k] = p[j];
        k = j;
    }
    p[k] = pki;
}

pub fn gsl_heapsort_index<T>(
    p: &mut [usize],
    data: &[T],
    count: usize,
    size: usize,
    compare: GslComparisonFn<T>,
) -> Result<(), GslError> {
    if count == 0 {
        return Ok(());
    }

    for (i, item) in p.iter_mut().take(count).enumerate() {
        *item = i;
    }

    let mut n = count - 1;
    let mut k = n / 2 + 1;

    while k > 0 {
        k -= 1;
        downheap(p, data, size, n, k, compare);
    }

    while n > 0 {
        p.swap(0, n);
        n -= 1;
        downheap(p, data, size, n, 0, compare);
    }

    Ok(())
}