use std::num::Wrapping;

pub type SizeT = u64;

pub fn xsum(size1: SizeT, size2: SizeT) -> SizeT {
    let sum = Wrapping(size1) + Wrapping(size2);
    if sum.0 >= size1 {
        sum.0
    } else {
        SizeT::MAX
    }
}

pub fn xsum3(size1: SizeT, size2: SizeT, size3: SizeT) -> SizeT {
    xsum(xsum(size1, size2), size3)
}

pub fn xsum4(size1: SizeT, size2: SizeT, size3: SizeT, size4: SizeT) -> SizeT {
    xsum(xsum(xsum(size1, size2), size3), size4)
}

pub fn xmax(size1: SizeT, size2: SizeT) -> SizeT {
    if size1 >= size2 {
        size1
    } else {
        size2
    }
}