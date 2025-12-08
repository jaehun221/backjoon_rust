use std::io::{self, stdout, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();
    
    let s = stdout();
    let mut w = io::BufWriter::new(s.lock());

    for i in 1..=num {
        writeln!(w, "{i}").unwrap();
    }
}