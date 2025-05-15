use itertools::Itertools;
use std::fmt::Write;

fn main() {
    println!("All multisets of {{0,1,2,3}} by size:");
    for size in 0..=4 {
        let elements = vec![0, 1, 2, 3];
        for multiset in elements.iter().combinations_with_replacement(size) {
            let mut output = String::from("{");
            for (i, item) in multiset.iter().enumerate() {
                if i > 0 {
                    write!(output, " ").unwrap();
                }
                write!(output, "{}", item).unwrap();
            }
            output.push('}');
            println!("{}", output);
        }
    }
}