use std::io;

fn main() {
    let num:usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        input.trim().parse().unwrap()
    };

    for i in 1..=num {
        println!("{}", "*".repeat(i));
    }
}
