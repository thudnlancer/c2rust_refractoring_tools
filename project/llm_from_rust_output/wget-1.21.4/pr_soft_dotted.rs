use std::sync::Once;

type Ucs4 = u32;

#[derive(Copy, Clone)]
pub struct UcProperty {
    test_fn: Option<fn(Ucs4) -> bool>,
}

struct BitmapTable {
    header: [i32; 1],
    level1: [i32; 2],
    level2: [i16; 256],
    level3: [u32; 176],
}

impl BitmapTable {
    fn lookup(&self, uc: Ucs4) -> bool {
        let index1 = (uc >> 16) as usize;
        if index1 < self.header[0] as usize {
            let lookup1 = self.level1[index1];
            if lookup1 >= 0 {
                let index2 = ((uc >> 9) & 0x7F) as usize;
                let lookup2 = self.level2[lookup1 as usize + index2];
                if lookup2 >= 0 {
                    let index3 = ((uc >> 5) & 0xF) as usize;
                    let lookup3 = self.level3[lookup2 as usize + index3];
                    return (lookup3 >> (uc & 0x1F)) & 1 != 0;
                }
            }
        }
        false
    }
}

static U_PROPERTY_SOFT_DOTTED: UcProperty = UcProperty {
    test_fn: Some(uc_is_property_soft_dotted),
};

static mut U_PROPERTY_SOFT_DOTTED_DATA: Option<BitmapTable> = None;
static INIT: Once = Once::new();

fn initialize_data() {
    INIT.call_once(|| {
        let data = BitmapTable {
            header: [2],
            level1: [
                (3 * std::mem::size_of::<i32>() / std::mem::size_of::<i16>()) as i32,
                (3 * std::mem::size_of::<i32>() / std::mem::size_of::<i16>() + 128) as i32,
            ],
            level2: [
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>()) as i16,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 16) as i16,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 32) as i16,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 48) as i16,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 64) as i16,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 80) as i16,
                -1, -1, -1, -1, -1,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 96) as i16,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 112) as i16,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 128) as i16,
                -1, -1, -1,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 144) as i16,
                3 + (256 * std::mem::size_of::<i16>() / std::mem::size_of::<u32>() + 160) as i16,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            ],
            level3: [
                0, 0, 0, 0x600, 0, 0, 0, 0, 0, 0x8000, 0, 0, 0, 0, 0, 0,
                0, 0, 0x200, 0x100, 0x20000000, 0x40000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000,
                0, 0, 0x1400000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x4,
                0x400000, 0x110, 0, 0, 0, 0x2000, 0, 0, 0, 0, 0x800, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0x20000, 0, 0, 0, 0, 0, 0, 0x300, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0x10000000, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
        unsafe {
            U_PROPERTY_SOFT_DOTTED_DATA = Some(data);
        }
    });
}

pub fn uc_is_property_soft_dotted(uc: Ucs4) -> bool {
    initialize_data();
    unsafe {
        U_PROPERTY_SOFT_DOTTED_DATA
            .as_ref()
            .unwrap()
            .lookup(uc)
    }
}

pub static UC_PROPERTY_SOFT_DOTTED: UcProperty = U_PROPERTY_SOFT_DOTTED;