use std::{io, usize};

fn main() {
    let nums:Vec<usize> =  {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.split_whitespace().map(|x| x.parse().unwrap()).collect()
    };

    let mut num = 0;

    for i in nums {
        num += i.pow(2);
    }

    println!("{}", num % 10);
}
