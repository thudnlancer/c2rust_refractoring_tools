pub fn inttostr(i: i32, buf: &mut [u8]) -> &[u8] {
    let mut i = i;
    let mut pos = buf.len();
    
    if pos == 0 {
        return &[];
    }

    pos -= 1;
    buf[pos] = b'\0';
    
    if i == 0 {
        if pos == 0 {
            return &[];
        }
        pos -= 1;
        buf[pos] = b'0';
        return &buf[pos..];
    }

    let is_negative = i < 0;
    while i != 0 {
        if pos == 0 {
            return &[];
        }
        pos -= 1;
        buf[pos] = if is_negative {
            b'0' - (i % 10) as u8
        } else {
            b'0' + (i % 10) as u8
        };
        i /= 10;
    }

    if is_negative {
        if pos == 0 {
            return &[];
        }
        pos -= 1;
        buf[pos] = b'-';
    }

    &buf[pos..]
}