use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut n_nodes: u64 = 0;
    let mut n_edges: u64 = 0;
    for (i, l) in contents.lines().enumerate() {
        match i {
            0 => n_nodes = l.parse().unwrap(),
            _ => n_edges += 1,
        }
    }
    println!("{}", n_nodes - (n_edges + 1));
}