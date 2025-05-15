pub fn uinttostr(mut num: u32, buffer: &mut [u8]) -> Result<&str, &'static str> {
    let mut i = buffer.len() - 1;
    buffer[i] = b'\0';
    
    if num == 0 {
        if i == 0 {
            return Err("Buffer too small");
        }
        i -= 1;
        buffer[i] = b'0';
    } else {
        while num > 0 {
            if i == 0 {
                return Err("Buffer too small");
            }
            i -= 1;
            buffer[i] = (num % 10) as u8 + b'0';
            num /= 10;
        }
    }
    
    std::str::from_utf8(&buffer[i..]).map_err(|_| "Invalid UTF-8 sequence")
}