pub fn glp_strspx(str: &mut String) -> &mut String {
    let mut result = String::new();
    for c in str.chars() {
        if c != ' ' {
            result.push(c);
        }
    }
    *str = result;
    str
}