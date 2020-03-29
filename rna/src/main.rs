use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let rna = dna_to_rna(&contents);
    println!("RNA:\n{}", rna)

}

fn dna_to_rna(dna: &String) -> String {
    let mut rna = String::from("");
    for c in dna.chars() {
        if c == 'T' {
            rna.push('U');
        } else {
            rna.push(c);
        }
    }
    rna
}