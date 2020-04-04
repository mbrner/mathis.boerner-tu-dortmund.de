use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut n: usize = 0;
    let mut d: usize = 0;
    for (i, p) in contents.split_whitespace().enumerate() {
        match i {
            0 => n = p.parse().unwrap(),
            1 => d = p.parse().unwrap(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let mut pop: Vec<u128> = Vec::new();
    let mut month = 2usize;
    pop.push(1);
    pop.push(1);
    pop.push(1);
    while month < n {
        let f_n1 = pop.get(pop.len()-1).unwrap();
        let f_n2 = pop.get(pop.len()-2).unwrap();
        let mut new_val =  f_n1 + f_n2;
        match pop.get(((month as i32) - (d as i32)) as usize) {
            Some(k) => new_val -= *k,
            None => (),
        }
        pop.push(new_val);
        month += 1;
        //println!("{}: {:?} (died: {} - {})", month, pop, death, idx);
    }
    println!("{}", pop.get(pop.len()-1).unwrap());
}