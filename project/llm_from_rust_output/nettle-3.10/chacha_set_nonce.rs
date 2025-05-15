use std::convert::TryInto;

type Uint8 = u8;
type Uint32 = u32;

#[derive(Copy, Clone)]
pub struct ChachaCtx {
    pub state: [Uint32; 16],
}

impl ChachaCtx {
    pub fn set_nonce(&mut self, nonce: &[Uint8; 8]) {
        self.state[12] = 0;
        self.state[13] = 0;
        self.state[14] = Uint32::from_le_bytes(nonce[0..4].try_into().unwrap());
        self.state[15] = Uint32::from_le_bytes(nonce[4..8].try_into().unwrap());
    }

    pub fn set_nonce96(&mut self, nonce: &[Uint8; 12]) {
        self.state[12] = 0;
        self.state[13] = Uint32::from_le_bytes(nonce[0..4].try_into().unwrap());
        self.state[14] = Uint32::from_le_bytes(nonce[4..8].try_into().unwrap());
        self.state[15] = Uint32::from_le_bytes(nonce[8..12].try_into().unwrap());
    }

    pub fn set_counter(&mut self, counter: &[Uint8; 8]) {
        self.state[12] = Uint32::from_le_bytes(counter[0..4].try_into().unwrap());
        self.state[13] = Uint32::from_le_bytes(counter[4..8].try_into().unwrap());
    }

    pub fn set_counter32(&mut self, counter: &[Uint8; 4]) {
        self.state[12] = Uint32::from_le_bytes(counter.try_into().unwrap());
    }
}