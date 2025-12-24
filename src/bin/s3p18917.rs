use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut seq = input.split_whitespace(); 

    let n: usize = seq.next().unwrap().parse().unwrap();
    
    let mut sum: i64 = 0;
    let mut xor: i64 = 0;

    let mut out = String::new();

    for _ in 0..n {

        let query: i32 = seq.next().unwrap().parse().unwrap();

        match query {
            1 => {
                let x: i64 = seq.next().unwrap().parse().unwrap();
                sum += x;
                xor ^= x;
            }, 
            
            2 => {
                let x: i64 = seq.next().unwrap().parse().unwrap();
                sum -= x;
                xor ^= x;
            },

            3 => {
                out.push_str(&sum.to_string());
                out.push('\n');
            },

            4 => {
                out.push_str(&xor.to_string());
                out.push('\n');
            },
            _ => {},
        
        };
    };

    print!("{out}")
}