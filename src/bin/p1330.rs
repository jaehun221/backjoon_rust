use std::{cmp::Ordering, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums:Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    match nums[0].cmp(&nums[1]) {
        Ordering::Greater => println!(">"),
        Ordering::Less => println!("<"),
        Ordering::Equal => println!("=="),
    }

}