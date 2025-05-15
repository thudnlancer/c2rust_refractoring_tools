use std::mem;
use std::ptr;

type Byte = u8;
type uInt = u32;
type uLong = u64;
type Bytef = Byte;
type voidpf = *mut std::ffi::c_void;

struct InternalState;

struct ZStream {
    next_in: *mut Bytef,
    avail_in: uInt,
    total_in: uLong,
    next_out: *mut Bytef,
    avail_out: uInt,
    total_out: uLong,
    msg: *mut i8,
    state: *mut InternalState,
    zalloc: Option<extern "C" fn(voidpf, uInt, uInt) -> voidpf>,
    zfree: Option<extern "C" fn(voidpf, voidpf)>,
    opaque: voidpf,
    data_type: i32,
    adler: uLong,
    reserved: uLong,
}

struct GzHeader {
    text: i32,
    time: uLong,
    xflags: i32,
    os: i32,
    extra: *mut Bytef,
    extra_len: uInt,
    extra_max: uInt,
    name: *mut Bytef,
    name_max: uInt,
    comment: *mut Bytef,
    comm_max: uInt,
    hcrc: i32,
    done: i32,
}

#[repr(u32)]
enum InflateMode {
    Head = 0,
    Flags = 1,
    Time = 2,
    Os = 3,
    Exlen = 4,
    Extra = 5,
    Name = 6,
    Comment = 7,
    Hcrc = 8,
    Dictid = 9,
    Dict = 10,
    Type = 11,
    Typedo = 12,
    Stored = 13,
    Copy = 14,
    Copy_ = 15,
    Table = 16,
    Lenlens = 17,
    Codelens = 18,
    Len_ = 19,
    Len = 20,
    Lenext = 21,
    Dist = 22,
    Distext = 23,
    Match = 24,
    Lit = 25,
    Check = 26,
    Length = 27,
    Done = 28,
    Bad = 29,
    Mem = 30,
    Sync