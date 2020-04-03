use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut k: i64 = 0;
    let mut m: i64 = 0;
    let mut n: i64 = 0;
    for (i, p) in contents.split_whitespace().enumerate() {
        match i {
            0 => k = p.parse().unwrap(),
            1 => m = p.parse().unwrap(),
            2 => n = p.parse().unwrap(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    println!("{}", iprb(k as f64, m as f64, n as f64));

}

fn iprb(k: f64, m: f64, n: f64) -> f64 {
    let sum = (k + m + n) as f64;
    let mut p_dom = k as f64 / sum; // 1. homo dom 2. X

    p_dom += 0.5 * m / sum; // 1. hete dom 2. X

    p_dom += 0.5 * m / sum * 0.5 * (m - 1.0) / (sum - 1.0);  // 1. hete rez 2. hete dom
    p_dom += 0.5 * m / sum * k / (sum - 1.0); // 1. hete rez 2. homo dom

    p_dom += n / sum * 0.5 * m / (sum - 1.0); // 1. homo rez 2. hete dom
    p_dom += n / sum * k / (sum - 1.0); // 1. homo rez 2. homo dom
    p_dom
}