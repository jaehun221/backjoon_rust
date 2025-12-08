use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();    

    let mut word: Vec<(usize, String)> = Vec::new();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        
        let mut parts = s.split_whitespace();

        let n= parts.next().unwrap().parse::<usize>().unwrap();
        let p: String = parts.next().unwrap().to_string();

        word.push((n, p));
    }

    let mut output = String::new(); 

    for (n, s) in word {
        for ch in s.chars() {
            for _ in 0..n {
                output.push(ch);
            }
        }
        output.push('\n');
    }

    print!("{output}");
}