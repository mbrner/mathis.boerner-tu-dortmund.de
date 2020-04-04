use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let alphabet = &args[1];
    let filename = &args[2];
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let alphabet_content = fs::read_to_string(alphabet)
        .expect("Something went wrong reading the file");
    let alphabet = build_alphabet(& alphabet_content);
    let n = reverse_translation(&content, &alphabet);
    println!("{}", n);
}


fn reverse_translation(seq: &str, alphabet: &HashMap<String, u128>) -> u128 {
    let mut n = 1u128;
    for c in seq.chars() {
        match alphabet.get(&c.to_string()) {
            Some(factor) => n = n * factor % 1e6 as u128,
            _ => (),
        }
    }
    match alphabet.get("Stop") {
        Some(factor) => n = n * factor % 1e6 as u128,
        None => println!("No Stop colon found!"),
    }
    n
}


fn build_alphabet(contents: &str) -> HashMap<String, u128> {
    let mut alphabet: HashMap<String, u128> = HashMap::new();
    for line in contents.lines() {
        for (i, p) in line.split_whitespace().enumerate() {
            let mut new_value = 0u128;
            let mut key = "".to_string();
            match i % 2 {
                0 => key.clear(),
                1 => {
                    key = p.to_string();
                    match alphabet.get_mut(&key) {
                        Some(value) => new_value = *value + 1,
                        None => new_value = 1,
                    };
                }
                _ => println!("This should happen!"),
            }
            if key.len() > 0 {
                alphabet.insert(key.to_string(), new_value);
                key.clear();
            }
        }
    }
    alphabet
}
