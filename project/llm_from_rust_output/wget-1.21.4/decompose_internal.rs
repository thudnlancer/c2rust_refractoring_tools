use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Ucs4WithCcc {
    pub code: u32,
    pub ccc: i32,
}

impl PartialOrd for Ucs4WithCcc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ucs4WithCcc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ccc.cmp(&other.ccc)
    }
}

fn merge(
    src1: &[Ucs4WithCcc],
    src2: &[Ucs4WithCcc],
    dst: &mut [Ucs4WithCcc],
) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < src1.len() && j < src2.len() {
        if src1[i] <= src2[j] {
            dst[k] = src1[i];
            i += 1;
        } else {
            dst[k] = src2[j];
            j += 1;
        }
        k += 1;
    }

    if i < src1.len() {
        dst[k..].copy_from_slice(&src1[i..]);
    } else if j < src2.len() {
        dst[k..].copy_from_slice(&src2[j..]);
    }
}

pub fn gl_uninorm_decompose_merge_sort_fromto(
    src: &[Ucs4WithCcc],
    dst: &mut [Ucs4WithCcc],
    tmp: &mut [Ucs4WithCcc],
) {
    match src.len() {
        0 => return,
        1 => {
            dst[0] = src[0];
        }
        2 => {
            if src[0] <= src[1] {
                dst[0] = src[0];
                dst[1] = src[1];
            } else {
                dst[0] = src[1];
                dst[1] = src[0];
            }
        }
        3 => {
            let mut sorted = [src[0], src[1], src[2]];
            sorted.sort();
            dst[0] = sorted[0];
            dst[1] = sorted[1];
            dst[2] = sorted[2];
        }
        n => {
            let n1 = n / 2;
            let n2 = (n + 1) / 2;
            
            gl_uninorm_decompose_merge_sort_fromto(&src[n1..], &mut dst[n1..], tmp);
            gl_uninorm_decompose_merge_sort_fromto(&src[..n1], tmp, dst);
            merge(&tmp[..n1], &dst[n1..n1+n2], dst);
        }
    }
}

pub fn gl_uninorm_decompose_merge_sort_inplace(
    src: &mut [Ucs4WithCcc],
    tmp: &mut [Ucs4WithCcc],
) {
    match src.len() {
        0 | 1 => return,
        2 => {
            if src[0] > src[1] {
                src.swap(0, 1);
            }
        }
        3 => {
            src.sort();
        }
        n => {
            let n1 = n / 2;
            let n2 = (n + 1) / 2;
            
            gl_uninorm_decompose_merge_sort_inplace(&mut src[n1..], tmp);
            gl_uninorm_decompose_merge_sort_fromto(&src[..n1], tmp, &mut src[..n1]);
            merge(&tmp[..n1], &src[n1..n1+n2], src);
        }
    }
}