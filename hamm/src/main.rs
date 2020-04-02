use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let s = lines.next().unwrap();
    let t = lines.next().unwrap();
    println!("{}", hamm_dist(s, t));
}


fn hamm_dist(s: &str, t: &str) -> i64 {
    let mut counter: i64 = 0;
    for it in s.chars().zip(t.chars()) {
        let (s_i, t_i) = it;
        if s_i != t_i {
            counter = counter + 1;
        }
    }
    counter
}
