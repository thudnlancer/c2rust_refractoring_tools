#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cast128_ctx {
    pub rounds: u32,
    pub Kr: [u8; 16],
    pub Km: [uint32_t; 16],
}
static mut cast_sbox1: [uint32_t; 256] = [
    0x30fb40d4 as i32 as uint32_t,
    0x9fa0ff0b as u32,
    0x6beccd2f as i32 as uint32_t,
    0x3f258c7a as i32 as uint32_t,
    0x1e213f2f as i32 as uint32_t,
    0x9c004dd3 as u32,
    0x6003e540 as i32 as uint32_t,
    0xcf9fc949 as u32,
    0xbfd4af27 as u32,
    0x88bbbdb5 as u32,
    0xe2034090 as u32,
    0x98d09675 as u32,
    0x6e63a0e0 as i32 as uint32_t,
    0x15c361d2 as i32 as uint32_t,
    0xc2e7661d as u32,
    0x22d4ff8e as i32 as uint32_t,
    0x28683b6f as i32 as uint32_t,
    0xc07fd059 as u32,
    0xff2379c8 as u32,
    0x775f50e2 as i32 as uint32_t,
    0x43c340d3 as i32 as uint32_t,
    0xdf2f8656 as u32,
    0x887ca41a as u32,
    0xa2d2bd2d as u32,
    0xa1c9e0d6 as u32,
    0x346c4819 as i32 as uint32_t,
    0x61b76d87 as i32 as uint32_t,
    0x22540f2f as i32 as uint32_t,
    0x2abe32e1 as i32 as uint32_t,
    0xaa54166b as u32,
    0x22568e3a as i32 as uint32_t,
    0xa2d341d0 as u32,
    0x66db40c8 as i32 as uint32_t,
    0xa784392f as u32,
    0x4dff2f as i32 as uint32_t,
    0x2db9d2de as i32 as uint32_t,
    0x97943fac as u32,
    0x4a97c1d8 as i32 as uint32_t,
    0x527644b7 as i32 as uint32_t,
    0xb5f437a7 as u32,
    0xb82cbaef as u32,
    0xd751d159 as u32,
    0x6ff7f0ed as i32 as uint32_t,
    0x5a097a1f as i32 as uint32_t,
    0x827b68d0 as u32,
    0x90ecf52e as u32,
    0x22b0c054 as i32 as uint32_t,
    0xbc8e5935 as u32,
    0x4b6d2f7f as i32 as uint32_t,
    0x50bb64a2 as i32 as uint32_t,
    0xd2664910 as u32,
    0xbee5812d as u32,
    0xb7332290 as u32,
    0xe93b159f as u32,
    0xb48ee411 as u32,
    0x4bff345d as i32 as uint32_t,
    0xfd45c240 as u32,
    0xad31973f as u32,
    0xc4f6d02e as u32,
    0x55fc8165 as i32 as uint32_t,
    0xd5b1caad as u32,
    0xa1ac2dae as u32,
    0xa2d4b76d as u32,
    0xc19b0c50 as u32,
    0x882240f2 as u32,
    0xc6e4f38 as i32 as uint32_t,
    0xa4e4bfd7 as u32,
    0x4f5ba272 as i32 as uint32_t,
    0x564c1d2f as i32 as uint32_t,
    0xc59c5319 as u32,
    0xb949e354 as u32,
    0xb04669fe as u32,
    0xb1b6ab8a as u32,
    0xc71358dd as u32,
    0x6385c545 as i32 as uint32_t,
    0x110f935d as i32 as uint32_t,
    0x57538ad5 as i32 as uint32_t,
    0x6a390493 as i32 as uint32_t,
    0xe63d37e0 as u32,
    0x2a54f6b3 as i32 as uint32_t,
    0x3a787d5f as i32 as uint32_t,
    0x6276a0b5 as i32 as uint32_t,
    0x19a6fcdf as i32 as uint32_t,
    0x7a42206a as i32 as uint32_t,
    0x29f9d4d5 as i32 as uint32_t,
    0xf61b1891 as u32,
    0xbb72275e as u32,
    0xaa508167 as u32,
    0x38901091 as i32 as uint32_t,
    0xc6b505eb as u32,
    0x84c7cb8c as u32,
    0x2ad75a0f as i32 as uint32_t,
    0x874a1427 as u32,
    0xa2d1936b as u32,
    0x2ad286af as i32 as uint32_t,
    0xaa56d291 as u32,
    0xd7894360 as u32,
    0x425c750d as i32 as uint32_t,
    0x93b39e26 as u32,
    0x187184c9 as i32 as uint32_t,
    0x6c00b32d as i32 as uint32_t,
    0x73e2bb14 as i32 as uint32_t,
    0xa0bebc3c as u32,
    0x54623779 as i32 as uint32_t,
    0x64459eab as i32 as uint32_t,
    0x3f328b82 as i32 as uint32_t,
    0x7718cf82 as i32 as uint32_t,
    0x59a2cea6 as i32 as uint32_t,
    0x4ee002e as i32 as uint32_t,
    0x89fe78e6 as u32,
    0x3fab0950 as i32 as uint32_t,
    0x325ff6c2 as i32 as uint32_t,
    0x81383f05 as u32,
    0x6963c5c8 as i32 as uint32_t,
    0x76cb5ad6 as i32 as uint32_t,
    0xd49974c9 as u32,
    0xca180dcf as u32,
    0x380782d5 as i32 as uint32_t,
    0xc7fa5cf6 as u32,
    0x8ac31511 as u32,
    0x35e79e13 as i32 as uint32_t,
    0x47da91d0 as i32 as uint32_t,
    0xf40f9086 as u32,
    0xa7e2419e as u32,
    0x31366241 as i32 as uint32_t,
    0x51ef495 as i32 as uint32_t,
    0xaa573b04 as u32,
    0x4a805d8d as i32 as uint32_t,
    0x548300d0 as i32 as uint32_t,
    0x322a3c as i32 as uint32_t,
    0xbf64cddf as u32,
    0xba57a68e as u32,
    0x75c6372b as i32 as uint32_t,
    0x50afd341 as i32 as uint32_t,
    0xa7c13275 as u32,
    0x915a0bf5 as u32,
    0x6b54bfab as i32 as uint32_t,
    0x2b0b1426 as i32 as uint32_t,
    0xab4cc9d7 as u32,
    0x449ccd82 as i32 as uint32_t,
    0xf7fbf265 as u32,
    0xab85c5f3 as u32,
    0x1b55db94 as i32 as uint32_t,
    0xaad4e324 as u32,
    0xcfa4bd3f as u32,
    0x2deaa3e2 as i32 as uint32_t,
    0x9e204d02 as u32,
    0xc8bd25ac as u32,
    0xeadf55b3 as u32,
    0xd5bd9e98 as u32,
    0xe31231b2 as u32,
    0x2ad5ad6c as i32 as uint32_t,
    0x954329de as u32,
    0xadbe4528 as u32,
    0xd8710f69 as u32,
    0xaa51c90f as u32,
    0xaa786bf6 as u32,
    0x22513f1e as i32 as uint32_t,
    0xaa51a79b as u32,
    0x2ad344cc as i32 as uint32_t,
    0x7b5a41f0 as i32 as uint32_t,
    0xd37cfbad as u32,
    0x1b069505 as i32 as uint32_t,
    0x41ece491 as i32 as uint32_t,
    0xb4c332e6 as u32,
    0x32268d4 as i32 as uint32_t,
    0xc9600acc as u32,
    0xce387e6d as u32,
    0xbf6bb16c as u32,
    0x6a70fb78 as i32 as uint32_t,
    0xd03d9c9 as i32 as uint32_t,
    0xd4df39de as u32,
    0xe01063da as u32,
    0x4736f464 as i32 as uint32_t,
    0x5ad328d8 as i32 as uint32_t,
    0xb347cc96 as u32,
    0x75bb0fc3 as i32 as uint32_t,
    0x98511bfb as u32,
    0x4ffbcc35 as i32 as uint32_t,
    0xb58bcf6a as u32,
    0xe11f0abc as u32,
    0xbfc5fe4a as u32,
    0xa70aec10 as u32,
    0xac39570a as u32,
    0x3f04442f as i32 as uint32_t,
    0x6188b153 as i32 as uint32_t,
    0xe0397a2e as u32,
    0x5727cb79 as i32 as uint32_t,
    0x9ceb418f as u32,
    0x1cacd68d as i32 as uint32_t,
    0x2ad37c96 as i32 as uint32_t,
    0x175cb9d as i32 as uint32_t,
    0xc69dff09 as u32,
    0xc75b65f0 as u32,
    0xd9db40d8 as u32,
    0xec0e7779 as u32,
    0x4744ead4 as i32 as uint32_t,
    0xb11c3274 as u32,
    0xdd24cb9e as u32,
    0x7e1c54bd as i32 as uint32_t,
    0xf01144f9 as u32,
    0xd2240eb1 as u32,
    0x9675b3fd as u32,
    0xa3ac3755 as u32,
    0xd47c27af as u32,
    0x51c85f4d as i32 as uint32_t,
    0x56907596 as i32 as uint32_t,
    0xa5bb15e6 as u32,
    0x580304f0 as i32 as uint32_t,
    0xca042cf1 as u32,
    0x11a37ea as i32 as uint32_t,
    0x8dbfaadb as u32,
    0x35ba3e4a as i32 as uint32_t,
    0x3526ffa0 as i32 as uint32_t,
    0xc37b4d09 as u32,
    0xbc306ed9 as u32,
    0x98a52666 as u32,
    0x5648f725 as i32 as uint32_t,
    0xff5e569d as u32,
    0xced63d0 as i32 as uint32_t,
    0x7c63b2cf as i32 as uint32_t,
    0x700b45e1 as i32 as uint32_t,
    0xd5ea50f1 as u32,
    0x85a92872 as u32,
    0xaf1fbda7 as u32,
    0xd4234870 as u32,
    0xa7870bf3 as u32,
    0x2d3b4d79 as i32 as uint32_t,
    0x42e04198 as i32 as uint32_t,
    0xcd0ede7 as i32 as uint32_t,
    0x26470db8 as i32 as uint32_t,
    0xf881814c as u32,
    0x474d6ad7 as i32 as uint32_t,
    0x7c0c5e5c as i32 as uint32_t,
    0xd1231959 as u32,
    0x381b7298 as i32 as uint32_t,
    0xf5d2f4db as u32,
    0xab838653 as u32,
    0x6e2f1e23 as i32 as uint32_t,
    0x83719c9e as u32,
    0xbd91e046 as u32,
    0x9a56456e as u32,
    0xdc39200c as u32,
    0x20c8c571 as i32 as uint32_t,
    0x962bda1c as u32,
    0xe1e696ff as u32,
    0xb141ab08 as u32,
    0x7cca89b9 as i32 as uint32_t,
    0x1a69e783 as i32 as uint32_t,
    0x2cc4843 as i32 as uint32_t,
    0xa2f7c579 as u32,
    0x429ef47d as i32 as uint32_t,
    0x427b169c as i32 as uint32_t,
    0x5ac9f049 as i32 as uint32_t,
    0xdd8f0f00 as u32,
    0x5c8165bf as i32 as uint32_t,
];
static mut cast_sbox2: [uint32_t; 256] = [
    0x1f201094 as i32 as uint32_t,
    0xef0ba75b as u32,
    0x69e3cf7e as i32 as uint32_t,
    0x393f4380 as i32 as uint32_t,
    0xfe61cf7a as u32,
    0xeec5207a as u32,
    0x55889c94 as i32 as uint32_t,
    0x72fc0651 as i32 as uint32_t,
    0xada7ef79 as u32,
    0x4e1d7235 as i32 as uint32_t,
    0xd55a63ce as u32,
    0xde0436ba as u32,
    0x99c430ef as u32,
    0x5f0c0794 as i32 as uint32_t,
    0x18dcdb7d as i32 as uint32_t,
    0xa1d6eff3 as u32,
    0xa0b52f7b as u32,
    0x59e83605 as i32 as uint32_t,
    0xee15b094 as u32,
    0xe9ffd909 as u32,
    0xdc440086 as u32,
    0xef944459 as u32,
    0xba83ccb3 as u32,
    0xe0c3cdfb as u32,
    0xd1da4181 as u32,
    0x3b092ab1 as i32 as uint32_t,
    0xf997f1c1 as u32,
    0xa5e6cf7b as u32,
    0x1420ddb as i32 as uint32_t,
    0xe4e7ef5b as u32,
    0x25a1ff41 as i32 as uint32_t,
    0xe180f806 as u32,
    0x1fc41080 as i32 as uint32_t,
    0x179bee7a as i32 as uint32_t,
    0xd37ac6a9 as u32,
    0xfe5830a4 as u32,
    0x98de8b7f as u32,
    0x77e83f4e as i32 as uint32_t,
    0x79929269 as i32 as uint32_t,
    0x24fa9f7b as i32 as uint32_t,
    0xe113c85b as u32,
    0xacc40083 as u32,
    0xd7503525 as u32,
    0xf7ea615f as u32,
    0x62143154 as i32 as uint32_t,
    0xd554b63 as i32 as uint32_t,
    0x5d681121 as i32 as uint32_t,
    0xc866c359 as u32,
    0x3d63cf73 as i32 as uint32_t,
    0xcee234c0 as u32,
    0xd4d87e87 as u32,
    0x5c672b21 as i32 as uint32_t,
    0x71f6181 as i32 as uint32_t,
    0x39f7627f as i32 as uint32_t,
    0x361e3084 as i32 as uint32_t,
    0xe4eb573b as u32,
    0x602f64a4 as i32 as uint32_t,
    0xd63acd9c as u32,
    0x1bbc4635 as i32 as uint32_t,
    0x9e81032d as u32,
    0x2701f50c as i32 as uint32_t,
    0x99847ab4 as u32,
    0xa0e3df79 as u32,
    0xba6cf38c as u32,
    0x10843094 as i32 as uint32_t,
    0x2537a95e as i32 as uint32_t,
    0xf46f6ffe as u32,
    0xa1ff3b1f as u32,
    0x208cfb6a as i32 as uint32_t,
    0x8f458c74 as u32,
    0xd9e0a227 as u32,
    0x4ec73a34 as i32 as uint32_t,
    0xfc884f69 as u32,
    0x3e4de8df as i32 as uint32_t,
    0xef0e0088 as u32,
    0x3559648d as i32 as uint32_t,
    0x8a45388c as u32,
    0x1d804366 as i32 as uint32_t,
    0x721d9bfd as i32 as uint32_t,
    0xa58684bb as u32,
    0xe8256333 as u32,
    0x844e8212 as u32,
    0x128d8098 as i32 as uint32_t,
    0xfed33fb4 as u32,
    0xce280ae1 as u32,
    0x27e19ba5 as i32 as uint32_t,
    0xd5a6c252 as u32,
    0xe49754bd as u32,
    0xc5d655dd as u32,
    0xeb667064 as u32,
    0x77840b4d as i32 as uint32_t,
    0xa1b6a801 as u32,
    0x84db26a9 as u32,
    0xe0b56714 as u32,
    0x21f043b7 as i32 as uint32_t,
    0xe5d05860 as u32,
    0x54f03084 as i32 as uint32_t,
    0x66ff472 as i32 as uint32_t,
    0xa31aa153 as u32,
    0xdadc4755 as u32,
    0xb5625dbf as u32,
    0x68561be6 as i32 as uint32_t,
    0x83ca6b94 as u32,
    0x2d6ed23b as i32 as uint32_t,
    0xeccf01db as u32,
    0xa6d3d0ba as u32,
    0xb6803d5c as u32,
    0xaf77a709 as u32,
    0x33b4a34c as i32 as uint32_t,
    0x397bc8d6 as i32 as uint32_t,
    0x5ee22b95 as i32 as uint32_t,
    0x5f0e5304 as i32 as uint32_t,
    0x81ed6f61 as u32,
    0x20e74364 as i32 as uint32_t,
    0xb45e1378 as u32,
    0xde18639b as u32,
    0x881ca122 as u32,
    0xb96726d1 as u32,
    0x8049a7e8 as u32,
    0x22b7da7b as i32 as uint32_t,
    0x5e552d25 as i32 as uint32_t,
    0x5272d237 as i32 as uint32_t,
    0x79d2951c as i32 as uint32_t,
    0xc60d894c as u32,
    0x488cb402 as i32 as uint32_t,
    0x1ba4fe5b as i32 as uint32_t,
    0xa4b09f6b as u32,
    0x1ca815cf as i32 as uint32_t,
    0xa20c3005 as u32,
    0x8871df63 as u32,
    0xb9de2fcb as u32,
    0xcc6c9e9 as i32 as uint32_t,
    0xbeeff53 as i32 as uint32_t,
    0xe3214517 as u32,
    0xb4542835 as u32,
    0x9f63293c as u32,
    0xee41e729 as u32,
    0x6e1d2d7c as i32 as uint32_t,
    0x50045286 as i32 as uint32_t,
    0x1e6685f3 as i32 as uint32_t,
    0xf33401c6 as u32,
    0x30a22c95 as i32 as uint32_t,
    0x31a70850 as i32 as uint32_t,
    0x60930f13 as i32 as uint32_t,
    0x73f98417 as i32 as uint32_t,
    0xa1269859 as u32,
    0xec645c44 as u32,
    0x52c877a9 as i32 as uint32_t,
    0xcdff33a6 as u32,
    0xa02b1741 as u32,
    0x7cbad9a2 as i32 as uint32_t,
    0x2180036f as i32 as uint32_t,
    0x50d99c08 as i32 as uint32_t,
    0xcb3f4861 as u32,
    0xc26bd765 as u32,
    0x64a3f6ab as i32 as uint32_t,
    0x80342676 as u32,
    0x25a75e7b as i32 as uint32_t,
    0xe4e6d1fc as u32,
    0x20c710e6 as i32 as uint32_t,
    0xcdf0b680 as u32,
    0x17844d3b as i32 as uint32_t,
    0x31eef84d as i32 as uint32_t,
    0x7e0824e4 as i32 as uint32_t,
    0x2ccb49eb as i32 as uint32_t,
    0x846a3bae as u32,
    0x8ff77888 as u32,
    0xee5d60f6 as u32,
    0x7af75673 as i32 as uint32_t,
    0x2fdd5cdb as i32 as uint32_t,
    0xa11631c1 as u32,
    0x30f66f43 as i32 as uint32_t,
    0xb3faec54 as u32,
    0x157fd7fa as i32 as uint32_t,
    0xef8579cc as u32,
    0xd152de58 as u32,
    0xdb2ffd5e as u32,
    0x8f32ce19 as u32,
    0x306af97a as i32 as uint32_t,
    0x2f03ef8 as i32 as uint32_t,
    0x99319ad5 as u32,
    0xc242fa0f as u32,
    0xa7e3ebb0 as u32,
    0xc68e4906 as u32,
    0xb8da230c as u32,
    0x80823028 as u32,
    0xdcdef3c8 as u32,
    0xd35fb171 as u32,
    0x88a1bc8 as i32 as uint32_t,
    0xbec0c560 as u32,
    0x61a3c9e8 as i32 as uint32_t,
    0xbca8f54d as u32,
    0xc72feffa as u32,
    0x22822e99 as i32 as uint32_t,
    0x82c570b4 as u32,
    0xd8d94e89 as u32,
    0x8b1c34bc as u32,
    0x301e16e6 as i32 as uint32_t,
    0x273be979 as i32 as uint32_t,
    0xb0ffeaa6 as u32,
    0x61d9b8c6 as i32 as uint32_t,
    0xb24869 as i32 as uint32_t,
    0xb7ffce3f as u32,
    0x8dc283b as i32 as uint32_t,
    0x43daf65a as i32 as uint32_t,
    0xf7e19798 as u32,
    0x7619b72f as i32 as uint32_t,
    0x8f1c9ba4 as u32,
    0xdc8637a0 as u32,
    0x16a7d3b1 as i32 as uint32_t,
    0x9fc393b7 as u32,
    0xa7136eeb as u32,
    0xc6bcc63e as u32,
    0x1a513742 as i32 as uint32_t,
    0xef6828bc as u32,
    0x520365d6 as i32 as uint32_t,
    0x2d6a77ab as i32 as uint32_t,
    0x3527ed4b as i32 as uint32_t,
    0x821fd216 as u32,
    0x95c6e2e as i32 as uint32_t,
    0xdb92f2fb as u32,
    0x5eea29cb as i32 as uint32_t,
    0x145892f5 as i32 as uint32_t,
    0x91584f7f as u32,
    0x5483697b as i32 as uint32_t,
    0x2667a8cc as i32 as uint32_t,
    0x85196048 as u32,
    0x8c4bacea as u32,
    0x833860d4 as u32,
    0xd23e0f9 as i32 as uint32_t,
    0x6c387e8a as i32 as uint32_t,
    0xae6d249 as i32 as uint32_t,
    0xb284600c as u32,
    0xd835731d as u32,
    0xdcb1c647 as u32,
    0xac4c56ea as u32,
    0x3ebd81b3 as i32 as uint32_t,
    0x230eabb0 as i32 as uint32_t,
    0x6438bc87 as i32 as uint32_t,
    0xf0b5b1fa as u32,
    0x8f5ea2b3 as u32,
    0xfc184642 as u32,
    0xa036b7a as i32 as uint32_t,
    0x4fb089bd as i32 as uint32_t,
    0x649da589 as i32 as uint32_t,
    0xa345415e as u32,
    0x5c038323 as i32 as uint32_t,
    0x3e5d3bb9 as i32 as uint32_t,
    0x43d79572 as i32 as uint32_t,
    0x7e6dd07c as i32 as uint32_t,
    0x6dfdf1e as i32 as uint32_t,
    0x6c6cc4ef as i32 as uint32_t,
    0x7160a539 as i32 as uint32_t,
    0x73bfbe70 as i32 as uint32_t,
    0x83877605 as u32,
    0x4523ecf1 as i32 as uint32_t,
];
static mut cast_sbox3: [uint32_t; 256] = [
    0x8defc240 as u32,
    0x25fa5d9f as i32 as uint32_t,
    0xeb903dbf as u32,
    0xe810c907 as u32,
    0x47607fff as i32 as uint32_t,
    0x369fe44b as i32 as uint32_t,
    0x8c1fc644 as u32,
    0xaececa90 as u32,
    0xbeb1f9bf as u32,
    0xeefbcaea as u32,
    0xe8cf1950 as u32,
    0x51df07ae as i32 as uint32_t,
    0x920e8806 as u32,
    0xf0ad0548 as u32,
    0xe13c8d83 as u32,
    0x927010d5 as u32,
    0x11107d9f as i32 as uint32_t,
    0x7647db9 as i32 as uint32_t,
    0xb2e3e4d4 as u32,
    0x3d4f285e as i32 as uint32_t,
    0xb9afa820 as u32,
    0xfade82e0 as u32,
    0xa067268b as u32,
    0x8272792e as u32,
    0x553fb2c0 as i32 as uint32_t,
    0x489ae22b as i32 as uint32_t,
    0xd4ef9794 as u32,
    0x125e3fbc as i32 as uint32_t,
    0x21fffcee as i32 as uint32_t,
    0x825b1bfd as u32,
    0x9255c5ed as u32,
    0x1257a240 as i32 as uint32_t,
    0x4e1a8302 as i32 as uint32_t,
    0xbae07fff as u32,
    0x528246e7 as i32 as uint32_t,
    0x8e57140e as u32,
    0x3373f7bf as i32 as uint32_t,
    0x8c9f8188 as u32,
    0xa6fc4ee8 as u32,
    0xc982b5a5 as u32,
    0xa8c01db7 as u32,
    0x579fc264 as i32 as uint32_t,
    0x67094f31 as i32 as uint32_t,
    0xf2bd3f5f as u32,
    0x40fff7c1 as i32 as uint32_t,
    0x1fb78dfc as i32 as uint32_t,
    0x8e6bd2c1 as u32,
    0x437be59b as i32 as uint32_t,
    0x99b03dbf as u32,
    0xb5dbc64b as u32,
    0x638dc0e6 as i32 as uint32_t,
    0x55819d99 as i32 as uint32_t,
    0xa197c81c as u32,
    0x4a012d6e as i32 as uint32_t,
    0xc5884a28 as u32,
    0xccc36f71 as u32,
    0xb843c213 as u32,
    0x6c0743f1 as i32 as uint32_t,
    0x8309893c as u32,
    0xfeddd5f as i32 as uint32_t,
    0x2f7fe850 as i32 as uint32_t,
    0xd7c07f7e as u32,
    0x2507fbf as i32 as uint32_t,
    0x5afb9a04 as i32 as uint32_t,
    0xa747d2d0 as u32,
    0x1651192e as i32 as uint32_t,
    0xaf70bf3e as u32,
    0x58c31380 as i32 as uint32_t,
    0x5f98302e as i32 as uint32_t,
    0x727cc3c4 as i32 as uint32_t,
    0xa0fb402 as i32 as uint32_t,
    0xf7fef82 as i32 as uint32_t,
    0x8c96fdad as u32,
    0x5d2c2aae as i32 as uint32_t,
    0x8ee99a49 as u32,
    0x50da88b8 as i32 as uint32_t,
    0x8427f4a0 as u32,
    0x1eac5790 as i32 as uint32_t,
    0x796fb449 as i32 as uint32_t,
    0x8252dc15 as u32,
    0xefbd7d9b as u32,
    0xa672597d as u32,
    0xada840d8 as u32,
    0x45f54504 as i32 as uint32_t,
    0xfa5d7403 as u32,
    0xe83ec305 as u32,
    0x4f91751a as i32 as uint32_t,
    0x925669c2 as u32,
    0x23efe941 as i32 as uint32_t,
    0xa903f12e as u32,
    0x60270df2 as i32 as uint32_t,
    0x276e4b6 as i32 as uint32_t,
    0x94fd6574 as u32,
    0x927985b2 as u32,
    0x8276dbcb as u32,
    0x2778176 as i32 as uint32_t,
    0xf8af918d as u32,
    0x4e48f79e as i32 as uint32_t,
    0x8f616ddf as u32,
    0xe29d840e as u32,
    0x842f7d83 as u32,
    0x340ce5c8 as i32 as uint32_t,
    0x96bbb682 as u32,
    0x93b4b148 as u32,
    0xef303cab as u32,
    0x984faf28 as u32,
    0x779faf9b as i32 as uint32_t,
    0x92dc560d as u32,
    0x224d1e20 as i32 as uint32_t,
    0x8437aa88 as u32,
    0x7d29dc96 as i32 as uint32_t,
    0x2756d3dc as i32 as uint32_t,
    0x8b907cee as u32,
    0xb51fd240 as u32,
    0xe7c07ce3 as u32,
    0xe566b4a1 as u32,
    0xc3e9615e as u32,
    0x3cf8209d as i32 as uint32_t,
    0x6094d1e3 as i32 as uint32_t,
    0xcd9ca341 as u32,
    0x5c76460e as i32 as uint32_t,
    0xea983b as i32 as uint32_t,
    0xd4d67881 as u32,
    0xfd47572c as u32,
    0xf76cedd9 as u32,
    0xbda8229c as u32,
    0x127dadaa as i32 as uint32_t,
    0x438a074e as i32 as uint32_t,
    0x1f97c090 as i32 as uint32_t,
    0x81bdb8a as i32 as uint32_t,
    0x93a07ebe as u32,
    0xb938ca15 as u32,
    0x97b03cff as u32,
    0x3dc2c0f8 as i32 as uint32_t,
    0x8d1ab2ec as u32,
    0x64380e51 as i32 as uint32_t,
    0x68cc7bfb as i32 as uint32_t,
    0xd90f2788 as u32,
    0x12490181 as i32 as uint32_t,
    0x5de5ffd4 as i32 as uint32_t,
    0xdd7ef86a as u32,
    0x76a2e214 as i32 as uint32_t,
    0xb9a40368 as u32,
    0x925d958f as u32,
    0x4b39fffa as i32 as uint32_t,
    0xba39aee9 as u32,
    0xa4ffd30b as u32,
    0xfaf7933b as u32,
    0x6d498623 as i32 as uint32_t,
    0x193cbcfa as i32 as uint32_t,
    0x27627545 as i32 as uint32_t,
    0x825cf47a as u32,
    0x61bd8ba0 as i32 as uint32_t,
    0xd11e42d1 as u32,
    0xcead04f4 as u32,
    0x127ea392 as i32 as uint32_t,
    0x10428db7 as i32 as uint32_t,
    0x8272a972 as u32,
    0x9270c4a8 as u32,
    0x127de50b as i32 as uint32_t,
    0x285ba1c8 as i32 as uint32_t,
    0x3c62f44f as i32 as uint32_t,
    0x35c0eaa5 as i32 as uint32_t,
    0xe805d231 as u32,
    0x428929fb as i32 as uint32_t,
    0xb4fcdf82 as u32,
    0x4fb66a53 as i32 as uint32_t,
    0xe7dc15b as i32 as uint32_t,
    0x1f081fab as i32 as uint32_t,
    0x108618ae as i32 as uint32_t,
    0xfcfd086d as u32,
    0xf9ff2889 as u32,
    0x694bcc11 as i32 as uint32_t,
    0x236a5cae as i32 as uint32_t,
    0x12deca4d as i32 as uint32_t,
    0x2c3f8cc5 as i32 as uint32_t,
    0xd2d02dfe as u32,
    0xf8ef5896 as u32,
    0xe4cf52da as u32,
    0x95155b67 as u32,
    0x494a488c as i32 as uint32_t,
    0xb9b6a80c as u32,
    0x5c8f82bc as i32 as uint32_t,
    0x89d36b45 as u32,
    0x3a609437 as i32 as uint32_t,
    0xec00c9a9 as u32,
    0x44715253 as i32 as uint32_t,
    0xa874b49 as i32 as uint32_t,
    0xd773bc40 as u32,
    0x7c34671c as i32 as uint32_t,
    0x2717ef6 as i32 as uint32_t,
    0x4feb5536 as i32 as uint32_t,
    0xa2d02fff as u32,
    0xd2bf60c4 as u32,
    0xd43f03c0 as u32,
    0x50b4ef6d as i32 as uint32_t,
    0x7478cd1 as i32 as uint32_t,
    0x6e1888 as i32 as uint32_t,
    0xa2e53f55 as u32,
    0xb9e6d4bc as u32,
    0xa2048016 as u32,
    0x97573833 as u32,
    0xd7207d67 as u32,
    0xde0f8f3d as u32,
    0x72f87b33 as i32 as uint32_t,
    0xabcc4f33 as u32,
    0x7688c55d as i32 as uint32_t,
    0x7b00a6b0 as i32 as uint32_t,
    0x947b0001 as u32,
    0x570075d2 as i32 as uint32_t,
    0xf9bb88f8 as u32,
    0x8942019e as u32,
    0x4264a5ff as i32 as uint32_t,
    0x856302e0 as u32,
    0x72dbd92b as i32 as uint32_t,
    0xee971b69 as u32,
    0x6ea22fde as i32 as uint32_t,
    0x5f08ae2b as i32 as uint32_t,
    0xaf7a616d as u32,
    0xe5c98767 as u32,
    0xcf1febd2 as u32,
    0x61efc8c2 as i32 as uint32_t,
    0xf1ac2571 as u32,
    0xcc8239c2 as u32,
    0x67214cb8 as i32 as uint32_t,
    0xb1e583d1 as u32,
    0xb7dc3e62 as u32,
    0x7f10bdce as i32 as uint32_t,
    0xf90a5c38 as u32,
    0xff0443d as i32 as uint32_t,
    0x606e6dc6 as i32 as uint32_t,
    0x60543a49 as i32 as uint32_t,
    0x5727c148 as i32 as uint32_t,
    0x2be98a1d as i32 as uint32_t,
    0x8ab41738 as u32,
    0x20e1be24 as i32 as uint32_t,
    0xaf96da0f as u32,
    0x68458425 as i32 as uint32_t,
    0x99833be5 as u32,
    0x600d457d as i32 as uint32_t,
    0x282f9350 as i32 as uint32_t,
    0x8334b362 as u32,
    0xd91d1120 as u32,
    0x2b6d8da0 as i32 as uint32_t,
    0x642b1e31 as i32 as uint32_t,
    0x9c305a00 as u32,
    0x52bce688 as i32 as uint32_t,
    0x1b03588a as i32 as uint32_t,
    0xf7baefd5 as u32,
    0x4142ed9c as i32 as uint32_t,
    0xa4315c11 as u32,
    0x83323ec5 as u32,
    0xdfef4636 as u32,
    0xa133c501 as u32,
    0xe9d3531c as u32,
    0xee353783 as u32,
];
static mut cast_sbox4: [uint32_t; 256] = [
    0x9db30420 as u32,
    0x1fb6e9de as i32 as uint32_t,
    0xa7be7bef as u32,
    0xd273a298 as u32,
    0x4a4f7bdb as i32 as uint32_t,
    0x64ad8c57 as i32 as uint32_t,
    0x85510443 as u32,
    0xfa020ed1 as u32,
    0x7e287aff as i32 as uint32_t,
    0xe60fb663 as u32,
    0x95f35a1 as i32 as uint32_t,
    0x79ebf120 as i32 as uint32_t,
    0xfd059d43 as u32,
    0x6497b7b1 as i32 as uint32_t,
    0xf3641f63 as u32,
    0x241e4adf as i32 as uint32_t,
    0x28147f5f as i32 as uint32_t,
    0x4fa2b8cd as i32 as uint32_t,
    0xc9430040 as u32,
    0xcc32220 as i32 as uint32_t,
    0xfdd30b30 as u32,
    0xc0a5374f as u32,
    0x1d2d00d9 as i32 as uint32_t,
    0x24147b15 as i32 as uint32_t,
    0xee4d111a as u32,
    0xfca5167 as i32 as uint32_t,
    0x71ff904c as i32 as uint32_t,
    0x2d195ffe as i32 as uint32_t,
    0x1a05645f as i32 as uint32_t,
    0xc13fefe as i32 as uint32_t,
    0x81b08ca as i32 as uint32_t,
    0x5170121 as i32 as uint32_t,
    0x80530100 as u32,
    0xe83e5efe as u32,
    0xac9af4f8 as u32,
    0x7fe72701 as i32 as uint32_t,
    0xd2b8ee5f as u32,
    0x6df4261 as i32 as uint32_t,
    0xbb9e9b8a as u32,
    0x7293ea25 as i32 as uint32_t,
    0xce84ffdf as u32,
    0xf5718801 as u32,
    0x3dd64b04 as i32 as uint32_t,
    0xa26f263b as u32,
    0x7ed48400 as i32 as uint32_t,
    0x547eebe6 as i32 as uint32_t,
    0x446d4ca0 as i32 as uint32_t,
    0x6cf3d6f5 as i32 as uint32_t,
    0x2649abdf as i32 as uint32_t,
    0xaea0c7f5 as u32,
    0x36338cc1 as i32 as uint32_t,
    0x503f7e93 as i32 as uint32_t,
    0xd3772061 as u32,
    0x11b638e1 as i32 as uint32_t,
    0x72500e03 as i32 as uint32_t,
    0xf80eb2bb as u32,
    0xabe0502e as u32,
    0xec8d77de as u32,
    0x57971e81 as i32 as uint32_t,
    0xe14f6746 as u32,
    0xc9335400 as u32,
    0x6920318f as i32 as uint32_t,
    0x81dbb99 as i32 as uint32_t,
    0xffc304a5 as u32,
    0x4d351805 as i32 as uint32_t,
    0x7f3d5ce3 as i32 as uint32_t,
    0xa6c866c6 as u32,
    0x5d5bcca9 as i32 as uint32_t,
    0xdaec6fea as u32,
    0x9f926f91 as u32,
    0x9f46222f as u32,
    0x3991467d as i32 as uint32_t,
    0xa5bf6d8e as u32,
    0x1143c44f as i32 as uint32_t,
    0x43958302 as i32 as uint32_t,
    0xd0214eeb as u32,
    0x22083b8 as i32 as uint32_t,
    0x3fb6180c as i32 as uint32_t,
    0x18f8931e as i32 as uint32_t,
    0x281658e6 as i32 as uint32_t,
    0x26486e3e as i32 as uint32_t,
    0x8bd78a70 as u32,
    0x7477e4c1 as i32 as uint32_t,
    0xb506e07c as u32,
    0xf32d0a25 as u32,
    0x79098b02 as i32 as uint32_t,
    0xe4eabb81 as u32,
    0x28123b23 as i32 as uint32_t,
    0x69dead38 as i32 as uint32_t,
    0x1574ca16 as i32 as uint32_t,
    0xdf871b62 as u32,
    0x211c40b7 as i32 as uint32_t,
    0xa51a9ef9 as u32,
    0x14377b as i32 as uint32_t,
    0x41e8ac8 as i32 as uint32_t,
    0x9114003 as i32 as uint32_t,
    0xbd59e4d2 as u32,
    0xe3d156d5 as u32,
    0x4fe876d5 as i32 as uint32_t,
    0x2f91a340 as i32 as uint32_t,
    0x557be8de as i32 as uint32_t,
    0xeae4a7 as i32 as uint32_t,
    0xce5c2ec as i32 as uint32_t,
    0x4db4bba6 as i32 as uint32_t,
    0xe756bdff as u32,
    0xdd3369ac as u32,
    0xec17b035 as u32,
    0x6572327 as i32 as uint32_t,
    0x99afc8b0 as u32,
    0x56c8c391 as i32 as uint32_t,
    0x6b65811c as i32 as uint32_t,
    0x5e146119 as i32 as uint32_t,
    0x6e85cb75 as i32 as uint32_t,
    0xbe07c002 as u32,
    0xc2325577 as u32,
    0x893ff4ec as u32,
    0x5bbfc92d as i32 as uint32_t,
    0xd0ec3b25 as u32,
    0xb7801ab7 as u32,
    0x8d6d3b24 as u32,
    0x20c763ef as i32 as uint32_t,
    0xc366a5fc as u32,
    0x9c382880 as u32,
    0xace3205 as i32 as uint32_t,
    0xaac9548a as u32,
    0xeca1d7c7 as u32,
    0x41afa32 as i32 as uint32_t,
    0x1d16625a as i32 as uint32_t,
    0x6701902c as i32 as uint32_t,
    0x9b757a54 as u32,
    0x31d477f7 as i32 as uint32_t,
    0x9126b031 as u32,
    0x36cc6fdb as i32 as uint32_t,
    0xc70b8b46 as u32,
    0xd9e66a48 as u32,
    0x56e55a79 as i32 as uint32_t,
    0x26a4ceb as i32 as uint32_t,
    0x52437eff as i32 as uint32_t,
    0x2f8f76b4 as i32 as uint32_t,
    0xdf980a5 as i32 as uint32_t,
    0x8674cde3 as u32,
    0xedda04eb as u32,
    0x17a9be04 as i32 as uint32_t,
    0x2c18f4df as i32 as uint32_t,
    0xb7747f9d as u32,
    0xab2af7b4 as u32,
    0xefc34d20 as u32,
    0x2e096b7c as i32 as uint32_t,
    0x1741a254 as i32 as uint32_t,
    0xe5b6a035 as u32,
    0x213d42f6 as i32 as uint32_t,
    0x2c1c7c26 as i32 as uint32_t,
    0x61c2f50f as i32 as uint32_t,
    0x6552daf9 as i32 as uint32_t,
    0xd2c231f8 as u32,
    0x25130f69 as i32 as uint32_t,
    0xd8167fa2 as u32,
    0x418f2c8 as i32 as uint32_t,
    0x1a96a6 as i32 as uint32_t,
    0xd1526ab as i32 as uint32_t,
    0x63315c21 as i32 as uint32_t,
    0x5e0a72ec as i32 as uint32_t,
    0x49bafefd as i32 as uint32_t,
    0x187908d9 as i32 as uint32_t,
    0x8d0dbd86 as u32,
    0x311170a7 as i32 as uint32_t,
    0x3e9b640c as i32 as uint32_t,
    0xcc3e10d7 as u32,
    0xd5cad3b6 as u32,
    0xcaec388 as i32 as uint32_t,
    0xf73001e1 as u32,
    0x6c728aff as i32 as uint32_t,
    0x71eae2a1 as i32 as uint32_t,
    0x1f9af36e as i32 as uint32_t,
    0xcfcbd12f as u32,
    0xc1de8417 as u32,
    0xac07be6b as u32,
    0xcb44a1d8 as u32,
    0x8b9b0f56 as u32,
    0x13988c3 as i32 as uint32_t,
    0xb1c52fca as u32,
    0xb4be31cd as u32,
    0xd8782806 as u32,
    0x12a3a4e2 as i32 as uint32_t,
    0x6f7de532 as i32 as uint32_t,
    0x58fd7eb6 as i32 as uint32_t,
    0xd01ee900 as u32,
    0x24adffc2 as i32 as uint32_t,
    0xf4990fc5 as u32,
    0x9711aac5 as u32,
    0x1d7b95 as i32 as uint32_t,
    0x82e5e7d2 as u32,
    0x109873f6 as i32 as uint32_t,
    0x613096 as i32 as uint32_t,
    0xc32d9521 as u32,
    0xada121ff as u32,
    0x29908415 as i32 as uint32_t,
    0x7fbb977f as i32 as uint32_t,
    0xaf9eb3db as u32,
    0x29c9ed2a as i32 as uint32_t,
    0x5ce2a465 as i32 as uint32_t,
    0xa730f32c as u32,
    0xd0aa3fe8 as u32,
    0x8a5cc091 as u32,
    0xd49e2ce7 as u32,
    0xce454a9 as i32 as uint32_t,
    0xd60acd86 as u32,
    0x15f1919 as i32 as uint32_t,
    0x77079103 as i32 as uint32_t,
    0xdea03af6 as u32,
    0x78a8565e as i32 as uint32_t,
    0xdee356df as u32,
    0x21f05cbe as i32 as uint32_t,
    0x8b75e387 as u32,
    0xb3c50651 as u32,
    0xb8a5c3ef as u32,
    0xd8eeb6d2 as u32,
    0xe523be77 as u32,
    0xc2154529 as u32,
    0x2f69efdf as i32 as uint32_t,
    0xafe67afb as u32,
    0xf470c4b2 as u32,
    0xf3e0eb5b as u32,
    0xd6cc9876 as u32,
    0x39e4460c as i32 as uint32_t,
    0x1fda8538 as i32 as uint32_t,
    0x1987832f as i32 as uint32_t,
    0xca007367 as u32,
    0xa99144f8 as u32,
    0x296b299e as i32 as uint32_t,
    0x492fc295 as i32 as uint32_t,
    0x9266beab as u32,
    0xb5676e69 as u32,
    0x9bd3ddda as u32,
    0xdf7e052f as u32,
    0xdb25701c as u32,
    0x1b5e51ee as i32 as uint32_t,
    0xf65324e6 as u32,
    0x6afce36c as i32 as uint32_t,
    0x316cc04 as i32 as uint32_t,
    0x8644213e as u32,
    0xb7dc59d0 as u32,
    0x7965291f as i32 as uint32_t,
    0xccd6fd43 as u32,
    0x41823979 as i32 as uint32_t,
    0x932bcdf6 as u32,
    0xb657c34d as u32,
    0x4edfd282 as i32 as uint32_t,
    0x7ae5290c as i32 as uint32_t,
    0x3cb9536b as i32 as uint32_t,
    0x851e20fe as u32,
    0x9833557e as u32,
    0x13ecf0b0 as i32 as uint32_t,
    0xd3ffb372 as u32,
    0x3f85c5c1 as i32 as uint32_t,
    0xaef7ed2 as i32 as uint32_t,
];
static mut cast_sbox5: [uint32_t; 256] = [
    0x7ec90c04 as i32 as uint32_t,
    0x2c6e74b9 as i32 as uint32_t,
    0x9b0e66df as u32,
    0xa6337911 as u32,
    0xb86a7fff as u32,
    0x1dd358f5 as i32 as uint32_t,
    0x44dd9d44 as i32 as uint32_t,
    0x1731167f as i32 as uint32_t,
    0x8fbf1fa as i32 as uint32_t,
    0xe7f511cc as u32,
    0xd2051b00 as u32,
    0x735aba00 as i32 as uint32_t,
    0x2ab722d8 as i32 as uint32_t,
    0x386381cb as i32 as uint32_t,
    0xacf6243a as u32,
    0x69befd7a as i32 as uint32_t,
    0xe6a2e77f as u32,
    0xf0c720cd as u32,
    0xc4494816 as u32,
    0xccf5c180 as u32,
    0x38851640 as i32 as uint32_t,
    0x15b0a848 as i32 as uint32_t,
    0xe68b18cb as u32,
    0x4caadeff as i32 as uint32_t,
    0x5f480a01 as i32 as uint32_t,
    0x412b2aa as i32 as uint32_t,
    0x259814fc as i32 as uint32_t,
    0x41d0efe2 as i32 as uint32_t,
    0x4e40b48d as i32 as uint32_t,
    0x248eb6fb as i32 as uint32_t,
    0x8dba1cfe as u32,
    0x41a99b02 as i32 as uint32_t,
    0x1a550a04 as i32 as uint32_t,
    0xba8f65cb as u32,
    0x7251f4e7 as i32 as uint32_t,
    0x95a51725 as u32,
    0xc106ecd7 as u32,
    0x97a5980a as u32,
    0xc539b9aa as u32,
    0x4d79fe6a as i32 as uint32_t,
    0xf2f3f763 as u32,
    0x68af8040 as i32 as uint32_t,
    0xed0c9e56 as u32,
    0x11b4958b as i32 as uint32_t,
    0xe1eb5a88 as u32,
    0x8709e6b0 as u32,
    0xd7e07156 as u32,
    0x4e29fea7 as i32 as uint32_t,
    0x6366e52d as i32 as uint32_t,
    0x2d1c000 as i32 as uint32_t,
    0xc4ac8e05 as u32,
    0x9377f571 as u32,
    0xc05372a as i32 as uint32_t,
    0x578535f2 as i32 as uint32_t,
    0x2261be02 as i32 as uint32_t,
    0xd642a0c9 as u32,
    0xdf13a280 as u32,
    0x74b55bd2 as i32 as uint32_t,
    0x682199c0 as i32 as uint32_t,
    0xd421e5ec as u32,
    0x53fb3ce8 as i32 as uint32_t,
    0xc8adedb3 as u32,
    0x28a87fc9 as i32 as uint32_t,
    0x3d959981 as i32 as uint32_t,
    0x5c1ff900 as i32 as uint32_t,
    0xfe38d399 as u32,
    0xc4eff0b as i32 as uint32_t,
    0x62407ea as i32 as uint32_t,
    0xaa2f4fb1 as u32,
    0x4fb96976 as i32 as uint32_t,
    0x90c79505 as u32,
    0xb0a8a774 as u32,
    0xef55a1ff as u32,
    0xe59ca2c2 as u32,
    0xa6b62d27 as u32,
    0xe66a4263 as u32,
    0xdf65001f as u32,
    0xec50966 as i32 as uint32_t,
    0xdfdd55bc as u32,
    0x29de0655 as i32 as uint32_t,
    0x911e739a as u32,
    0x17af8975 as i32 as uint32_t,
    0x32c7911c as i32 as uint32_t,
    0x89f89468 as u32,
    0xd01e980 as i32 as uint32_t,
    0x524755f4 as i32 as uint32_t,
    0x3b63cc9 as i32 as uint32_t,
    0xcc844b2 as i32 as uint32_t,
    0xbcf3f0aa as u32,
    0x87ac36e9 as u32,
    0xe53a7426 as u32,
    0x1b3d82b as i32 as uint32_t,
    0x1a9e7449 as i32 as uint32_t,
    0x64ee2d7e as i32 as uint32_t,
    0xcddbb1da as u32,
    0x1c94910 as i32 as uint32_t,
    0xb868bf80 as u32,
    0xd26f3fd as i32 as uint32_t,
    0x9342ede7 as u32,
    0x4a5c284 as i32 as uint32_t,
    0x636737b6 as i32 as uint32_t,
    0x50f5b616 as i32 as uint32_t,
    0xf24766e3 as u32,
    0x8eca36c1 as u32,
    0x136e05db as i32 as uint32_t,
    0xfef18391 as u32,
    0xfb887a37 as u32,
    0xd6e7f7d4 as u32,
    0xc7fb7dc9 as u32,
    0x3063fcdf as i32 as uint32_t,
    0xb6f589de as u32,
    0xec2941da as u32,
    0x26e46695 as i32 as uint32_t,
    0xb7566419 as u32,
    0xf654efc5 as u32,
    0xd08d58b7 as u32,
    0x48925401 as i32 as uint32_t,
    0xc1bacb7f as u32,
    0xe5ff550f as u32,
    0xb6083049 as u32,
    0x5bb5d0e8 as i32 as uint32_t,
    0x87d72e5a as u32,
    0xab6a6ee1 as u32,
    0x223a66ce as i32 as uint32_t,
    0xc62bf3cd as u32,
    0x9e0885f9 as u32,
    0x68cb3e47 as i32 as uint32_t,
    0x86c010f as i32 as uint32_t,
    0xa21de820 as u32,
    0xd18b69de as u32,
    0xf3f65777 as u32,
    0xfa02c3f6 as u32,
    0x407edac3 as i32 as uint32_t,
    0xcbb3d550 as u32,
    0x1793084d as i32 as uint32_t,
    0xb0d70eba as u32,
    0xab378d5 as i32 as uint32_t,
    0xd951fb0c as u32,
    0xded7da56 as u32,
    0x4124bbe4 as i32 as uint32_t,
    0x94ca0b56 as u32,
    0xf5755d1 as i32 as uint32_t,
    0xe0e1e56e as u32,
    0x6184b5be as i32 as uint32_t,
    0x580a249f as i32 as uint32_t,
    0x94f74bc0 as u32,
    0xe327888e as u32,
    0x9f7b5561 as u32,
    0xc3dc0280 as u32,
    0x5687715 as i32 as uint32_t,
    0x646c6bd7 as i32 as uint32_t,
    0x44904db3 as i32 as uint32_t,
    0x66b4f0a3 as i32 as uint32_t,
    0xc0f1648a as u32,
    0x697ed5af as i32 as uint32_t,
    0x49e92ff6 as i32 as uint32_t,
    0x309e374f as i32 as uint32_t,
    0x2cb6356a as i32 as uint32_t,
    0x85808573 as u32,
    0x4991f840 as i32 as uint32_t,
    0x76f0ae02 as i32 as uint32_t,
    0x83be84d as i32 as uint32_t,
    0x28421c9a as i32 as uint32_t,
    0x44489406 as i32 as uint32_t,
    0x736e4cb8 as i32 as uint32_t,
    0xc1092910 as u32,
    0x8bc95fc6 as u32,
    0x7d869cf4 as i32 as uint32_t,
    0x134f616f as i32 as uint32_t,
    0x2e77118d as i32 as uint32_t,
    0xb31b2be1 as u32,
    0xaa90b472 as u32,
    0x3ca5d717 as i32 as uint32_t,
    0x7d161bba as i32 as uint32_t,
    0x9cad9010 as u32,
    0xaf462ba2 as u32,
    0x9fe459d2 as u32,
    0x45d34559 as i32 as uint32_t,
    0xd9f2da13 as u32,
    0xdbc65487 as u32,
    0xf3e4f94e as u32,
    0x176d486f as i32 as uint32_t,
    0x97c13ea as i32 as uint32_t,
    0x631da5c7 as i32 as uint32_t,
    0x445f7382 as i32 as uint32_t,
    0x175683f4 as i32 as uint32_t,
    0xcdc66a97 as u32,
    0x70be0288 as i32 as uint32_t,
    0xb3cdcf72 as u32,
    0x6e5dd2f3 as i32 as uint32_t,
    0x20936079 as i32 as uint32_t,
    0x459b80a5 as i32 as uint32_t,
    0xbe60e2db as u32,
    0xa9c23101 as u32,
    0xeba5315c as u32,
    0x224e42f2 as i32 as uint32_t,
    0x1c5c1572 as i32 as uint32_t,
    0xf6721b2c as u32,
    0x1ad2fff3 as i32 as uint32_t,
    0x8c25404e as u32,
    0x324ed72f as i32 as uint32_t,
    0x4067b7fd as i32 as uint32_t,
    0x523138e as i32 as uint32_t,
    0x5ca3bc78 as i32 as uint32_t,
    0xdc0fd66e as u32,
    0x75922283 as i32 as uint32_t,
    0x784d6b17 as i32 as uint32_t,
    0x58ebb16e as i32 as uint32_t,
    0x44094f85 as i32 as uint32_t,
    0x3f481d87 as i32 as uint32_t,
    0xfcfeae7b as u32,
    0x77b5ff76 as i32 as uint32_t,
    0x8c2302bf as u32,
    0xaaf47556 as u32,
    0x5f46b02a as i32 as uint32_t,
    0x2b092801 as i32 as uint32_t,
    0x3d38f5f7 as i32 as uint32_t,
    0xca81f36 as i32 as uint32_t,
    0x52af4a8a as i32 as uint32_t,
    0x66d5e7c0 as i32 as uint32_t,
    0xdf3b0874 as u32,
    0x95055110 as u32,
    0x1b5ad7a8 as i32 as uint32_t,
    0xf61ed5ad as u32,
    0x6cf6e479 as i32 as uint32_t,
    0x20758184 as i32 as uint32_t,
    0xd0cefa65 as u32,
    0x88f7be58 as u32,
    0x4a046826 as i32 as uint32_t,
    0xff6f8f3 as i32 as uint32_t,
    0xa09c7f70 as u32,
    0x5346aba0 as i32 as uint32_t,
    0x5ce96c28 as i32 as uint32_t,
    0xe176eda3 as u32,
    0x6bac307f as i32 as uint32_t,
    0x376829d2 as i32 as uint32_t,
    0x85360fa9 as u32,
    0x17e3fe2a as i32 as uint32_t,
    0x24b79767 as i32 as uint32_t,
    0xf5a96b20 as u32,
    0xd6cd2595 as u32,
    0x68ff1ebf as i32 as uint32_t,
    0x7555442c as i32 as uint32_t,
    0xf19f06be as u32,
    0xf9e0659a as u32,
    0xeeb9491d as u32,
    0x34010718 as i32 as uint32_t,
    0xbb30cab8 as u32,
    0xe822fe15 as u32,
    0x88570983 as u32,
    0x750e6249 as i32 as uint32_t,
    0xda627e55 as u32,
    0x5e76ffa8 as i32 as uint32_t,
    0xb1534546 as u32,
    0x6d47de08 as i32 as uint32_t,
    0xefe9e7d4 as u32,
];
static mut cast_sbox6: [uint32_t; 256] = [
    0xf6fa8f9d as u32,
    0x2cac6ce1 as i32 as uint32_t,
    0x4ca34867 as i32 as uint32_t,
    0xe2337f7c as u32,
    0x95db08e7 as u32,
    0x16843b4 as i32 as uint32_t,
    0xeced5cbc as u32,
    0x325553ac as i32 as uint32_t,
    0xbf9f0960 as u32,
    0xdfa1e2ed as u32,
    0x83f0579d as u32,
    0x63ed86b9 as i32 as uint32_t,
    0x1ab6a6b8 as i32 as uint32_t,
    0xde5ebe39 as u32,
    0xf38ff732 as u32,
    0x8989b138 as u32,
    0x33f14961 as i32 as uint32_t,
    0xc01937bd as u32,
    0xf506c6da as u32,
    0xe4625e7e as u32,
    0xa308ea99 as u32,
    0x4e23e33c as i32 as uint32_t,
    0x79cbd7cc as i32 as uint32_t,
    0x48a14367 as i32 as uint32_t,
    0xa3149619 as u32,
    0xfec94bd5 as u32,
    0xa114174a as u32,
    0xeaa01866 as u32,
    0xa084db2d as u32,
    0x9a8486f as i32 as uint32_t,
    0xa888614a as u32,
    0x2900af98 as i32 as uint32_t,
    0x1665991 as i32 as uint32_t,
    0xe1992863 as u32,
    0xc8f30c60 as u32,
    0x2e78ef3c as i32 as uint32_t,
    0xd0d51932 as u32,
    0xcf0fec14 as u32,
    0xf7ca07d2 as u32,
    0xd0a82072 as u32,
    0xfd41197e as u32,
    0x9305a6b0 as u32,
    0xe86be3da as u32,
    0x74bed3cd as i32 as uint32_t,
    0x372da53c as i32 as uint32_t,
    0x4c7f4448 as i32 as uint32_t,
    0xdab5d440 as u32,
    0x6dba0ec3 as i32 as uint32_t,
    0x83919a7 as i32 as uint32_t,
    0x9fbaeed9 as u32,
    0x49dbcfb0 as i32 as uint32_t,
    0x4e670c53 as i32 as uint32_t,
    0x5c3d9c01 as i32 as uint32_t,
    0x64bdb941 as i32 as uint32_t,
    0x2c0e636a as i32 as uint32_t,
    0xba7dd9cd as u32,
    0xea6f7388 as u32,
    0xe70bc762 as u32,
    0x35f29adb as i32 as uint32_t,
    0x5c4cdd8d as i32 as uint32_t,
    0xf0d48d8c as u32,
    0xb88153e2 as u32,
    0x8a19866 as i32 as uint32_t,
    0x1ae2eac8 as i32 as uint32_t,
    0x284caf89 as i32 as uint32_t,
    0xaa928223 as u32,
    0x9334be53 as u32,
    0x3b3a21bf as i32 as uint32_t,
    0x16434be3 as i32 as uint32_t,
    0x9aea3906 as u32,
    0xefe8c36e as u32,
    0xf890cdd9 as u32,
    0x80226dae as u32,
    0xc340a4a3 as u32,
    0xdf7e9c09 as u32,
    0xa694a807 as u32,
    0x5b7c5ecc as i32 as uint32_t,
    0x221db3a6 as i32 as uint32_t,
    0x9a69a02f as u32,
    0x68818a54 as i32 as uint32_t,
    0xceb2296f as u32,
    0x53c0843a as i32 as uint32_t,
    0xfe893655 as u32,
    0x25bfe68a as i32 as uint32_t,
    0xb4628abc as u32,
    0xcf222ebf as u32,
    0x25ac6f48 as i32 as uint32_t,
    0xa9a99387 as u32,
    0x53bddb65 as i32 as uint32_t,
    0xe76ffbe7 as u32,
    0xe967fd78 as u32,
    0xba93563 as i32 as uint32_t,
    0x8e342bc1 as u32,
    0xe8a11be9 as u32,
    0x4980740d as i32 as uint32_t,
    0xc8087dfc as u32,
    0x8de4bf99 as u32,
    0xa11101a0 as u32,
    0x7fd37975 as i32 as uint32_t,
    0xda5a26c0 as u32,
    0xe81f994f as u32,
    0x9528cd89 as u32,
    0xfd339fed as u32,
    0xb87834bf as u32,
    0x5f04456d as i32 as uint32_t,
    0x22258698 as i32 as uint32_t,
    0xc9c4c83b as u32,
    0x2dc156be as i32 as uint32_t,
    0x4f628daa as i32 as uint32_t,
    0x57f55ec5 as i32 as uint32_t,
    0xe2220abe as u32,
    0xd2916ebf as u32,
    0x4ec75b95 as i32 as uint32_t,
    0x24f2c3c0 as i32 as uint32_t,
    0x42d15d99 as i32 as uint32_t,
    0xcd0d7fa0 as u32,
    0x7b6e27ff as i32 as uint32_t,
    0xa8dc8af0 as u32,
    0x7345c106 as i32 as uint32_t,
    0xf41e232f as u32,
    0x35162386 as i32 as uint32_t,
    0xe6ea8926 as u32,
    0x3333b094 as i32 as uint32_t,
    0x157ec6f2 as i32 as uint32_t,
    0x372b74af as i32 as uint32_t,
    0x692573e4 as i32 as uint32_t,
    0xe9a9d848 as u32,
    0xf3160289 as u32,
    0x3a62ef1d as i32 as uint32_t,
    0xa787e238 as u32,
    0xf3a5f676 as u32,
    0x74364853 as i32 as uint32_t,
    0x20951063 as i32 as uint32_t,
    0x4576698d as i32 as uint32_t,
    0xb6fad407 as u32,
    0x592af950 as i32 as uint32_t,
    0x36f73523 as i32 as uint32_t,
    0x4cfb6e87 as i32 as uint32_t,
    0x7da4cec0 as i32 as uint32_t,
    0x6c152daa as i32 as uint32_t,
    0xcb0396a8 as u32,
    0xc50dfe5d as u32,
    0xfcd707ab as u32,
    0x921c42f as i32 as uint32_t,
    0x89dff0bb as u32,
    0x5fe2be78 as i32 as uint32_t,
    0x448f4f33 as i32 as uint32_t,
    0x754613c9 as i32 as uint32_t,
    0x2b05d08d as i32 as uint32_t,
    0x48b9d585 as i32 as uint32_t,
    0xdc049441 as u32,
    0xc8098f9b as u32,
    0x7dede786 as i32 as uint32_t,
    0xc39a3373 as u32,
    0x42410005 as i32 as uint32_t,
    0x6a091751 as i32 as uint32_t,
    0xef3c8a6 as i32 as uint32_t,
    0x890072d6 as u32,
    0x28207682 as i32 as uint32_t,
    0xa9a9f7be as u32,
    0xbf32679d as u32,
    0xd45b5b75 as u32,
    0xb353fd00 as u32,
    0xcbb0e358 as u32,
    0x830f220a as u32,
    0x1f8fb214 as i32 as uint32_t,
    0xd372cf08 as u32,
    0xcc3c4a13 as u32,
    0x8cf63166 as u32,
    0x61c87be as i32 as uint32_t,
    0x88c98f88 as u32,
    0x6062e397 as i32 as uint32_t,
    0x47cf8e7a as i32 as uint32_t,
    0xb6c85283 as u32,
    0x3cc2acfb as i32 as uint32_t,
    0x3fc06976 as i32 as uint32_t,
    0x4e8f0252 as i32 as uint32_t,
    0x64d8314d as i32 as uint32_t,
    0xda3870e3 as u32,
    0x1e665459 as i32 as uint32_t,
    0xc10908f0 as u32,
    0x513021a5 as i32 as uint32_t,
    0x6c5b68b7 as i32 as uint32_t,
    0x822f8aa0 as u32,
    0x3007cd3e as i32 as uint32_t,
    0x74719eef as i32 as uint32_t,
    0xdc872681 as u32,
    0x73340d4 as i32 as uint32_t,
    0x7e432fd9 as i32 as uint32_t,
    0xc5ec241 as i32 as uint32_t,
    0x8809286c as u32,
    0xf592d891 as u32,
    0x8a930f6 as i32 as uint32_t,
    0x957ef305 as u32,
    0xb7fbffbd as u32,
    0xc266e96f as u32,
    0x6fe4ac98 as i32 as uint32_t,
    0xb173ecc0 as u32,
    0xbc60b42a as u32,
    0x953498da as u32,
    0xfba1ae12 as u32,
    0x2d4bd736 as i32 as uint32_t,
    0xf25faab as i32 as uint32_t,
    0xa4f3fceb as u32,
    0xe2969123 as u32,
    0x257f0c3d as i32 as uint32_t,
    0x9348af49 as u32,
    0x361400bc as i32 as uint32_t,
    0xe8816f4a as u32,
    0x3814f200 as i32 as uint32_t,
    0xa3f94043 as u32,
    0x9c7a54c2 as u32,
    0xbc704f57 as u32,
    0xda41e7f9 as u32,
    0xc25ad33a as u32,
    0x54f4a084 as i32 as uint32_t,
    0xb17f5505 as u32,
    0x59357cbe as i32 as uint32_t,
    0xedbd15c8 as u32,
    0x7f97c5ab as i32 as uint32_t,
    0xba5ac7b5 as u32,
    0xb6f6deaf as u32,
    0x3a479c3a as i32 as uint32_t,
    0x5302da25 as i32 as uint32_t,
    0x653d7e6a as i32 as uint32_t,
    0x54268d49 as i32 as uint32_t,
    0x51a477ea as i32 as uint32_t,
    0x5017d55b as i32 as uint32_t,
    0xd7d25d88 as u32,
    0x44136c76 as i32 as uint32_t,
    0x404a8c8 as i32 as uint32_t,
    0xb8e5a121 as u32,
    0xb81a928a as u32,
    0x60ed5869 as i32 as uint32_t,
    0x97c55b96 as u32,
    0xeaec991b as u32,
    0x29935913 as i32 as uint32_t,
    0x1fdb7f1 as i32 as uint32_t,
    0x88e8dfa as i32 as uint32_t,
    0x9ab6f6f5 as u32,
    0x3b4cbf9f as i32 as uint32_t,
    0x4a5de3ab as i32 as uint32_t,
    0xe6051d35 as u32,
    0xa0e1d855 as u32,
    0xd36b4cf1 as u32,
    0xf544edeb as u32,
    0xb0e93524 as u32,
    0xbebb8fbd as u32,
    0xa2d762cf as u32,
    0x49c92f54 as i32 as uint32_t,
    0x38b5f331 as i32 as uint32_t,
    0x7128a454 as i32 as uint32_t,
    0x48392905 as i32 as uint32_t,
    0xa65b1db8 as u32,
    0x851c97bd as u32,
    0xd675cf2f as u32,
];
static mut cast_sbox7: [uint32_t; 256] = [
    0x85e04019 as u32,
    0x332bf567 as i32 as uint32_t,
    0x662dbfff as i32 as uint32_t,
    0xcfc65693 as u32,
    0x2a8d7f6f as i32 as uint32_t,
    0xab9bc912 as u32,
    0xde6008a1 as u32,
    0x2028da1f as i32 as uint32_t,
    0x227bce7 as i32 as uint32_t,
    0x4d642916 as i32 as uint32_t,
    0x18fac300 as i32 as uint32_t,
    0x50f18b82 as i32 as uint32_t,
    0x2cb2cb11 as i32 as uint32_t,
    0xb232e75c as u32,
    0x4b3695f2 as i32 as uint32_t,
    0xb28707de as u32,
    0xa05fbcf6 as u32,
    0xcd4181e9 as u32,
    0xe150210c as u32,
    0xe24ef1bd as u32,
    0xb168c381 as u32,
    0xfde4e789 as u32,
    0x5c79b0d8 as i32 as uint32_t,
    0x1e8bfd43 as i32 as uint32_t,
    0x4d495001 as i32 as uint32_t,
    0x38be4341 as i32 as uint32_t,
    0x913cee1d as u32,
    0x92a79c3f as u32,
    0x89766be as i32 as uint32_t,
    0xbaeeadf4 as u32,
    0x1286becf as i32 as uint32_t,
    0xb6eacb19 as u32,
    0x2660c200 as i32 as uint32_t,
    0x7565bde4 as i32 as uint32_t,
    0x64241f7a as i32 as uint32_t,
    0x8248dca9 as u32,
    0xc3b3ad66 as u32,
    0x28136086 as i32 as uint32_t,
    0xbd8dfa8 as i32 as uint32_t,
    0x356d1cf2 as i32 as uint32_t,
    0x107789be as i32 as uint32_t,
    0xb3b2e9ce as u32,
    0x502aa8f as i32 as uint32_t,
    0xbc0351e as i32 as uint32_t,
    0x166bf52a as i32 as uint32_t,
    0xeb12ff82 as u32,
    0xe3486911 as u32,
    0xd34d7516 as u32,
    0x4e7b3aff as i32 as uint32_t,
    0x5f43671b as i32 as uint32_t,
    0x9cf6e037 as u32,
    0x4981ac83 as i32 as uint32_t,
    0x334266ce as i32 as uint32_t,
    0x8c9341b7 as u32,
    0xd0d854c0 as u32,
    0xcb3a6c88 as u32,
    0x47bc2829 as i32 as uint32_t,
    0x4725ba37 as i32 as uint32_t,
    0xa66ad22b as u32,
    0x7ad61f1e as i32 as uint32_t,
    0xc5cbafa as i32 as uint32_t,
    0x4437f107 as i32 as uint32_t,
    0xb6e79962 as u32,
    0x42d2d816 as i32 as uint32_t,
    0xa961288 as i32 as uint32_t,
    0xe1a5c06e as u32,
    0x13749e67 as i32 as uint32_t,
    0x72fc081a as i32 as uint32_t,
    0xb1d139f7 as u32,
    0xf9583745 as u32,
    0xcf19df58 as u32,
    0xbec3f756 as u32,
    0xc06eba30 as u32,
    0x7211b24 as i32 as uint32_t,
    0x45c28829 as i32 as uint32_t,
    0xc95e317f as u32,
    0xbc8ec511 as u32,
    0x38bc46e9 as i32 as uint32_t,
    0xc6e6fa14 as u32,
    0xbae8584a as u32,
    0xad4ebc46 as u32,
    0x468f508b as i32 as uint32_t,
    0x7829435f as i32 as uint32_t,
    0xf124183b as u32,
    0x821dba9f as u32,
    0xaff60ff4 as u32,
    0xea2c4e6d as u32,
    0x16e39264 as i32 as uint32_t,
    0x92544a8b as u32,
    0x9b4fc3 as i32 as uint32_t,
    0xaba68ced as u32,
    0x9ac96f78 as u32,
    0x6a5b79a as i32 as uint32_t,
    0xb2856e6e as u32,
    0x1aec3ca9 as i32 as uint32_t,
    0xbe838688 as u32,
    0xe0804e9 as i32 as uint32_t,
    0x55f1be56 as i32 as uint32_t,
    0xe7e5363b as u32,
    0xb3a1f25d as u32,
    0xf7debb85 as u32,
    0x61fe033c as i32 as uint32_t,
    0x16746233 as i32 as uint32_t,
    0x3c034c28 as i32 as uint32_t,
    0xda6d0c74 as u32,
    0x79aac56c as i32 as uint32_t,
    0x3ce4e1ad as i32 as uint32_t,
    0x51f0c802 as i32 as uint32_t,
    0x98f8f35a as u32,
    0x1626a49f as i32 as uint32_t,
    0xeed82b29 as u32,
    0x1d382fe3 as i32 as uint32_t,
    0xc4fb99a as i32 as uint32_t,
    0xbb325778 as u32,
    0x3ec6d97b as i32 as uint32_t,
    0x6e77a6a9 as i32 as uint32_t,
    0xcb658b5c as u32,
    0xd45230c7 as u32,
    0x2bd1408b as i32 as uint32_t,
    0x60c03eb7 as i32 as uint32_t,
    0xb9068d78 as u32,
    0xa33754f4 as u32,
    0xf430c87d as u32,
    0xc8a71302 as u32,
    0xb96d8c32 as u32,
    0xebd4e7be as u32,
    0xbe8b9d2d as u32,
    0x7979fb06 as i32 as uint32_t,
    0xe7225308 as u32,
    0x8b75cf77 as u32,
    0x11ef8da4 as i32 as uint32_t,
    0xe083c858 as u32,
    0x8d6b786f as u32,
    0x5a6317a6 as i32 as uint32_t,
    0xfa5cf7a0 as u32,
    0x5dda0033 as i32 as uint32_t,
    0xf28ebfb0 as u32,
    0xf5b9c310 as u32,
    0xa0eac280 as u32,
    0x8b9767a as i32 as uint32_t,
    0xa3d9d2b0 as u32,
    0x79d34217 as i32 as uint32_t,
    0x21a718d as i32 as uint32_t,
    0x9ac6336a as u32,
    0x2711fd60 as i32 as uint32_t,
    0x438050e3 as i32 as uint32_t,
    0x69908a8 as i32 as uint32_t,
    0x3d7fedc4 as i32 as uint32_t,
    0x826d2bef as u32,
    0x4eeb8476 as i32 as uint32_t,
    0x488dcf25 as i32 as uint32_t,
    0x36c9d566 as i32 as uint32_t,
    0x28e74e41 as i32 as uint32_t,
    0xc2610aca as u32,
    0x3d49a9cf as i32 as uint32_t,
    0xbae3b9df as u32,
    0xb65f8de6 as u32,
    0x92aeaf64 as u32,
    0x3ac7d5e6 as i32 as uint32_t,
    0x9ea80509 as u32,
    0xf22b017d as u32,
    0xa4173f70 as u32,
    0xdd1e16c3 as u32,
    0x15e0d7f9 as i32 as uint32_t,
    0x50b1b887 as i32 as uint32_t,
    0x2b9f4fd5 as i32 as uint32_t,
    0x625aba82 as i32 as uint32_t,
    0x6a017962 as i32 as uint32_t,
    0x2ec01b9c as i32 as uint32_t,
    0x15488aa9 as i32 as uint32_t,
    0xd716e740 as u32,
    0x40055a2c as i32 as uint32_t,
    0x93d29a22 as u32,
    0xe32dbf9a as u32,
    0x58745b9 as i32 as uint32_t,
    0x3453dc1e as i32 as uint32_t,
    0xd699296e as u32,
    0x496cff6f as i32 as uint32_t,
    0x1c9f4986 as i32 as uint32_t,
    0xdfe2ed07 as u32,
    0xb87242d1 as u32,
    0x19de7eae as i32 as uint32_t,
    0x53e561a as i32 as uint32_t,
    0x15ad6f8c as i32 as uint32_t,
    0x66626c1c as i32 as uint32_t,
    0x7154c24c as i32 as uint32_t,
    0xea082b2a as u32,
    0x93eb2939 as u32,
    0x17dcb0f0 as i32 as uint32_t,
    0x58d4f2ae as i32 as uint32_t,
    0x9ea294fb as u32,
    0x52cf564c as i32 as uint32_t,
    0x9883fe66 as u32,
    0x2ec40581 as i32 as uint32_t,
    0x763953c3 as i32 as uint32_t,
    0x1d6692e as i32 as uint32_t,
    0xd3a0c108 as u32,
    0xa1e7160e as u32,
    0xe4f2dfa6 as u32,
    0x693ed285 as i32 as uint32_t,
    0x74904698 as i32 as uint32_t,
    0x4c2b0edd as i32 as uint32_t,
    0x4f757656 as i32 as uint32_t,
    0x5d393378 as i32 as uint32_t,
    0xa132234f as u32,
    0x3d321c5d as i32 as uint32_t,
    0xc3f5e194 as u32,
    0x4b269301 as i32 as uint32_t,
    0xc79f022f as u32,
    0x3c997e7e as i32 as uint32_t,
    0x5e4f9504 as i32 as uint32_t,
    0x3ffafbbd as i32 as uint32_t,
    0x76f7ad0e as i32 as uint32_t,
    0x296693f4 as i32 as uint32_t,
    0x3d1fce6f as i32 as uint32_t,
    0xc61e45be as u32,
    0xd3b5ab34 as u32,
    0xf72bf9b7 as u32,
    0x1b0434c0 as i32 as uint32_t,
    0x4e72b567 as i32 as uint32_t,
    0x5592a33d as i32 as uint32_t,
    0xb5229301 as u32,
    0xcfd2a87f as u32,
    0x60aeb767 as i32 as uint32_t,
    0x1814386b as i32 as uint32_t,
    0x30bcc33d as i32 as uint32_t,
    0x38a0c07d as i32 as uint32_t,
    0xfd1606f2 as u32,
    0xc363519b as u32,
    0x589dd390 as i32 as uint32_t,
    0x5479f8e6 as i32 as uint32_t,
    0x1cb8d647 as i32 as uint32_t,
    0x97fd61a9 as u32,
    0xea7759f4 as u32,
    0x2d57539d as i32 as uint32_t,
    0x569a58cf as i32 as uint32_t,
    0xe84e63ad as u32,
    0x462e1b78 as i32 as uint32_t,
    0x6580f87e as i32 as uint32_t,
    0xf3817914 as u32,
    0x91da55f4 as u32,
    0x40a230f3 as i32 as uint32_t,
    0xd1988f35 as u32,
    0xb6e318d2 as u32,
    0x3ffa50bc as i32 as uint32_t,
    0x3d40f021 as i32 as uint32_t,
    0xc3c0bdae as u32,
    0x4958c24c as i32 as uint32_t,
    0x518f36b2 as i32 as uint32_t,
    0x84b1d370 as u32,
    0xfedce83 as i32 as uint32_t,
    0x878ddada as u32,
    0xf2a279c7 as u32,
    0x94e01be8 as u32,
    0x90716f4b as u32,
    0x954b8aa3 as u32,
];
static mut cast_sbox8: [uint32_t; 256] = [
    0xe216300d as u32,
    0xbbddfffc as u32,
    0xa7ebdabd as u32,
    0x35648095 as i32 as uint32_t,
    0x7789f8b7 as i32 as uint32_t,
    0xe6c1121b as u32,
    0xe241600 as i32 as uint32_t,
    0x52ce8b5 as i32 as uint32_t,
    0x11a9cfb0 as i32 as uint32_t,
    0xe5952f11 as u32,
    0xece7990a as u32,
    0x9386d174 as u32,
    0x2a42931c as i32 as uint32_t,
    0x76e38111 as i32 as uint32_t,
    0xb12def3a as u32,
    0x37ddddfc as i32 as uint32_t,
    0xde9adeb1 as u32,
    0xa0cc32c as i32 as uint32_t,
    0xbe197029 as u32,
    0x84a00940 as u32,
    0xbb243a0f as u32,
    0xb4d137cf as u32,
    0xb44e79f0 as u32,
    0x49eedfd as i32 as uint32_t,
    0xb15a15d as i32 as uint32_t,
    0x480d3168 as i32 as uint32_t,
    0x8bbbde5a as u32,
    0x669ded42 as i32 as uint32_t,
    0xc7ece831 as u32,
    0x3f8f95e7 as i32 as uint32_t,
    0x72df191b as i32 as uint32_t,
    0x7580330d as i32 as uint32_t,
    0x94074251 as u32,
    0x5c7dcdfa as i32 as uint32_t,
    0xabbe6d63 as u32,
    0xaa402164 as u32,
    0xb301d40a as u32,
    0x2e7d1ca as i32 as uint32_t,
    0x53571dae as i32 as uint32_t,
    0x7a3182a2 as i32 as uint32_t,
    0x12a8ddec as i32 as uint32_t,
    0xfdaa335d as u32,
    0x176f43e8 as i32 as uint32_t,
    0x71fb46d4 as i32 as uint32_t,
    0x38129022 as i32 as uint32_t,
    0xce949ad4 as u32,
    0xb84769ad as u32,
    0x965bd862 as u32,
    0x82f3d055 as u32,
    0x66fb9767 as i32 as uint32_t,
    0x15b80b4e as i32 as uint32_t,
    0x1d5b47a0 as i32 as uint32_t,
    0x4cfde06f as i32 as uint32_t,
    0xc28ec4b8 as u32,
    0x57e8726e as i32 as uint32_t,
    0x647a78fc as i32 as uint32_t,
    0x99865d44 as u32,
    0x608bd593 as i32 as uint32_t,
    0x6c200e03 as i32 as uint32_t,
    0x39dc5ff6 as i32 as uint32_t,
    0x5d0b00a3 as i32 as uint32_t,
    0xae63aff2 as u32,
    0x7e8bd632 as i32 as uint32_t,
    0x70108c0c as i32 as uint32_t,
    0xbbd35049 as u32,
    0x2998df04 as i32 as uint32_t,
    0x980cf42a as u32,
    0x9b6df491 as u32,
    0x9e7edd53 as u32,
    0x6918548 as i32 as uint32_t,
    0x58cb7e07 as i32 as uint32_t,
    0x3b74ef2e as i32 as uint32_t,
    0x522fffb1 as i32 as uint32_t,
    0xd24708cc as u32,
    0x1c7e27cd as i32 as uint32_t,
    0xa4eb215b as u32,
    0x3cf1d2e2 as i32 as uint32_t,
    0x19b47a38 as i32 as uint32_t,
    0x424f7618 as i32 as uint32_t,
    0x35856039 as i32 as uint32_t,
    0x9d17dee7 as u32,
    0x27eb35e6 as i32 as uint32_t,
    0xc9aff67b as u32,
    0x36baf5b8 as i32 as uint32_t,
    0x9c467cd as i32 as uint32_t,
    0xc18910b1 as u32,
    0xe11dbf7b as u32,
    0x6cd1af8 as i32 as uint32_t,
    0x7170c608 as i32 as uint32_t,
    0x2d5e3354 as i32 as uint32_t,
    0xd4de495a as u32,
    0x64c6d006 as i32 as uint32_t,
    0xbcc0c62c as u32,
    0x3dd00db3 as i32 as uint32_t,
    0x708f8f34 as i32 as uint32_t,
    0x77d51b42 as i32 as uint32_t,
    0x264f620f as i32 as uint32_t,
    0x24b8d2bf as i32 as uint32_t,
    0x15c1b79e as i32 as uint32_t,
    0x46a52564 as i32 as uint32_t,
    0xf8d7e54e as u32,
    0x3e378160 as i32 as uint32_t,
    0x7895cda5 as i32 as uint32_t,
    0x859c15a5 as u32,
    0xe6459788 as u32,
    0xc37bc75f as u32,
    0xdb07ba0c as u32,
    0x676a3ab as i32 as uint32_t,
    0x7f229b1e as i32 as uint32_t,
    0x31842e7b as i32 as uint32_t,
    0x24259fd7 as i32 as uint32_t,
    0xf8bef472 as u32,
    0x835ffcb8 as u32,
    0x6df4c1f2 as i32 as uint32_t,
    0x96f5b195 as u32,
    0xfd0af0fc as u32,
    0xb0fe134c as u32,
    0xe2506d3d as u32,
    0x4f9b12ea as i32 as uint32_t,
    0xf215f225 as u32,
    0xa223736f as u32,
    0x9fb4c428 as u32,
    0x25d04979 as i32 as uint32_t,
    0x34c713f8 as i32 as uint32_t,
    0xc4618187 as u32,
    0xea7a6e98 as u32,
    0x7cd16efc as i32 as uint32_t,
    0x1436876c as i32 as uint32_t,
    0xf1544107 as u32,
    0xbedeee14 as u32,
    0x56e9af27 as i32 as uint32_t,
    0xa04aa441 as u32,
    0x3cf7c899 as i32 as uint32_t,
    0x92ecbae6 as u32,
    0xdd67016d as u32,
    0x151682eb as i32 as uint32_t,
    0xa842eedf as u32,
    0xfdba60b4 as u32,
    0xf1907b75 as u32,
    0x20e3030f as i32 as uint32_t,
    0x24d8c29e as i32 as uint32_t,
    0xe139673b as u32,
    0xefa63fb8 as u32,
    0x71873054 as i32 as uint32_t,
    0xb6f2cf3b as u32,
    0x9f326442 as u32,
    0xcb15a4cc as u32,
    0xb01a4504 as u32,
    0xf1e47d8d as u32,
    0x844a1be5 as u32,
    0xbae7dfdc as u32,
    0x42cbda70 as i32 as uint32_t,
    0xcd7dae0a as u32,
    0x57e85b7a as i32 as uint32_t,
    0xd53f5af6 as u32,
    0x20cf4d8c as i32 as uint32_t,
    0xcea4d428 as u32,
    0x79d130a4 as i32 as uint32_t,
    0x3486ebfb as i32 as uint32_t,
    0x33d3cddc as i32 as uint32_t,
    0x77853b53 as i32 as uint32_t,
    0x37effcb5 as i32 as uint32_t,
    0xc5068778 as u32,
    0xe580b3e6 as u32,
    0x4e68b8f4 as i32 as uint32_t,
    0xc5c8b37e as u32,
    0xd809ea2 as i32 as uint32_t,
    0x398feb7c as i32 as uint32_t,
    0x132a4f94 as i32 as uint32_t,
    0x43b7950e as i32 as uint32_t,
    0x2fee7d1c as i32 as uint32_t,
    0x223613bd as i32 as uint32_t,
    0xdd06caa2 as u32,
    0x37df932b as i32 as uint32_t,
    0xc4248289 as u32,
    0xacf3ebc3 as u32,
    0x5715f6b7 as i32 as uint32_t,
    0xef3478dd as u32,
    0xf267616f as u32,
    0xc148cbe4 as u32,
    0x9052815e as u32,
    0x5e410fab as i32 as uint32_t,
    0xb48a2465 as u32,
    0x2eda7fa4 as i32 as uint32_t,
    0xe87b40e4 as u32,
    0xe98ea084 as u32,
    0x5889e9e1 as i32 as uint32_t,
    0xefd390fc as u32,
    0xdd07d35b as u32,
    0xdb485694 as u32,
    0x38d7e5b2 as i32 as uint32_t,
    0x57720101 as i32 as uint32_t,
    0x730edebc as i32 as uint32_t,
    0x5b643113 as i32 as uint32_t,
    0x94917e4f as u32,
    0x503c2fba as i32 as uint32_t,
    0x646f1282 as i32 as uint32_t,
    0x7523d24a as i32 as uint32_t,
    0xe0779695 as u32,
    0xf9c17a8f as u32,
    0x7a5b2121 as i32 as uint32_t,
    0xd187b896 as u32,
    0x29263a4d as i32 as uint32_t,
    0xba510cdf as u32,
    0x81f47c9f as u32,
    0xad1163ed as u32,
    0xea7b5965 as u32,
    0x1a00726e as i32 as uint32_t,
    0x11403092 as i32 as uint32_t,
    0xda6d77 as i32 as uint32_t,
    0x4a0cdd61 as i32 as uint32_t,
    0xad1f4603 as u32,
    0x605bdfb0 as i32 as uint32_t,
    0x9eedc364 as u32,
    0x22ebe6a8 as i32 as uint32_t,
    0xcee7d28a as u32,
    0xa0e736a0 as u32,
    0x5564a6b9 as i32 as uint32_t,
    0x10853209 as i32 as uint32_t,
    0xc7eb8f37 as u32,
    0x2de705ca as i32 as uint32_t,
    0x8951570f as u32,
    0xdf09822b as u32,
    0xbd691a6c as u32,
    0xaa12e4f2 as u32,
    0x87451c0f as u32,
    0xe0f6a27a as u32,
    0x3ada4819 as i32 as uint32_t,
    0x4cf1764f as i32 as uint32_t,
    0xd771c2b as i32 as uint32_t,
    0x67cdb156 as i32 as uint32_t,
    0x350d8384 as i32 as uint32_t,
    0x5938fa0f as i32 as uint32_t,
    0x42399ef3 as i32 as uint32_t,
    0x36997b07 as i32 as uint32_t,
    0xe84093d as i32 as uint32_t,
    0x4aa93e61 as i32 as uint32_t,
    0x8360d87b as u32,
    0x1fa98b0c as i32 as uint32_t,
    0x1149382c as i32 as uint32_t,
    0xe97625a5 as u32,
    0x614d1b7 as i32 as uint32_t,
    0xe25244b as i32 as uint32_t,
    0xc768347 as i32 as uint32_t,
    0x589e8d82 as i32 as uint32_t,
    0xd2059d1 as i32 as uint32_t,
    0xa466bb1e as u32,
    0xf8da0a82 as u32,
    0x4f19130 as i32 as uint32_t,
    0xba6e4ec0 as u32,
    0x99265164 as u32,
    0x1ee7230d as i32 as uint32_t,
    0x50b2ad80 as i32 as uint32_t,
    0xeaee6801 as u32,
    0x8db2a283 as u32,
    0xea8bf59e as u32,
];
#[no_mangle]
pub unsafe extern "C" fn nettle_cast128_encrypt(
    mut ctx: *const cast128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const i8,
            b"cast128.c\0" as *const u8 as *const i8,
            99 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 92],
                &[i8; 92],
            >(
                b"void nettle_cast128_encrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12424: {
        if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const i8,
                b"cast128.c\0" as *const u8 as *const i8,
                99 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 92],
                    &[i8; 92],
                >(
                    b"void nettle_cast128_encrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut t: uint32_t = 0;
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        l = (*src.offset(0 as i32 as isize) as uint32_t) << 24 as i32
            | (*src.offset(1 as i32 as isize) as uint32_t) << 16 as i32
            | (*src.offset(2 as i32 as isize) as uint32_t) << 8 as i32
            | *src.offset(3 as i32 as isize) as uint32_t;
        r = (*src.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
            << 24 as i32
            | (*src.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
                << 16 as i32
            | (*src.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
                << 8 as i32
            | *src.offset(4 as i32 as isize).offset(3 as i32 as isize) as uint32_t;
        t = ((*ctx).Km[0 as i32 as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[0 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[0 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[1 as i32 as usize] ^ l;
        t = t << (*ctx).Kr[1 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[1 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[2 as i32 as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[2 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[2 as i32 as usize] as i32) & 31 as i32);
        l
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = ((*ctx).Km[3 as i32 as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[3 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[3 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[4 as i32 as usize] ^ r;
        t = t << (*ctx).Kr[4 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[4 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[5 as i32 as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[5 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[5 as i32 as usize] as i32) & 31 as i32);
        r
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = ((*ctx).Km[6 as i32 as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[6 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[6 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[7 as i32 as usize] ^ l;
        t = t << (*ctx).Kr[7 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[7 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[8 as i32 as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[8 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[8 as i32 as usize] as i32) & 31 as i32);
        l
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = ((*ctx).Km[9 as i32 as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[9 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[9 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[10 as i32 as usize] ^ r;
        t = t << (*ctx).Kr[10 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[10 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[11 as i32 as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[11 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[11 as i32 as usize] as i32) & 31 as i32);
        r
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        if (*ctx).rounds & 16 as i32 as u32 != 0 {
            t = ((*ctx).Km[12 as i32 as usize]).wrapping_add(r);
            t = t << (*ctx).Kr[12 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[12 as i32 as usize] as i32) & 31 as i32);
            l
                ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                        as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize],
                    );
            t = (*ctx).Km[13 as i32 as usize] ^ l;
            t = t << (*ctx).Kr[13 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[13 as i32 as usize] as i32) & 31 as i32);
            r
                ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    .wrapping_add(
                        cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
            t = ((*ctx).Km[14 as i32 as usize]).wrapping_sub(r);
            t = t << (*ctx).Kr[14 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[14 as i32 as usize] as i32) & 31 as i32);
            l
                ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                    .wrapping_add(
                        cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                        as usize])
                    .wrapping_sub(
                        cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize],
                    );
            t = ((*ctx).Km[15 as i32 as usize]).wrapping_add(l);
            t = t << (*ctx).Kr[15 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[15 as i32 as usize] as i32) & 31 as i32);
            r
                ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                        as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize],
                    );
        }
        *dst.offset(0 as i32 as isize) = (r >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(1 as i32 as isize) = (r >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(2 as i32 as isize) = (r >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset(3 as i32 as isize) = (r & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(0 as i32 as isize) = (l >> 24 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(1 as i32 as isize) = (l >> 16 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(2 as i32 as isize) = (l >> 8 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(3 as i32 as isize) = (l
            & 0xff as i32 as u32) as uint8_t;
        length = (length as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
        dst = dst.offset(8 as i32 as isize);
        src = src.offset(8 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cast128_decrypt(
    mut ctx: *const cast128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const i8,
            b"cast128.c\0" as *const u8 as *const i8,
            141 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 92],
                &[i8; 92],
            >(
                b"void nettle_cast128_decrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14620: {
        if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const i8,
                b"cast128.c\0" as *const u8 as *const i8,
                141 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 92],
                    &[i8; 92],
                >(
                    b"void nettle_cast128_decrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut t: uint32_t = 0;
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        r = (*src.offset(0 as i32 as isize) as uint32_t) << 24 as i32
            | (*src.offset(1 as i32 as isize) as uint32_t) << 16 as i32
            | (*src.offset(2 as i32 as isize) as uint32_t) << 8 as i32
            | *src.offset(3 as i32 as isize) as uint32_t;
        l = (*src.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
            << 24 as i32
            | (*src.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
                << 16 as i32
            | (*src.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
                << 8 as i32
            | *src.offset(4 as i32 as isize).offset(3 as i32 as isize) as uint32_t;
        if (*ctx).rounds & 16 as i32 as u32 != 0 {
            t = ((*ctx).Km[15 as i32 as usize]).wrapping_add(l);
            t = t << (*ctx).Kr[15 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[15 as i32 as usize] as i32) & 31 as i32);
            r
                ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                        as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize],
                    );
            t = ((*ctx).Km[14 as i32 as usize]).wrapping_sub(r);
            t = t << (*ctx).Kr[14 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[14 as i32 as usize] as i32) & 31 as i32);
            l
                ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                    .wrapping_add(
                        cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                        as usize])
                    .wrapping_sub(
                        cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize],
                    );
            t = (*ctx).Km[13 as i32 as usize] ^ l;
            t = t << (*ctx).Kr[13 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[13 as i32 as usize] as i32) & 31 as i32);
            r
                ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    .wrapping_add(
                        cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
            t = ((*ctx).Km[12 as i32 as usize]).wrapping_add(r);
            t = t << (*ctx).Kr[12 as i32 as usize] as i32
                | t >> (-((*ctx).Kr[12 as i32 as usize] as i32) & 31 as i32);
            l
                ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t
                        as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t
                            as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize],
                    );
        }
        t = ((*ctx).Km[11 as i32 as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[11 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[11 as i32 as usize] as i32) & 31 as i32);
        r
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[10 as i32 as usize] ^ r;
        t = t << (*ctx).Kr[10 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[10 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[9 as i32 as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[9 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[9 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = ((*ctx).Km[8 as i32 as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[8 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[8 as i32 as usize] as i32) & 31 as i32);
        l
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[7 as i32 as usize] ^ l;
        t = t << (*ctx).Kr[7 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[7 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[6 as i32 as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[6 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[6 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = ((*ctx).Km[5 as i32 as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[5 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[5 as i32 as usize] as i32) & 31 as i32);
        r
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[4 as i32 as usize] ^ r;
        t = t << (*ctx).Kr[4 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[4 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[3 as i32 as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[3 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[3 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = ((*ctx).Km[2 as i32 as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[2 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[2 as i32 as usize] as i32) & 31 as i32);
        l
            ^= ((cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        t = (*ctx).Km[1 as i32 as usize] ^ l;
        t = t << (*ctx).Kr[1 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[1 as i32 as usize] as i32) & 31 as i32);
        r
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                ) ^ cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize];
        t = ((*ctx).Km[0 as i32 as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[0 as i32 as usize] as i32
            | t >> (-((*ctx).Kr[0 as i32 as usize] as i32) & 31 as i32);
        l
            ^= (cast_sbox1[(t >> 24 as i32) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize],
                )
                .wrapping_add(cast_sbox4[(t & 0xff as i32 as u32) as uint8_t as usize]);
        *dst.offset(0 as i32 as isize) = (l >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(1 as i32 as isize) = (l >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(2 as i32 as isize) = (l >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset(3 as i32 as isize) = (l & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(0 as i32 as isize) = (r >> 24 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(1 as i32 as isize) = (r >> 16 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(2 as i32 as isize) = (r >> 8 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize).offset(3 as i32 as isize) = (r
            & 0xff as i32 as u32) as uint8_t;
        length = (length as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
        dst = dst.offset(8 as i32 as isize);
        src = src.offset(8 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cast5_set_key(
    mut ctx: *mut cast128_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    let mut x0: uint32_t = 0;
    let mut x1: uint32_t = 0;
    let mut x2: uint32_t = 0;
    let mut x3: uint32_t = 0;
    let mut z0: uint32_t = 0;
    let mut z1: uint32_t = 0;
    let mut z2: uint32_t = 0;
    let mut z3: uint32_t = 0;
    let mut w: uint32_t = 0;
    let mut full: i32 = 0;
    if length >= 5 as i32 as u64 {} else {
        __assert_fail(
            b"length >= CAST5_MIN_KEY_SIZE\0" as *const u8 as *const i8,
            b"cast128.c\0" as *const u8 as *const i8,
            233 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8680: {
        if length >= 5 as i32 as u64 {} else {
            __assert_fail(
                b"length >= CAST5_MIN_KEY_SIZE\0" as *const u8 as *const i8,
                b"cast128.c\0" as *const u8 as *const i8,
                233 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"length <= CAST5_MAX_KEY_SIZE\0" as *const u8 as *const i8,
            b"cast128.c\0" as *const u8 as *const i8,
            234 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8637: {
        if length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"length <= CAST5_MAX_KEY_SIZE\0" as *const u8 as *const i8,
                b"cast128.c\0" as *const u8 as *const i8,
                234 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    full = (length > 10 as i32 as u64) as i32;
    x0 = (*key.offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(1 as i32 as isize) as uint32_t) << 16 as i32
        | (*key.offset(2 as i32 as isize) as uint32_t) << 8 as i32
        | *key.offset(3 as i32 as isize) as uint32_t;
    match length & 3 as i32 as u64 {
        0 => {
            w = (*key
                .offset(length as isize)
                .offset(-(4 as i32 as isize))
                .offset(0 as i32 as isize) as uint32_t) << 24 as i32
                | (*key
                    .offset(length as isize)
                    .offset(-(4 as i32 as isize))
                    .offset(1 as i32 as isize) as uint32_t) << 16 as i32
                | (*key
                    .offset(length as isize)
                    .offset(-(4 as i32 as isize))
                    .offset(2 as i32 as isize) as uint32_t) << 8 as i32
                | *key
                    .offset(length as isize)
                    .offset(-(4 as i32 as isize))
                    .offset(3 as i32 as isize) as uint32_t;
        }
        3 => {
            w = ((*key
                .offset(length as isize)
                .offset(-(3 as i32 as isize))
                .offset(0 as i32 as isize) as uint32_t) << 16 as i32
                | (*key
                    .offset(length as isize)
                    .offset(-(3 as i32 as isize))
                    .offset(1 as i32 as isize) as uint32_t) << 8 as i32
                | *key
                    .offset(length as isize)
                    .offset(-(3 as i32 as isize))
                    .offset(2 as i32 as isize) as uint32_t) << 8 as i32;
        }
        2 => {
            w = ((*key
                .offset(length as isize)
                .offset(-(2 as i32 as isize))
                .offset(0 as i32 as isize) as uint32_t) << 8 as i32
                | *key
                    .offset(length as isize)
                    .offset(-(2 as i32 as isize))
                    .offset(1 as i32 as isize) as uint32_t) << 16 as i32;
        }
        1 => {
            w = (*key.offset(length.wrapping_sub(1 as i32 as u64) as isize) as uint32_t)
                << 24 as i32;
        }
        _ => {}
    }
    if length <= 8 as i32 as u64 {
        x1 = w;
        x3 = 0 as i32 as uint32_t;
        x2 = x3;
    } else {
        x1 = (*key.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
            << 24 as i32
            | (*key.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
                << 16 as i32
            | (*key.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
                << 8 as i32
            | *key.offset(4 as i32 as isize).offset(3 as i32 as isize) as uint32_t;
        if length <= 12 as i32 as u64 {
            x2 = w;
            x3 = 0 as i32 as uint32_t;
        } else {
            x2 = (*key.offset(8 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
                << 24 as i32
                | (*key.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
                    << 16 as i32
                | (*key.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
                    << 8 as i32
                | *key.offset(8 as i32 as isize).offset(3 as i32 as isize) as uint32_t;
            x3 = w;
        }
    }
    z0 = x0 ^ cast_sbox5[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z3 = x1 ^ cast_sbox5[(z2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[0 as i32 as usize] = cast_sbox5[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[1 as i32 as usize] = cast_sbox5[(z2 >> 8 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[2 as i32 as usize] = cast_sbox5[(z3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[3 as i32 as usize] = cast_sbox5[(z3 >> 8 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 24 as i32) as uint8_t as usize];
    x0 = z2 ^ cast_sbox5[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as i32) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x3 = z3 ^ cast_sbox5[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[4 as i32 as usize] = cast_sbox5[(x0 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 24 as i32) as uint8_t as usize];
    (*ctx).Km[5 as i32 as usize] = cast_sbox5[(x0 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(x0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[6 as i32 as usize] = cast_sbox5[(x1 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x0 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[7 as i32 as usize] = cast_sbox5[(x1 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(x1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x1 & 0xff as i32 as u32) as uint8_t as usize];
    z0 = x0 ^ cast_sbox5[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z3 = x1 ^ cast_sbox5[(z2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[8 as i32 as usize] = cast_sbox5[(z0 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox5[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[9 as i32 as usize] = cast_sbox5[(z0 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 24 as i32) as uint8_t as usize];
    (*ctx).Km[10 as i32 as usize] = cast_sbox5[(z1 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Km[11 as i32 as usize] = cast_sbox5[(z1 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x0 = z2 ^ cast_sbox5[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as i32) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x3 = z3 ^ cast_sbox5[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as i32 as u32) as uint8_t as usize];
    if full != 0 {
        (*ctx).Km[12 as i32 as usize] = cast_sbox5[(x2 >> 24 as i32) as uint8_t as usize]
            ^ cast_sbox6[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x1 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox5[(x0 & 0xff as i32 as u32) as uint8_t as usize];
        (*ctx).Km[13 as i32 as usize] = cast_sbox5[(x2 >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t as usize]
            ^ cast_sbox6[(x2 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 24 as i32) as uint8_t as usize]
            ^ cast_sbox6[(x1 & 0xff as i32 as u32) as uint8_t as usize];
        (*ctx).Km[14 as i32 as usize] = cast_sbox5[(x3 >> 24 as i32) as uint8_t as usize]
            ^ cast_sbox6[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x0 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize];
        (*ctx).Km[15 as i32 as usize] = cast_sbox5[(x3 >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t as usize]
            ^ cast_sbox6[(x3 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 24 as i32) as uint8_t as usize]
            ^ cast_sbox8[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    }
    z0 = x0 ^ cast_sbox5[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z3 = x1 ^ cast_sbox5[(z2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Kr[0 as i32 as usize] = ((cast_sbox5[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    (*ctx).Kr[1 as i32 as usize] = ((cast_sbox5[(z2 >> 8 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    (*ctx).Kr[2 as i32 as usize] = ((cast_sbox5[(z3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    (*ctx).Kr[3 as i32 as usize] = ((cast_sbox5[(z3 >> 8 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 24 as i32) as uint8_t as usize]) & 31 as i32 as u32) as u8;
    x0 = z2 ^ cast_sbox5[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as i32) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x3 = z3 ^ cast_sbox5[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Kr[4 as i32 as usize] = ((cast_sbox5[(x0 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 24 as i32) as uint8_t as usize]) & 31 as i32 as u32) as u8;
    (*ctx).Kr[5 as i32 as usize] = ((cast_sbox5[(x0 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(x0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    (*ctx).Kr[6 as i32 as usize] = ((cast_sbox5[(x1 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x0 & 0xff as i32 as u32) as uint8_t as usize]) & 31 as i32 as u32)
        as u8;
    (*ctx).Kr[7 as i32 as usize] = ((cast_sbox5[(x1 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(x1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x1 & 0xff as i32 as u32) as uint8_t as usize]) & 31 as i32 as u32)
        as u8;
    z0 = x0 ^ cast_sbox5[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    z3 = x1 ^ cast_sbox5[(z2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as i32 as u32) as uint8_t as usize];
    (*ctx).Kr[8 as i32 as usize] = ((cast_sbox5[(z0 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox5[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    (*ctx).Kr[9 as i32 as usize] = ((cast_sbox5[(z0 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z3 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 24 as i32) as uint8_t as usize]) & 31 as i32 as u32) as u8;
    (*ctx).Kr[10 as i32 as usize] = ((cast_sbox5[(z1 & 0xff as i32 as u32) as uint8_t
        as usize] ^ cast_sbox6[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    (*ctx).Kr[11 as i32 as usize] = ((cast_sbox5[(z1 >> 16 as i32 & 0xff as i32 as u32)
        as uint8_t as usize] ^ cast_sbox6[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize])
        & 31 as i32 as u32) as u8;
    x0 = z2 ^ cast_sbox5[(z1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as i32) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize];
    x3 = z3 ^ cast_sbox5[(x2 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as i32 as u32) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as i32) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as i32 as u32) as uint8_t as usize];
    if full != 0 {
        (*ctx).Kr[12 as i32 as usize] = ((cast_sbox5[(x2 >> 24 as i32) as uint8_t
            as usize]
            ^ cast_sbox6[(x2 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x1 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox5[(x0 & 0xff as i32 as u32) as uint8_t as usize])
            & 31 as i32 as u32) as u8;
        (*ctx).Kr[13 as i32 as usize] = ((cast_sbox5[(x2 >> 8 as i32
            & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox6[(x2 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x1 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 24 as i32) as uint8_t as usize]
            ^ cast_sbox6[(x1 & 0xff as i32 as u32) as uint8_t as usize])
            & 31 as i32 as u32) as u8;
        (*ctx).Kr[14 as i32 as usize] = ((cast_sbox5[(x3 >> 24 as i32) as uint8_t
            as usize]
            ^ cast_sbox6[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x0 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 8 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x2 >> 24 as i32) as uint8_t as usize]) & 31 as i32 as u32)
            as u8;
        (*ctx).Kr[15 as i32 as usize] = ((cast_sbox5[(x3 >> 8 as i32
            & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox6[(x3 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox7[(x0 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 24 as i32) as uint8_t as usize]
            ^ cast_sbox8[(x3 >> 16 as i32 & 0xff as i32 as u32) as uint8_t as usize])
            & 31 as i32 as u32) as u8;
    }
    (*ctx).rounds = (if full != 0 { 16 as i32 } else { 12 as i32 }) as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cast128_set_key(
    mut ctx: *mut cast128_ctx,
    mut key: *const uint8_t,
) {
    nettle_cast5_set_key(ctx, 16 as i32 as size_t, key);
}