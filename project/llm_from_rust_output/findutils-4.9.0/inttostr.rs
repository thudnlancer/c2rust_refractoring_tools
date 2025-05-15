pub fn inttostr(i: i32, buf: &mut [u8]) -> &mut [u8] {
    let mut i = i;
    let mut pos = buf.len();
    
    if pos == 0 {
        return buf;
    }
    
    pos -= 1;
    buf[pos] = b'\0';
    
    if i < 0 {
        while i != 0 {
            pos -= 1;
            buf[pos] = b'0' - (i % 10) as u8;
            i /= 10;
        }
        pos -= 1;
        buf[pos] = b'-';
    } else {
        while i != 0 {
            pos -= 1;
            buf[pos] = b'0' + (i % 10) as u8;
            i /= 10;
        }
    }
    
    &mut buf[pos..]
}