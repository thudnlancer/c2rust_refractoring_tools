pub fn inttostr(i: i32) -> String {
    let mut buf = String::new();
    if i < 0 {
        buf.push('-');
        let mut num = -i;
        let mut digits = Vec::new();
        if num == 0 {
            digits.push(b'0');
        } else {
            while num > 0 {
                digits.push(b'0' + (num % 10) as u8);
                num /= 10;
            }
        }
        for &c in digits.iter().rev() {
            buf.push(c as char);
        }
    } else {
        let mut num = i;
        let mut digits = Vec::new();
        if num == 0 {
            digits.push(b'0');
        } else {
            while num > 0 {
                digits.push(b'0' + (num % 10) as u8);
                num /= 10;
            }
        }
        for &c in digits.iter().rev() {
            buf.push(c as char);
        }
    }
    buf
}