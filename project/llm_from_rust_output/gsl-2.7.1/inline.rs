pub type SizeT = usize;

pub fn gsl_sf_legendre_array_index(l: SizeT, m: SizeT) -> SizeT {
    (l * (l + 1) / 2) + m
}