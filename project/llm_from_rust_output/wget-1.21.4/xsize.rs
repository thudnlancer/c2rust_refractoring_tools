pub type size_t = u64;

pub fn xsum(size1: size_t, size2: size_t) -> size_t {
    match size1.checked_add(size2) {
        Some(sum) => sum,
        None => size_t::MAX,
    }
}

pub fn xsum3(size1: size_t, size2: size_t, size3: size_t) -> size_t {
    xsum(xsum(size1, size2), size3)
}

pub fn xsum4(size1: size_t, size2: size_t, size3: size_t, size4: size_t) -> size_t {
    xsum(xsum(xsum(size1, size2), size3), size4)
}

pub fn xmax(size1: size_t, size2: size_t) -> size_t {
    size1.max(size2)
}