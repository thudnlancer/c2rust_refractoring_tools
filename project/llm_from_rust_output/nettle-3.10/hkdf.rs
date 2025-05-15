use std::mem;

type SizeT = usize;
type UInt8T = u8;

pub struct HkdfContext<U, D> {
    mac_ctx: Box<dyn std::any::Any>,
    update: U,
    digest: D,
    digest_size: SizeT,
}

impl<U, D> HkdfContext<U, D>
where
    U: FnMut(&mut dyn std::any::Any, SizeT, &[UInt8T]),
    D: FnMut(&mut dyn std::any::Any, SizeT, &mut [UInt8T]),
{
    pub fn new(mac_ctx: Box<dyn std::any::Any>, update: U, digest: D, digest_size: SizeT) -> Self {
        HkdfContext {
            mac_ctx,
            update,
            digest,
            digest_size,
        }
    }

    pub fn extract(&mut self, secret: &[UInt8T], dst: &mut [UInt8T]) {
        (self.update)(&mut self.mac_ctx, secret.len(), secret);
        (self.digest)(&mut self.mac_ctx, self.digest_size, dst);
    }

    pub fn expand(&mut self, info: &[UInt8T], length: SizeT, dst: &mut [UInt8T]) {
        if length == 0 {
            return;
        }

        let mut i = 1u8;
        let mut remaining = length;
        let mut offset = 0;

        loop {
            (self.update)(&mut self.mac_ctx, info.len(), info);
            (self.update)(&mut self.mac_ctx, 1, &[i]);

            if remaining <= self.digest_size {
                break;
            }

            let (current_dst, next_dst) = dst[offset..].split_at_mut(self.digest_size);
            (self.digest)(&mut self.mac_ctx, self.digest_size, current_dst);
            (self.update)(&mut self.mac_ctx, self.digest_size, current_dst);

            offset += self.digest_size;
            remaining -= self.digest_size;
            i = i.wrapping_add(1);
        }

        let final_dst = &mut dst[offset..offset + remaining];
        (self.digest)(&mut self.mac_ctx, remaining, final_dst);
    }
}