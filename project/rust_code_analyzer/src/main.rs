extern crate clap;

use clap::{App, Arg};
mod cast;

fn main() {
    use std::io::{self, Read};

    let matches = App::new("rust_code_analyzer")
        .arg(
            Arg::with_name("enum_name")
                .long("enum-name")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let enum_name = matches.value_of("enum_name").unwrap();

    // 从 stdin 读取代码
    let mut code = String::new();
    io::stdin().read_to_string(&mut code).expect("Failed to read code from stdin");


    match cast::rewrite_casts(&code, enum_name) {
        Ok(transformed_code) => println!("{}", transformed_code),
        Err(err) => {
            eprintln!("Error during transformation: {}", err);
            std::process::exit(1);
        }
    }
    
}

