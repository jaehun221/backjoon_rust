use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();

    println!("{}", check(n) as i32);
}

fn check(num:i32) -> bool {
    num % 400 == 0 || (num % 4 == 0 && num % 100 != 0)
}