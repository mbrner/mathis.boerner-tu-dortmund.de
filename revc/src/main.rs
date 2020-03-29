use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let complement = revc(&contents);
    println!("Reverse complement:\n{}", complement);
}

fn revc(dna: &String) -> String {
    let mut complement = String::from("");
    for c in dna.chars() {
        match c {
            'A' => complement.push('T'),
            'T' => complement.push('A'),
            'G' => complement.push('C'),
            'C' => complement.push('G'),
            _ => println!("Unexpected char {} occured -> ignored", c)
        }
    }
    complement.chars().rev().collect::<String>()
}