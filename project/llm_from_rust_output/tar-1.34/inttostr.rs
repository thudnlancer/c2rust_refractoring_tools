pub fn inttostr(i: i32) -> String {
    let mut s = String::new();
    if i < 0 {
        s.push('-');
        s.extend(i.abs().to_string().chars());
    } else {
        s = i.to_string();
    }
    s
}