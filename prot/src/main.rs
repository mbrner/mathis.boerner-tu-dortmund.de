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
    let result = translate_protein(&content, &alphabet);
    println!("{}", result)

}

fn translate_protein(contents: &str, alphabet: &HashMap<String, String>) -> String {
    let mut string_a = contents.to_string();
    let mut string_b = contents.to_string();
    let mut string_c = contents.to_string();
    string_a.pop();
    string_a.pop();
    string_b.pop();
    string_b.remove(0);
    string_c.remove(0);
    string_c.remove(0);
    let mut translated_seq: String = "".to_string();
    let mut skip = 0;
    for _ in 0..string_a.len() {
        let key = format!("{}{}{}", string_a.remove(0), string_b.remove(0), string_c.remove(0));
        match skip {
             0 => (),
             _ => {
                 skip -= 1;
                 continue;
             }

        }
        match alphabet.get(&key) {
            Some(replacement) => {
                if replacement == "Stop" {
                     break;
                } else {
                    skip = key.len() - 1;
                    translated_seq.push_str(replacement);
                }
            }
            None => translated_seq.push(key.chars().next().unwrap()),
    }
    }
    translated_seq
}



fn build_alphabet(contents: &str) -> HashMap<String, String> {
    let mut alphabet: HashMap<String, String> = HashMap::new();
    let mut key = "".to_string();
    let mut value = "".to_string();
    for line in contents.lines() {
        for (i, p) in line.split_whitespace().enumerate() {
            match i % 2 {
                0 => key = p.to_string(),
                1 => value = p.to_string(),
                _ => println!("This should happen!")
            }
            if (&key != "") && (&value != "") {
                alphabet.insert(key.to_string(), value.to_string());
                key.clear();
                value.clear();
            }
        }
    }
    alphabet
}
