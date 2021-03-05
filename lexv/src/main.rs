use std::env;
use std::fs;


const BREAK_CHAR: char = '$';


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut alphabet: Vec<char> = Vec::new();
    alphabet.push(BREAK_CHAR);
    let mut n: usize = 0;
    for (i, p) in contents.lines().enumerate() {
        match i {
            0 => alphabet.extend::<Vec<char>>(p.replace(" ", "").chars().collect()),
            1 => n = p.parse().unwrap(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let mut strs: Vec<String> = Vec::new();
    strs.extend(lexv_rec("".to_string(), n, &alphabet));
    for s in strs {
        println!("{}", s);
    }
}

fn lexv_rec(prefix: String, k: usize, alphabet: &Vec<char>) -> Vec<String> {
    let mut strs: Vec<String> = Vec::new();
    for c in alphabet {
        if c == &BREAK_CHAR {
            if prefix.len() > 0 {
                strs.push(prefix.to_string());
            }
        } else {
            let s: String = format!("{}{}", prefix, c);
            if s.len() >= k {
                strs.push(s);
            } else {
                strs.extend(lexv_rec(s.to_string(), k, alphabet));
            }
        }
    }
    strs
}