use sha1::{Sha1, Digest};
use hex_literal::hex;

fn test_main() {
    /* Hashes 10 000 000 x 30 000 bytes > 64 * 2^32. This overflows the
       low word of the block counter. This test vector is not cross
       checked with any other sha1 implementation. */
    let expected_hash = hex!("0ba79364dc64648f2074fb4bc5c28bcfb7a787b0");
    
    let mut hasher = Sha1::new();
    let data = vec![b'a'; 30000];
    
    for _ in 0..10_000_000 {
        hasher.update(&data);
    }
    
    let result = hasher.finalize();
    assert_eq!(result[..], expected_hash[..]);
}