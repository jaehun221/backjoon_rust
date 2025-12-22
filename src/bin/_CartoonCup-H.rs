use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut token: Vec<i8> = input
        .split_whitespace()
        .filter(|&x| x != "+")
        .map(|x| if x == "!" {9} else {x.parse().unwrap()})
        .collect();

    for _ in 0..n {
        let k = (token.remove(0) + 1) as usize;
        let v: Vec<i8> = token.drain(..k).collect();
        println!("{}", check(v))
    }
}

fn check(v: Vec<i8>) -> String {
    let n:i8 = v.iter().sum();
    if n > 9 {
        "!".to_string()
    } else {
        
        n.to_string()
    }
}