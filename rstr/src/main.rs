use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut n: i64 = 0;
    let mut p: f64 = 0.0;
    let mut seq: String = "".to_string();
    for (i, c) in contents.split_whitespace().enumerate() {
        match i {
            0 => n = c.parse().unwrap(),
            1 => p = c.parse().unwrap(),
            2 => seq = c.to_string(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let p_x = get_prob(&seq, n, p);
    println!("{:.3}", p_x);
}

fn get_prob(seq: &str, n: i64, p: f64) -> f64 {
    let mut p_x = 1f64;
    for c in seq.chars() {
        if c  == 'C' || c == 'G' {
            p_x *= p * 0.5f64;
        } else if c == 'T' || c == 'A' {
            p_x *= (1.0f64 - p) * 0.5f64;
        } else {
            panic!("unexpected char {}", c);
        }
    }

    1f64- ((1.0f64 - p_x).powf(n as f64))
}