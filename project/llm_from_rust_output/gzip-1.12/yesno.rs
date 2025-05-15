use std::io::{self, Read};

fn yesno() -> io::Result<bool> {
    let mut yes = false;
    let mut input = [0u8; 1];
    
    loop {
        if io::stdin().read_exact(&mut input).is_err() {
            break;
        }
        let c = input[0] as char;
        
        if c == '\n' {
            break;
        }
        
        if !yes {
            yes = c == 'y' || c == 'Y';
        }
    }
    
    Ok(yes)
}