use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut n: u128 = 0;
    let mut k: u128 = 0;
    for (i, p) in contents.split_whitespace().enumerate() {
        match i {
            0 => n = p.parse().unwrap(),
            1 => k = p.parse().unwrap(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let mut s: u128 = 1;
    let mut counter: u128 = 0;
    while (counter < k) && (n > 0){
        s *= n;
        counter += 1u128;
        n -= 1;
        println!("{} ({})", s, n);
    }
    println!("{}", s % 1000000);
}