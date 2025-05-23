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
pub struct blowfish_ctx {
    pub s: [[uint32_t; 256]; 4],
    pub p: [uint32_t; 18],
}
#[no_mangle]
pub static mut _nettle_blowfish_initial_ctx: blowfish_ctx = {
    let mut init = blowfish_ctx {
        s: [
            [
                0xd1310ba6 as u32,
                0x98dfb5ac as u32,
                0x2ffd72db as i32 as uint32_t,
                0xd01adfb7 as u32,
                0xb8e1afed as u32,
                0x6a267e96 as i32 as uint32_t,
                0xba7c9045 as u32,
                0xf12c7f99 as u32,
                0x24a19947 as i32 as uint32_t,
                0xb3916cf7 as u32,
                0x801f2e2 as i32 as uint32_t,
                0x858efc16 as u32,
                0x636920d8 as i32 as uint32_t,
                0x71574e69 as i32 as uint32_t,
                0xa458fea3 as u32,
                0xf4933d7e as u32,
                0xd95748f as i32 as uint32_t,
                0x728eb658 as i32 as uint32_t,
                0x718bcd58 as i32 as uint32_t,
                0x82154aee as u32,
                0x7b54a41d as i32 as uint32_t,
                0xc25a59b5 as u32,
                0x9c30d539 as u32,
                0x2af26013 as i32 as uint32_t,
                0xc5d1b023 as u32,
                0x286085f0 as i32 as uint32_t,
                0xca417918 as u32,
                0xb8db38ef as u32,
                0x8e79dcb0 as u32,
                0x603a180e as i32 as uint32_t,
                0x6c9e0e8b as i32 as uint32_t,
                0xb01e8a3e as u32,
                0xd71577c1 as u32,
                0xbd314b27 as u32,
                0x78af2fda as i32 as uint32_t,
                0x55605c60 as i32 as uint32_t,
                0xe65525f3 as u32,
                0xaa55ab94 as u32,
                0x57489862 as i32 as uint32_t,
                0x63e81440 as i32 as uint32_t,
                0x55ca396a as i32 as uint32_t,
                0x2aab10b6 as i32 as uint32_t,
                0xb4cc5c34 as u32,
                0x1141e8ce as i32 as uint32_t,
                0xa15486af as u32,
                0x7c72e993 as i32 as uint32_t,
                0xb3ee1411 as u32,
                0x636fbc2a as i32 as uint32_t,
                0x2ba9c55d as i32 as uint32_t,
                0x741831f6 as i32 as uint32_t,
                0xce5c3e16 as u32,
                0x9b87931e as u32,
                0xafd6ba33 as u32,
                0x6c24cf5c as i32 as uint32_t,
                0x7a325381 as i32 as uint32_t,
                0x28958677 as i32 as uint32_t,
                0x3b8f4898 as i32 as uint32_t,
                0x6b4bb9af as i32 as uint32_t,
                0xc4bfe81b as u32,
                0x66282193 as i32 as uint32_t,
                0x61d809cc as i32 as uint32_t,
                0xfb21a991 as u32,
                0x487cac60 as i32 as uint32_t,
                0x5dec8032 as i32 as uint32_t,
                0xef845d5d as u32,
                0xe98575b1 as u32,
                0xdc262302 as u32,
                0xeb651b88 as u32,
                0x23893e81 as i32 as uint32_t,
                0xd396acc5 as u32,
                0xf6d6ff3 as i32 as uint32_t,
                0x83f44239 as u32,
                0x2e0b4482 as i32 as uint32_t,
                0xa4842004 as u32,
                0x69c8f04a as i32 as uint32_t,
                0x9e1f9b5e as u32,
                0x21c66842 as i32 as uint32_t,
                0xf6e96c9a as u32,
                0x670c9c61 as i32 as uint32_t,
                0xabd388f0 as u32,
                0x6a51a0d2 as i32 as uint32_t,
                0xd8542f68 as u32,
                0x960fa728 as u32,
                0xab5133a3 as u32,
                0x6eef0b6c as i32 as uint32_t,
                0x137a3be4 as i32 as uint32_t,
                0xba3bf050 as u32,
                0x7efb2a98 as i32 as uint32_t,
                0xa1f1651d as u32,
                0x39af0176 as i32 as uint32_t,
                0x66ca593e as i32 as uint32_t,
                0x82430e88 as u32,
                0x8cee8619 as u32,
                0x456f9fb4 as i32 as uint32_t,
                0x7d84a5c3 as i32 as uint32_t,
                0x3b8b5ebe as i32 as uint32_t,
                0xe06f75d8 as u32,
                0x85c12073 as u32,
                0x401a449f as i32 as uint32_t,
                0x56c16aa6 as i32 as uint32_t,
                0x4ed3aa62 as i32 as uint32_t,
                0x363f7706 as i32 as uint32_t,
                0x1bfedf72 as i32 as uint32_t,
                0x429b023d as i32 as uint32_t,
                0x37d0d724 as i32 as uint32_t,
                0xd00a1248 as u32,
                0xdb0fead3 as u32,
                0x49f1c09b as i32 as uint32_t,
                0x75372c9 as i32 as uint32_t,
                0x80991b7b as u32,
                0x25d479d8 as i32 as uint32_t,
                0xf6e8def7 as u32,
                0xe3fe501a as u32,
                0xb6794c3b as u32,
                0x976ce0bd as u32,
                0x4c006ba as i32 as uint32_t,
                0xc1a94fb6 as u32,
                0x409f60c4 as i32 as uint32_t,
                0x5e5c9ec2 as i32 as uint32_t,
                0x196a2463 as i32 as uint32_t,
                0x68fb6faf as i32 as uint32_t,
                0x3e6c53b5 as i32 as uint32_t,
                0x1339b2eb as i32 as uint32_t,
                0x3b52ec6f as i32 as uint32_t,
                0x6dfc511f as i32 as uint32_t,
                0x9b30952c as u32,
                0xcc814544 as u32,
                0xaf5ebd09 as u32,
                0xbee3d004 as u32,
                0xde334afd as u32,
                0x660f2807 as i32 as uint32_t,
                0x192e4bb3 as i32 as uint32_t,
                0xc0cba857 as u32,
                0x45c8740f as i32 as uint32_t,
                0xd20b5f39 as u32,
                0xb9d3fbdb as u32,
                0x5579c0bd as i32 as uint32_t,
                0x1a60320a as i32 as uint32_t,
                0xd6a100c6 as u32,
                0x402c7279 as i32 as uint32_t,
                0x679f25fe as i32 as uint32_t,
                0xfb1fa3cc as u32,
                0x8ea5e9f8 as u32,
                0xdb3222f8 as u32,
                0x3c7516df as i32 as uint32_t,
                0xfd616b15 as u32,
                0x2f501ec8 as i32 as uint32_t,
                0xad0552ab as u32,
                0x323db5fa as i32 as uint32_t,
                0xfd238760 as u32,
                0x53317b48 as i32 as uint32_t,
                0x3e00df82 as i32 as uint32_t,
                0x9e5c57bb as u32,
                0xca6f8ca0 as u32,
                0x1a87562e as i32 as uint32_t,
                0xdf1769db as u32,
                0xd542a8f6 as u32,
                0x287effc3 as i32 as uint32_t,
                0xac6732c6 as u32,
                0x8c4f5573 as u32,
                0x695b27b0 as i32 as uint32_t,
                0xbbca58c8 as u32,
                0xe1ffa35d as u32,
                0xb8f011a0 as u32,
                0x10fa3d98 as i32 as uint32_t,
                0xfd2183b8 as u32,
                0x4afcb56c as i32 as uint32_t,
                0x2dd1d35b as i32 as uint32_t,
                0x9a53e479 as u32,
                0xb6f84565 as u32,
                0xd28e49bc as u32,
                0x4bfb9790 as i32 as uint32_t,
                0xe1ddf2da as u32,
                0xa4cb7e33 as u32,
                0x62fb1341 as i32 as uint32_t,
                0xcee4c6e8 as u32,
                0xef20cada as u32,
                0x36774c01 as i32 as uint32_t,
                0xd07e9efe as u32,
                0x2bf11fb4 as i32 as uint32_t,
                0x95dbda4d as u32,
                0xae909198 as u32,
                0xeaad8e71 as u32,
                0x6b93d5a0 as i32 as uint32_t,
                0xd08ed1d0 as u32,
                0xafc725e0 as u32,
                0x8e3c5b2f as u32,
                0x8e7594b7 as u32,
                0x8ff6e2fb as u32,
                0xf2122b64 as u32,
                0x8888b812 as u32,
                0x900df01c as u32,
                0x4fad5ea0 as i32 as uint32_t,
                0x688fc31c as i32 as uint32_t,
                0xd1cff191 as u32,
                0xb3a8c1ad as u32,
                0x2f2f2218 as i32 as uint32_t,
                0xbe0e1777 as u32,
                0xea752dfe as u32,
                0x8b021fa1 as u32,
                0xe5a0cc0f as u32,
                0xb56f74e8 as u32,
                0x18acf3d6 as i32 as uint32_t,
                0xce89e299 as u32,
                0xb4a84fe0 as u32,
                0xfd13e0b7 as u32,
                0x7cc43b81 as i32 as uint32_t,
                0xd2ada8d9 as u32,
                0x165fa266 as i32 as uint32_t,
                0x80957705 as u32,
                0x93cc7314 as u32,
                0x211a1477 as i32 as uint32_t,
                0xe6ad2065 as u32,
                0x77b5fa86 as i32 as uint32_t,
                0xc75442f5 as u32,
                0xfb9d35cf as u32,
                0xebcdaf0c as u32,
                0x7b3e89a0 as i32 as uint32_t,
                0xd6411bd3 as u32,
                0xae1e7e49 as u32,
                0x250e2d as i32 as uint32_t,
                0x2071b35e as i32 as uint32_t,
                0x226800bb as i32 as uint32_t,
                0x57b8e0af as i32 as uint32_t,
                0x2464369b as i32 as uint32_t,
                0xf009b91e as u32,
                0x5563911d as i32 as uint32_t,
                0x59dfa6aa as i32 as uint32_t,
                0x78c14389 as i32 as uint32_t,
                0xd95a537f as u32,
                0x207d5ba2 as i32 as uint32_t,
                0x2e5b9c5 as i32 as uint32_t,
                0x83260376 as u32,
                0x6295cfa9 as i32 as uint32_t,
                0x11c81968 as i32 as uint32_t,
                0x4e734a41 as i32 as uint32_t,
                0xb3472dca as u32,
                0x7b14a94a as i32 as uint32_t,
                0x1b510052 as i32 as uint32_t,
                0x9a532915 as u32,
                0xd60f573f as u32,
                0xbc9bc6e4 as u32,
                0x2b60a476 as i32 as uint32_t,
                0x81e67400 as u32,
                0x8ba6fb5 as i32 as uint32_t,
                0x571be91f as i32 as uint32_t,
                0xf296ec6b as u32,
                0x2a0dd915 as i32 as uint32_t,
                0xb6636521 as u32,
                0xe7b9f9b6 as u32,
                0xff34052e as u32,
                0xc5855664 as u32,
                0x53b02d5d as i32 as uint32_t,
                0xa99f8fa1 as u32,
                0x8ba4799 as i32 as uint32_t,
                0x6e85076a as i32 as uint32_t,
            ],
            [
                0x4b7a70e9 as i32 as uint32_t,
                0xb5b32944 as u32,
                0xdb75092e as u32,
                0xc4192623 as u32,
                0xad6ea6b0 as u32,
                0x49a7df7d as i32 as uint32_t,
                0x9cee60b8 as u32,
                0x8fedb266 as u32,
                0xecaa8c71 as u32,
                0x699a17ff as i32 as uint32_t,
                0x5664526c as i32 as uint32_t,
                0xc2b19ee1 as u32,
                0x193602a5 as i32 as uint32_t,
                0x75094c29 as i32 as uint32_t,
                0xa0591340 as u32,
                0xe4183a3e as u32,
                0x3f54989a as i32 as uint32_t,
                0x5b429d65 as i32 as uint32_t,
                0x6b8fe4d6 as i32 as uint32_t,
                0x99f73fd6 as u32,
                0xa1d29c07 as u32,
                0xefe830f5 as u32,
                0x4d2d38e6 as i32 as uint32_t,
                0xf0255dc1 as u32,
                0x4cdd2086 as i32 as uint32_t,
                0x8470eb26 as u32,
                0x6382e9c6 as i32 as uint32_t,
                0x21ecc5e as i32 as uint32_t,
                0x9686b3f as i32 as uint32_t,
                0x3ebaefc9 as i32 as uint32_t,
                0x3c971814 as i32 as uint32_t,
                0x6b6a70a1 as i32 as uint32_t,
                0x687f3584 as i32 as uint32_t,
                0x52a0e286 as i32 as uint32_t,
                0xb79c5305 as u32,
                0xaa500737 as u32,
                0x3e07841c as i32 as uint32_t,
                0x7fdeae5c as i32 as uint32_t,
                0x8e7d44ec as u32,
                0x5716f2b8 as i32 as uint32_t,
                0xb03ada37 as u32,
                0xf0500c0d as u32,
                0xf01c1f04 as u32,
                0x200b3ff as i32 as uint32_t,
                0xae0cf51a as u32,
                0x3cb574b2 as i32 as uint32_t,
                0x25837a58 as i32 as uint32_t,
                0xdc0921bd as u32,
                0xd19113f9 as u32,
                0x7ca92ff6 as i32 as uint32_t,
                0x94324773 as u32,
                0x22f54701 as i32 as uint32_t,
                0x3ae5e581 as i32 as uint32_t,
                0x37c2dadc as i32 as uint32_t,
                0xc8b57634 as u32,
                0x9af3dda7 as u32,
                0xa9446146 as u32,
                0xfd0030e as i32 as uint32_t,
                0xecc8c73e as u32,
                0xa4751e41 as u32,
                0xe238cd99 as u32,
                0x3bea0e2f as i32 as uint32_t,
                0x3280bba1 as i32 as uint32_t,
                0x183eb331 as i32 as uint32_t,
                0x4e548b38 as i32 as uint32_t,
                0x4f6db908 as i32 as uint32_t,
                0x6f420d03 as i32 as uint32_t,
                0xf60a04bf as u32,
                0x2cb81290 as i32 as uint32_t,
                0x24977c79 as i32 as uint32_t,
                0x5679b072 as i32 as uint32_t,
                0xbcaf89af as u32,
                0xde9a771f as u32,
                0xd9930810 as u32,
                0xb38bae12 as u32,
                0xdccf3f2e as u32,
                0x5512721f as i32 as uint32_t,
                0x2e6b7124 as i32 as uint32_t,
                0x501adde6 as i32 as uint32_t,
                0x9f84cd87 as u32,
                0x7a584718 as i32 as uint32_t,
                0x7408da17 as i32 as uint32_t,
                0xbc9f9abc as u32,
                0xe94b7d8c as u32,
                0xec7aec3a as u32,
                0xdb851dfa as u32,
                0x63094366 as i32 as uint32_t,
                0xc464c3d2 as u32,
                0xef1c1847 as u32,
                0x3215d908 as i32 as uint32_t,
                0xdd433b37 as u32,
                0x24c2ba16 as i32 as uint32_t,
                0x12a14d43 as i32 as uint32_t,
                0x2a65c451 as i32 as uint32_t,
                0x50940002 as i32 as uint32_t,
                0x133ae4dd as i32 as uint32_t,
                0x71dff89e as i32 as uint32_t,
                0x10314e55 as i32 as uint32_t,
                0x81ac77d6 as u32,
                0x5f11199b as i32 as uint32_t,
                0x43556f1 as i32 as uint32_t,
                0xd7a3c76b as u32,
                0x3c11183b as i32 as uint32_t,
                0x5924a509 as i32 as uint32_t,
                0xf28fe6ed as u32,
                0x97f1fbfa as u32,
                0x9ebabf2c as u32,
                0x1e153c6e as i32 as uint32_t,
                0x86e34570 as u32,
                0xeae96fb1 as u32,
                0x860e5e0a as u32,
                0x5a3e2ab3 as i32 as uint32_t,
                0x771fe71c as i32 as uint32_t,
                0x4e3d06fa as i32 as uint32_t,
                0x2965dcb9 as i32 as uint32_t,
                0x99e71d0f as u32,
                0x803e89d6 as u32,
                0x5266c825 as i32 as uint32_t,
                0x2e4cc978 as i32 as uint32_t,
                0x9c10b36a as u32,
                0xc6150eba as u32,
                0x94e2ea78 as u32,
                0xa5fc3c53 as u32,
                0x1e0a2df4 as i32 as uint32_t,
                0xf2f74ea7 as u32,
                0x361d2b3d as i32 as uint32_t,
                0x1939260f as i32 as uint32_t,
                0x19c27960 as i32 as uint32_t,
                0x5223a708 as i32 as uint32_t,
                0xf71312b6 as u32,
                0xebadfe6e as u32,
                0xeac31f66 as u32,
                0xe3bc4595 as u32,
                0xa67bc883 as u32,
                0xb17f37d1 as u32,
                0x18cff28 as i32 as uint32_t,
                0xc332ddef as u32,
                0xbe6c5aa5 as u32,
                0x65582185 as i32 as uint32_t,
                0x68ab9802 as i32 as uint32_t,
                0xeecea50f as u32,
                0xdb2f953b as u32,
                0x2aef7dad as i32 as uint32_t,
                0x5b6e2f84 as i32 as uint32_t,
                0x1521b628 as i32 as uint32_t,
                0x29076170 as i32 as uint32_t,
                0xecdd4775 as u32,
                0x619f1510 as i32 as uint32_t,
                0x13cca830 as i32 as uint32_t,
                0xeb61bd96 as u32,
                0x334fe1e as i32 as uint32_t,
                0xaa0363cf as u32,
                0xb5735c90 as u32,
                0x4c70a239 as i32 as uint32_t,
                0xd59e9e0b as u32,
                0xcbaade14 as u32,
                0xeecc86bc as u32,
                0x60622ca7 as i32 as uint32_t,
                0x9cab5cab as u32,
                0xb2f3846e as u32,
                0x648b1eaf as i32 as uint32_t,
                0x19bdf0ca as i32 as uint32_t,
                0xa02369b9 as u32,
                0x655abb50 as i32 as uint32_t,
                0x40685a32 as i32 as uint32_t,
                0x3c2ab4b3 as i32 as uint32_t,
                0x319ee9d5 as i32 as uint32_t,
                0xc021b8f7 as u32,
                0x9b540b19 as u32,
                0x875fa099 as u32,
                0x95f7997e as u32,
                0x623d7da8 as i32 as uint32_t,
                0xf837889a as u32,
                0x97e32d77 as u32,
                0x11ed935f as i32 as uint32_t,
                0x16681281 as i32 as uint32_t,
                0xe358829 as i32 as uint32_t,
                0xc7e61fd6 as u32,
                0x96dedfa1 as u32,
                0x7858ba99 as i32 as uint32_t,
                0x57f584a5 as i32 as uint32_t,
                0x1b227263 as i32 as uint32_t,
                0x9b83c3ff as u32,
                0x1ac24696 as i32 as uint32_t,
                0xcdb30aeb as u32,
                0x532e3054 as i32 as uint32_t,
                0x8fd948e4 as u32,
                0x6dbc3128 as i32 as uint32_t,
                0x58ebf2ef as i32 as uint32_t,
                0x34c6ffea as i32 as uint32_t,
                0xfe28ed61 as u32,
                0xee7c3c73 as u32,
                0x5d4a14d9 as i32 as uint32_t,
                0xe864b7e3 as u32,
                0x42105d14 as i32 as uint32_t,
                0x203e13e0 as i32 as uint32_t,
                0x45eee2b6 as i32 as uint32_t,
                0xa3aaabea as u32,
                0xdb6c4f15 as u32,
                0xfacb4fd0 as u32,
                0xc742f442 as u32,
                0xef6abbb5 as u32,
                0x654f3b1d as i32 as uint32_t,
                0x41cd2105 as i32 as uint32_t,
                0xd81e799e as u32,
                0x86854dc7 as u32,
                0xe44b476a as u32,
                0x3d816250 as i32 as uint32_t,
                0xcf62a1f2 as u32,
                0x5b8d2646 as i32 as uint32_t,
                0xfc8883a0 as u32,
                0xc1c7b6a3 as u32,
                0x7f1524c3 as i32 as uint32_t,
                0x69cb7492 as i32 as uint32_t,
                0x47848a0b as i32 as uint32_t,
                0x5692b285 as i32 as uint32_t,
                0x95bbf00 as i32 as uint32_t,
                0xad19489d as u32,
                0x1462b174 as i32 as uint32_t,
                0x23820e00 as i32 as uint32_t,
                0x58428d2a as i32 as uint32_t,
                0xc55f5ea as i32 as uint32_t,
                0x1dadf43e as i32 as uint32_t,
                0x233f7061 as i32 as uint32_t,
                0x3372f092 as i32 as uint32_t,
                0x8d937e41 as u32,
                0xd65fecf1 as u32,
                0x6c223bdb as i32 as uint32_t,
                0x7cde3759 as i32 as uint32_t,
                0xcbee7460 as u32,
                0x4085f2a7 as i32 as uint32_t,
                0xce77326e as u32,
                0xa6078084 as u32,
                0x19f8509e as i32 as uint32_t,
                0xe8efd855 as u32,
                0x61d99735 as i32 as uint32_t,
                0xa969a7aa as u32,
                0xc50c06c2 as u32,
                0x5a04abfc as i32 as uint32_t,
                0x800bcadc as u32,
                0x9e447a2e as u32,
                0xc3453484 as u32,
                0xfdd56705 as u32,
                0xe1e9ec9 as i32 as uint32_t,
                0xdb73dbd3 as u32,
                0x105588cd as i32 as uint32_t,
                0x675fda79 as i32 as uint32_t,
                0xe3674340 as u32,
                0xc5c43465 as u32,
                0x713e38d8 as i32 as uint32_t,
                0x3d28f89e as i32 as uint32_t,
                0xf16dff20 as u32,
                0x153e21e7 as i32 as uint32_t,
                0x8fb03d4a as u32,
                0xe6e39f2b as u32,
                0xdb83adf7 as u32,
            ],
            [
                0xe93d5a68 as u32,
                0x948140f7 as u32,
                0xf64c261c as u32,
                0x94692934 as u32,
                0x411520f7 as i32 as uint32_t,
                0x7602d4f7 as i32 as uint32_t,
                0xbcf46b2e as u32,
                0xd4a20068 as u32,
                0xd4082471 as u32,
                0x3320f46a as i32 as uint32_t,
                0x43b7d4b7 as i32 as uint32_t,
                0x500061af as i32 as uint32_t,
                0x1e39f62e as i32 as uint32_t,
                0x97244546 as u32,
                0x14214f74 as i32 as uint32_t,
                0xbf8b8840 as u32,
                0x4d95fc1d as i32 as uint32_t,
                0x96b591af as u32,
                0x70f4ddd3 as i32 as uint32_t,
                0x66a02f45 as i32 as uint32_t,
                0xbfbc09ec as u32,
                0x3bd9785 as i32 as uint32_t,
                0x7fac6dd0 as i32 as uint32_t,
                0x31cb8504 as i32 as uint32_t,
                0x96eb27b3 as u32,
                0x55fd3941 as i32 as uint32_t,
                0xda2547e6 as u32,
                0xabca0a9a as u32,
                0x28507825 as i32 as uint32_t,
                0x530429f4 as i32 as uint32_t,
                0xa2c86da as i32 as uint32_t,
                0xe9b66dfb as u32,
                0x68dc1462 as i32 as uint32_t,
                0xd7486900 as u32,
                0x680ec0a4 as i32 as uint32_t,
                0x27a18dee as i32 as uint32_t,
                0x4f3ffea2 as i32 as uint32_t,
                0xe887ad8c as u32,
                0xb58ce006 as u32,
                0x7af4d6b6 as i32 as uint32_t,
                0xaace1e7c as u32,
                0xd3375fec as u32,
                0xce78a399 as u32,
                0x406b2a42 as i32 as uint32_t,
                0x20fe9e35 as i32 as uint32_t,
                0xd9f385b9 as u32,
                0xee39d7ab as u32,
                0x3b124e8b as i32 as uint32_t,
                0x1dc9faf7 as i32 as uint32_t,
                0x4b6d1856 as i32 as uint32_t,
                0x26a36631 as i32 as uint32_t,
                0xeae397b2 as u32,
                0x3a6efa74 as i32 as uint32_t,
                0xdd5b4332 as u32,
                0x6841e7f7 as i32 as uint32_t,
                0xca7820fb as u32,
                0xfb0af54e as u32,
                0xd8feb397 as u32,
                0x454056ac as i32 as uint32_t,
                0xba489527 as u32,
                0x55533a3a as i32 as uint32_t,
                0x20838d87 as i32 as uint32_t,
                0xfe6ba9b7 as u32,
                0xd096954b as u32,
                0x55a867bc as i32 as uint32_t,
                0xa1159a58 as u32,
                0xcca92963 as u32,
                0x99e1db33 as u32,
                0xa62a4a56 as u32,
                0x3f3125f9 as i32 as uint32_t,
                0x5ef47e1c as i32 as uint32_t,
                0x9029317c as u32,
                0xfdf8e802 as u32,
                0x4272f70 as i32 as uint32_t,
                0x80bb155c as u32,
                0x5282ce3 as i32 as uint32_t,
                0x95c11548 as u32,
                0xe4c66d22 as u32,
                0x48c1133f as i32 as uint32_t,
                0xc70f86dc as u32,
                0x7f9c9ee as i32 as uint32_t,
                0x41041f0f as i32 as uint32_t,
                0x404779a4 as i32 as uint32_t,
                0x5d886e17 as i32 as uint32_t,
                0x325f51eb as i32 as uint32_t,
                0xd59bc0d1 as u32,
                0xf2bcc18f as u32,
                0x41113564 as i32 as uint32_t,
                0x257b7834 as i32 as uint32_t,
                0x602a9c60 as i32 as uint32_t,
                0xdff8e8a3 as u32,
                0x1f636c1b as i32 as uint32_t,
                0xe12b4c2 as i32 as uint32_t,
                0x2e1329e as i32 as uint32_t,
                0xaf664fd1 as u32,
                0xcad18115 as u32,
                0x6b2395e0 as i32 as uint32_t,
                0x333e92e1 as i32 as uint32_t,
                0x3b240b62 as i32 as uint32_t,
                0xeebeb922 as u32,
                0x85b2a20e as u32,
                0xe6ba0d99 as u32,
                0xde720c8c as u32,
                0x2da2f728 as i32 as uint32_t,
                0xd0127845 as u32,
                0x95b794fd as u32,
                0x647d0862 as i32 as uint32_t,
                0xe7ccf5f0 as u32,
                0x5449a36f as i32 as uint32_t,
                0x877d48fa as u32,
                0xc39dfd27 as u32,
                0xf33e8d1e as u32,
                0xa476341 as i32 as uint32_t,
                0x992eff74 as u32,
                0x3a6f6eab as i32 as uint32_t,
                0xf4f8fd37 as u32,
                0xa812dc60 as u32,
                0xa1ebddf8 as u32,
                0x991be14c as u32,
                0xdb6e6b0d as u32,
                0xc67b5510 as u32,
                0x6d672c37 as i32 as uint32_t,
                0x2765d43b as i32 as uint32_t,
                0xdcd0e804 as u32,
                0xf1290dc7 as u32,
                0xcc00ffa3 as u32,
                0xb5390f92 as u32,
                0x690fed0b as i32 as uint32_t,
                0x667b9ffb as i32 as uint32_t,
                0xcedb7d9c as u32,
                0xa091cf0b as u32,
                0xd9155ea3 as u32,
                0xbb132f88 as u32,
                0x515bad24 as i32 as uint32_t,
                0x7b9479bf as i32 as uint32_t,
                0x763bd6eb as i32 as uint32_t,
                0x37392eb3 as i32 as uint32_t,
                0xcc115979 as u32,
                0x8026e297 as u32,
                0xf42e312d as u32,
                0x6842ada7 as i32 as uint32_t,
                0xc66a2b3b as u32,
                0x12754ccc as i32 as uint32_t,
                0x782ef11c as i32 as uint32_t,
                0x6a124237 as i32 as uint32_t,
                0xb79251e7 as u32,
                0x6a1bbe6 as i32 as uint32_t,
                0x4bfb6350 as i32 as uint32_t,
                0x1a6b1018 as i32 as uint32_t,
                0x11caedfa as i32 as uint32_t,
                0x3d25bdd8 as i32 as uint32_t,
                0xe2e1c3c9 as u32,
                0x44421659 as i32 as uint32_t,
                0xa121386 as i32 as uint32_t,
                0xd90cec6e as u32,
                0xd5abea2a as u32,
                0x64af674e as i32 as uint32_t,
                0xda86a85f as u32,
                0xbebfe988 as u32,
                0x64e4c3fe as i32 as uint32_t,
                0x9dbc8057 as u32,
                0xf0f7c086 as u32,
                0x60787bf8 as i32 as uint32_t,
                0x6003604d as i32 as uint32_t,
                0xd1fd8346 as u32,
                0xf6381fb0 as u32,
                0x7745ae04 as i32 as uint32_t,
                0xd736fccc as u32,
                0x83426b33 as u32,
                0xf01eab71 as u32,
                0xb0804187 as u32,
                0x3c005e5f as i32 as uint32_t,
                0x77a057be as i32 as uint32_t,
                0xbde8ae24 as u32,
                0x55464299 as i32 as uint32_t,
                0xbf582e61 as u32,
                0x4e58f48f as i32 as uint32_t,
                0xf2ddfda2 as u32,
                0xf474ef38 as u32,
                0x8789bdc2 as u32,
                0x5366f9c3 as i32 as uint32_t,
                0xc8b38e74 as u32,
                0xb475f255 as u32,
                0x46fcd9b9 as i32 as uint32_t,
                0x7aeb2661 as i32 as uint32_t,
                0x8b1ddf84 as u32,
                0x846a0e79 as u32,
                0x915f95e2 as u32,
                0x466e598e as i32 as uint32_t,
                0x20b45770 as i32 as uint32_t,
                0x8cd55591 as u32,
                0xc902de4c as u32,
                0xb90bace1 as u32,
                0xbb8205d0 as u32,
                0x11a86248 as i32 as uint32_t,
                0x7574a99e as i32 as uint32_t,
                0xb77f19b6 as u32,
                0xe0a9dc09 as u32,
                0x662d09a1 as i32 as uint32_t,
                0xc4324633 as u32,
                0xe85a1f02 as u32,
                0x9f0be8c as i32 as uint32_t,
                0x4a99a025 as i32 as uint32_t,
                0x1d6efe10 as i32 as uint32_t,
                0x1ab93d1d as i32 as uint32_t,
                0xba5a4df as i32 as uint32_t,
                0xa186f20f as u32,
                0x2868f169 as i32 as uint32_t,
                0xdcb7da83 as u32,
                0x573906fe as i32 as uint32_t,
                0xa1e2ce9b as u32,
                0x4fcd7f52 as i32 as uint32_t,
                0x50115e01 as i32 as uint32_t,
                0xa70683fa as u32,
                0xa002b5c4 as u32,
                0xde6d027 as i32 as uint32_t,
                0x9af88c27 as u32,
                0x773f8641 as i32 as uint32_t,
                0xc3604c06 as u32,
                0x61a806b5 as i32 as uint32_t,
                0xf0177a28 as u32,
                0xc0f586e0 as u32,
                0x6058aa as i32 as uint32_t,
                0x30dc7d62 as i32 as uint32_t,
                0x11e69ed7 as i32 as uint32_t,
                0x2338ea63 as i32 as uint32_t,
                0x53c2dd94 as i32 as uint32_t,
                0xc2c21634 as u32,
                0xbbcbee56 as u32,
                0x90bcb6de as u32,
                0xebfc7da1 as u32,
                0xce591d76 as u32,
                0x6f05e409 as i32 as uint32_t,
                0x4b7c0188 as i32 as uint32_t,
                0x39720a3d as i32 as uint32_t,
                0x7c927c24 as i32 as uint32_t,
                0x86e3725f as u32,
                0x724d9db9 as i32 as uint32_t,
                0x1ac15bb4 as i32 as uint32_t,
                0xd39eb8fc as u32,
                0xed545578 as u32,
                0x8fca5b5 as i32 as uint32_t,
                0xd83d7cd3 as u32,
                0x4dad0fc4 as i32 as uint32_t,
                0x1e50ef5e as i32 as uint32_t,
                0xb161e6f8 as u32,
                0xa28514d9 as u32,
                0x6c51133c as i32 as uint32_t,
                0x6fd5c7e7 as i32 as uint32_t,
                0x56e14ec4 as i32 as uint32_t,
                0x362abfce as i32 as uint32_t,
                0xddc6c837 as u32,
                0xd79a3234 as u32,
                0x92638212 as u32,
                0x670efa8e as i32 as uint32_t,
                0x406000e0 as i32 as uint32_t,
            ],
            [
                0x3a39ce37 as i32 as uint32_t,
                0xd3faf5cf as u32,
                0xabc27737 as u32,
                0x5ac52d1b as i32 as uint32_t,
                0x5cb0679e as i32 as uint32_t,
                0x4fa33742 as i32 as uint32_t,
                0xd3822740 as u32,
                0x99bc9bbe as u32,
                0xd5118e9d as u32,
                0xbf0f7315 as u32,
                0xd62d1c7e as u32,
                0xc700c47b as u32,
                0xb78c1b6b as u32,
                0x21a19045 as i32 as uint32_t,
                0xb26eb1be as u32,
                0x6a366eb4 as i32 as uint32_t,
                0x5748ab2f as i32 as uint32_t,
                0xbc946e79 as u32,
                0xc6a376d2 as u32,
                0x6549c2c8 as i32 as uint32_t,
                0x530ff8ee as i32 as uint32_t,
                0x468dde7d as i32 as uint32_t,
                0xd5730a1d as u32,
                0x4cd04dc6 as i32 as uint32_t,
                0x2939bbdb as i32 as uint32_t,
                0xa9ba4650 as u32,
                0xac9526e8 as u32,
                0xbe5ee304 as u32,
                0xa1fad5f0 as u32,
                0x6a2d519a as i32 as uint32_t,
                0x63ef8ce2 as i32 as uint32_t,
                0x9a86ee22 as u32,
                0xc089c2b8 as u32,
                0x43242ef6 as i32 as uint32_t,
                0xa51e03aa as u32,
                0x9cf2d0a4 as u32,
                0x83c061ba as u32,
                0x9be96a4d as u32,
                0x8fe51550 as u32,
                0xba645bd6 as u32,
                0x2826a2f9 as i32 as uint32_t,
                0xa73a3ae1 as u32,
                0x4ba99586 as i32 as uint32_t,
                0xef5562e9 as u32,
                0xc72fefd3 as u32,
                0xf752f7da as u32,
                0x3f046f69 as i32 as uint32_t,
                0x77fa0a59 as i32 as uint32_t,
                0x80e4a915 as u32,
                0x87b08601 as u32,
                0x9b09e6ad as u32,
                0x3b3ee593 as i32 as uint32_t,
                0xe990fd5a as u32,
                0x9e34d797 as u32,
                0x2cf0b7d9 as i32 as uint32_t,
                0x22b8b51 as i32 as uint32_t,
                0x96d5ac3a as u32,
                0x17da67d as i32 as uint32_t,
                0xd1cf3ed6 as u32,
                0x7c7d2d28 as i32 as uint32_t,
                0x1f9f25cf as i32 as uint32_t,
                0xadf2b89b as u32,
                0x5ad6b472 as i32 as uint32_t,
                0x5a88f54c as i32 as uint32_t,
                0xe029ac71 as u32,
                0xe019a5e6 as u32,
                0x47b0acfd as i32 as uint32_t,
                0xed93fa9b as u32,
                0xe8d3c48d as u32,
                0x283b57cc as i32 as uint32_t,
                0xf8d56629 as u32,
                0x79132e28 as i32 as uint32_t,
                0x785f0191 as i32 as uint32_t,
                0xed756055 as u32,
                0xf7960e44 as u32,
                0xe3d35e8c as u32,
                0x15056dd4 as i32 as uint32_t,
                0x88f46dba as u32,
                0x3a16125 as i32 as uint32_t,
                0x564f0bd as i32 as uint32_t,
                0xc3eb9e15 as u32,
                0x3c9057a2 as i32 as uint32_t,
                0x97271aec as u32,
                0xa93a072a as u32,
                0x1b3f6d9b as i32 as uint32_t,
                0x1e6321f5 as i32 as uint32_t,
                0xf59c66fb as u32,
                0x26dcf319 as i32 as uint32_t,
                0x7533d928 as i32 as uint32_t,
                0xb155fdf5 as u32,
                0x3563482 as i32 as uint32_t,
                0x8aba3cbb as u32,
                0x28517711 as i32 as uint32_t,
                0xc20ad9f8 as u32,
                0xabcc5167 as u32,
                0xccad925f as u32,
                0x4de81751 as i32 as uint32_t,
                0x3830dc8e as i32 as uint32_t,
                0x379d5862 as i32 as uint32_t,
                0x9320f991 as u32,
                0xea7a90c2 as u32,
                0xfb3e7bce as u32,
                0x5121ce64 as i32 as uint32_t,
                0x774fbe32 as i32 as uint32_t,
                0xa8b6e37e as u32,
                0xc3293d46 as u32,
                0x48de5369 as i32 as uint32_t,
                0x6413e680 as i32 as uint32_t,
                0xa2ae0810 as u32,
                0xdd6db224 as u32,
                0x69852dfd as i32 as uint32_t,
                0x9072166 as i32 as uint32_t,
                0xb39a460a as u32,
                0x6445c0dd as i32 as uint32_t,
                0x586cdecf as i32 as uint32_t,
                0x1c20c8ae as i32 as uint32_t,
                0x5bbef7dd as i32 as uint32_t,
                0x1b588d40 as i32 as uint32_t,
                0xccd2017f as u32,
                0x6bb4e3bb as i32 as uint32_t,
                0xdda26a7e as u32,
                0x3a59ff45 as i32 as uint32_t,
                0x3e350a44 as i32 as uint32_t,
                0xbcb4cdd5 as u32,
                0x72eacea8 as i32 as uint32_t,
                0xfa6484bb as u32,
                0x8d6612ae as u32,
                0xbf3c6f47 as u32,
                0xd29be463 as u32,
                0x542f5d9e as i32 as uint32_t,
                0xaec2771b as u32,
                0xf64e6370 as u32,
                0x740e0d8d as i32 as uint32_t,
                0xe75b1357 as u32,
                0xf8721671 as u32,
                0xaf537d5d as u32,
                0x4040cb08 as i32 as uint32_t,
                0x4eb4e2cc as i32 as uint32_t,
                0x34d2466a as i32 as uint32_t,
                0x115af84 as i32 as uint32_t,
                0xe1b00428 as u32,
                0x95983a1d as u32,
                0x6b89fb4 as i32 as uint32_t,
                0xce6ea048 as u32,
                0x6f3f3b82 as i32 as uint32_t,
                0x3520ab82 as i32 as uint32_t,
                0x11a1d4b as i32 as uint32_t,
                0x277227f8 as i32 as uint32_t,
                0x611560b1 as i32 as uint32_t,
                0xe7933fdc as u32,
                0xbb3a792b as u32,
                0x344525bd as i32 as uint32_t,
                0xa08839e1 as u32,
                0x51ce794b as i32 as uint32_t,
                0x2f32c9b7 as i32 as uint32_t,
                0xa01fbac9 as u32,
                0xe01cc87e as u32,
                0xbcc7d1f6 as u32,
                0xcf0111c3 as u32,
                0xa1e8aac7 as u32,
                0x1a908749 as i32 as uint32_t,
                0xd44fbd9a as u32,
                0xd0dadecb as u32,
                0xd50ada38 as u32,
                0x339c32a as i32 as uint32_t,
                0xc6913667 as u32,
                0x8df9317c as u32,
                0xe0b12b4f as u32,
                0xf79e59b7 as u32,
                0x43f5bb3a as i32 as uint32_t,
                0xf2d519ff as u32,
                0x27d9459c as i32 as uint32_t,
                0xbf97222c as u32,
                0x15e6fc2a as i32 as uint32_t,
                0xf91fc71 as i32 as uint32_t,
                0x9b941525 as u32,
                0xfae59361 as u32,
                0xceb69ceb as u32,
                0xc2a86459 as u32,
                0x12baa8d1 as i32 as uint32_t,
                0xb6c1075e as u32,
                0xe3056a0c as u32,
                0x10d25065 as i32 as uint32_t,
                0xcb03a442 as u32,
                0xe0ec6e0e as u32,
                0x1698db3b as i32 as uint32_t,
                0x4c98a0be as i32 as uint32_t,
                0x3278e964 as i32 as uint32_t,
                0x9f1f9532 as u32,
                0xe0d392df as u32,
                0xd3a0342b as u32,
                0x8971f21e as u32,
                0x1b0a7441 as i32 as uint32_t,
                0x4ba3348c as i32 as uint32_t,
                0xc5be7120 as u32,
                0xc37632d8 as u32,
                0xdf359f8d as u32,
                0x9b992f2e as u32,
                0xe60b6f47 as u32,
                0xfe3f11d as i32 as uint32_t,
                0xe54cda54 as u32,
                0x1edad891 as i32 as uint32_t,
                0xce6279cf as u32,
                0xcd3e7e6f as u32,
                0x1618b166 as i32 as uint32_t,
                0xfd2c1d05 as u32,
                0x848fd2c5 as u32,
                0xf6fb2299 as u32,
                0xf523f357 as u32,
                0xa6327623 as u32,
                0x93a83531 as u32,
                0x56cccd02 as i32 as uint32_t,
                0xacf08162 as u32,
                0x5a75ebb5 as i32 as uint32_t,
                0x6e163697 as i32 as uint32_t,
                0x88d273cc as u32,
                0xde966292 as u32,
                0x81b949d0 as u32,
                0x4c50901b as i32 as uint32_t,
                0x71c65614 as i32 as uint32_t,
                0xe6c6c7bd as u32,
                0x327a140a as i32 as uint32_t,
                0x45e1d006 as i32 as uint32_t,
                0xc3f27b9a as u32,
                0xc9aa53fd as u32,
                0x62a80f00 as i32 as uint32_t,
                0xbb25bfe2 as u32,
                0x35bdd2f6 as i32 as uint32_t,
                0x71126905 as i32 as uint32_t,
                0xb2040222 as u32,
                0xb6cbcf7c as u32,
                0xcd769c2b as u32,
                0x53113ec0 as i32 as uint32_t,
                0x1640e3d3 as i32 as uint32_t,
                0x38abbd60 as i32 as uint32_t,
                0x2547adf0 as i32 as uint32_t,
                0xba38209c as u32,
                0xf746ce76 as u32,
                0x77afa1c5 as i32 as uint32_t,
                0x20756060 as i32 as uint32_t,
                0x85cbfe4e as u32,
                0x8ae88dd8 as u32,
                0x7aaaf9b0 as i32 as uint32_t,
                0x4cf9aa7e as i32 as uint32_t,
                0x1948c25c as i32 as uint32_t,
                0x2fb8a8c as i32 as uint32_t,
                0x1c36ae4 as i32 as uint32_t,
                0xd6ebe1f9 as u32,
                0x90d4f869 as u32,
                0xa65cdea0 as u32,
                0x3f09252d as i32 as uint32_t,
                0xc208e69f as u32,
                0xb74e6132 as u32,
                0xce77e25b as u32,
                0x578fdfe3 as i32 as uint32_t,
                0x3ac372e6 as i32 as uint32_t,
            ],
        ],
        p: [
            0x243f6a88 as i32 as uint32_t,
            0x85a308d3 as u32,
            0x13198a2e as i32 as uint32_t,
            0x3707344 as i32 as uint32_t,
            0xa4093822 as u32,
            0x299f31d0 as i32 as uint32_t,
            0x82efa98 as i32 as uint32_t,
            0xec4e6c89 as u32,
            0x452821e6 as i32 as uint32_t,
            0x38d01377 as i32 as uint32_t,
            0xbe5466cf as u32,
            0x34e90c6c as i32 as uint32_t,
            0xc0ac29b7 as u32,
            0xc97c50dd as u32,
            0x3f84d5b5 as i32 as uint32_t,
            0xb5470917 as u32,
            0x9216d5d9 as u32,
            0x8979fb1b as u32,
        ],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _nettle_blowfish_encround(
    mut ctx: *const blowfish_ctx,
    mut ret_xl: *mut uint32_t,
    mut ret_xr: *mut uint32_t,
) {
    let mut xl: uint32_t = 0;
    let mut xr: uint32_t = 0;
    xl = *ret_xl;
    xr = *ret_xr;
    xl ^= (*ctx).p[0 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[1 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[2 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[3 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[4 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[5 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[6 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[7 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[8 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[9 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[10 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[11 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[12 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[13 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[14 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[15 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[16 as i32 as usize];
    xr ^= (*ctx).p[(16 as i32 + 1 as i32) as usize];
    *ret_xl = xr;
    *ret_xr = xl;
}
unsafe extern "C" fn decround(
    mut ctx: *const blowfish_ctx,
    mut ret_xl: *mut uint32_t,
    mut ret_xr: *mut uint32_t,
) {
    let mut xl: uint32_t = 0;
    let mut xr: uint32_t = 0;
    xl = *ret_xl;
    xr = *ret_xr;
    xl ^= (*ctx).p[17 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[16 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[15 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[14 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[13 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[12 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[11 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[10 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[9 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[8 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[7 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[6 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[5 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[4 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[3 as i32 as usize];
    xr
        ^= (((*ctx)
            .s[0 as i32 as usize][(xl >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xl >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xl >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xl & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xr ^= (*ctx).p[2 as i32 as usize];
    xl
        ^= (((*ctx)
            .s[0 as i32 as usize][(xr >> 24 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx)
                    .s[1 as i32
                    as usize][(xr >> 16 as i32 & 0xff as i32 as u32) as usize],
            )
            ^ (*ctx)
                .s[2 as i32 as usize][(xr >> 8 as i32 & 0xff as i32 as u32) as usize])
            .wrapping_add(
                (*ctx).s[3 as i32 as usize][(xr & 0xff as i32 as u32) as usize],
            ) & 0xffffffff as u32;
    xl ^= (*ctx).p[1 as i32 as usize];
    xr ^= (*ctx).p[0 as i32 as usize];
    *ret_xl = xr;
    *ret_xr = xl;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_blowfish_encrypt(
    mut ctx: *const blowfish_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const i8,
            b"blowfish.c\0" as *const u8 as *const i8,
            337 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[i8; 94],
            >(
                b"void nettle_blowfish_encrypt(const struct blowfish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3953: {
        if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const i8,
                b"blowfish.c\0" as *const u8 as *const i8,
                337 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 94],
                    &[i8; 94],
                >(
                    b"void nettle_blowfish_encrypt(const struct blowfish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut d1: uint32_t = 0;
        let mut d2: uint32_t = 0;
        d1 = (*src.offset(0 as i32 as isize) as uint32_t) << 24 as i32
            | (*src.offset(1 as i32 as isize) as uint32_t) << 16 as i32
            | (*src.offset(2 as i32 as isize) as uint32_t) << 8 as i32
            | *src.offset(3 as i32 as isize) as uint32_t;
        d2 = (*src.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
            << 24 as i32
            | (*src.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
                << 16 as i32
            | (*src.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
                << 8 as i32
            | *src.offset(4 as i32 as isize).offset(3 as i32 as isize) as uint32_t;
        _nettle_blowfish_encround(ctx, &mut d1, &mut d2);
        *dst.offset(0 as i32 as isize) = (d1 >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(1 as i32 as isize) = (d1 >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(2 as i32 as isize) = (d1 >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(3 as i32 as isize) = (d1 & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize) = (d2 >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(5 as i32 as isize) = (d2 >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(6 as i32 as isize) = (d2 >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(7 as i32 as isize) = (d2 & 0xff as i32 as u32) as uint8_t;
        length = (length as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
        dst = dst.offset(8 as i32 as isize);
        src = src.offset(8 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_blowfish_decrypt(
    mut ctx: *const blowfish_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const i8,
            b"blowfish.c\0" as *const u8 as *const i8,
            359 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[i8; 94],
            >(
                b"void nettle_blowfish_decrypt(const struct blowfish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5762: {
        if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const i8,
                b"blowfish.c\0" as *const u8 as *const i8,
                359 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 94],
                    &[i8; 94],
                >(
                    b"void nettle_blowfish_decrypt(const struct blowfish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut d1: uint32_t = 0;
        let mut d2: uint32_t = 0;
        d1 = (*src.offset(0 as i32 as isize) as uint32_t) << 24 as i32
            | (*src.offset(1 as i32 as isize) as uint32_t) << 16 as i32
            | (*src.offset(2 as i32 as isize) as uint32_t) << 8 as i32
            | *src.offset(3 as i32 as isize) as uint32_t;
        d2 = (*src.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
            << 24 as i32
            | (*src.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
                << 16 as i32
            | (*src.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
                << 8 as i32
            | *src.offset(4 as i32 as isize).offset(3 as i32 as isize) as uint32_t;
        decround(ctx, &mut d1, &mut d2);
        *dst.offset(0 as i32 as isize) = (d1 >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(1 as i32 as isize) = (d1 >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(2 as i32 as isize) = (d1 >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(3 as i32 as isize) = (d1 & 0xff as i32 as u32) as uint8_t;
        *dst.offset(4 as i32 as isize) = (d2 >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(5 as i32 as isize) = (d2 >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(6 as i32 as isize) = (d2 >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(7 as i32 as isize) = (d2 & 0xff as i32 as u32) as uint8_t;
        length = (length as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
        dst = dst.offset(8 as i32 as isize);
        src = src.offset(8 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_blowfish_set_key(
    mut ctx: *mut blowfish_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut data: uint32_t = 0;
    let mut datal: uint32_t = 0;
    let mut datar: uint32_t = 0;
    *ctx = _nettle_blowfish_initial_ctx;
    j = 0 as i32;
    i = j;
    while i < 16 as i32 + 2 as i32 {
        data = (*key.offset(j as isize) as uint32_t) << 24 as i32
            | (*key.offset(((j + 1 as i32) as u64).wrapping_rem(length) as isize)
                as uint32_t) << 16 as i32
            | (*key.offset(((j + 2 as i32) as u64).wrapping_rem(length) as isize)
                as uint32_t) << 8 as i32
            | *key.offset(((j + 3 as i32) as u64).wrapping_rem(length) as isize)
                as uint32_t;
        (*ctx).p[i as usize] ^= data;
        j = ((j + 4 as i32) as u64).wrapping_rem(length) as i32;
        i += 1;
        i;
    }
    datar = 0 as i32 as uint32_t;
    datal = datar;
    i = 0 as i32;
    while i < 16 as i32 + 2 as i32 {
        _nettle_blowfish_encround(ctx, &mut datal, &mut datar);
        (*ctx).p[i as usize] = datal;
        (*ctx).p[(i + 1 as i32) as usize] = datar;
        i += 2 as i32;
    }
    j = 0 as i32;
    while j < 4 as i32 {
        i = 0 as i32;
        while i < 256 as i32 {
            _nettle_blowfish_encround(ctx, &mut datal, &mut datar);
            (*ctx).s[j as usize][i as usize] = datal;
            (*ctx).s[j as usize][(i + 1 as i32) as usize] = datar;
            i += 2 as i32;
        }
        j += 1;
        j;
    }
    i = 0 as i32;
    while i < 255 as i32 {
        j = i + 1 as i32;
        while j < 256 as i32 {
            if (*ctx).s[0 as i32 as usize][i as usize]
                == (*ctx).s[0 as i32 as usize][j as usize]
                || (*ctx).s[1 as i32 as usize][i as usize]
                    == (*ctx).s[1 as i32 as usize][j as usize]
                || (*ctx).s[2 as i32 as usize][i as usize]
                    == (*ctx).s[2 as i32 as usize][j as usize]
                || (*ctx).s[3 as i32 as usize][i as usize]
                    == (*ctx).s[3 as i32 as usize][j as usize]
            {
                return 0 as i32;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_blowfish128_set_key(
    mut ctx: *mut blowfish_ctx,
    mut key: *const uint8_t,
) -> i32 {
    return nettle_blowfish_set_key(ctx, 16 as i32 as size_t, key);
}