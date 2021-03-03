use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(contents);
    let seq = fast_a_content.values().next().unwrap().to_string();
    let matchings = build_fault_table(&seq);
    println!("{}", matchings.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + " "));
}

fn build_fault_table(seq: &str) -> Vec<usize> {
    let len = seq.chars().count();
    let mut fault_table = vec![0usize; len];
    let mut cnd = 0usize;
    let seq_bytes = seq.as_bytes();
    for pos in 1..len {
        if seq_bytes[pos] == seq_bytes[cnd] {
            cnd += 1;
            fault_table[pos] = cnd;
        } else {
            for i in 0..cnd+1 {
                if seq_bytes[pos-fault_table[pos]] == seq_bytes[cnd-i] {
                    fault_table[pos] += 1;
                } else {
                    fault_table[pos] = 0;
                }
            }
            cnd = fault_table[pos];
        }
    }
    fault_table
}

fn parse_fasta(input_str: String) -> HashMap<String, String> {
    let mut current_seq = "".to_string();
    let mut label = "".to_string();
    let mut in_seq = false;
    let mut content: HashMap<String, String> = HashMap::new();
    for l in input_str.lines() {
        if l.len() > 1 {
            let mut l_string = l.to_string();
            if l_string.ends_with("\n") {
                l_string.pop();
            }
            match l.chars().next().unwrap() {
                ';' => println!("Comment will be ignored"),
                '>' => {
                    if in_seq {
                        content.insert(label.to_string(), current_seq.to_string());
                        label.clear();
                    }
                    in_seq = true;
                    for (i, c) in l_string.chars().enumerate() {
                        if i > 0 {
                            label.push(c);
                        }
                    }
                    current_seq.clear();
                }
                _ => current_seq.push_str(&l_string),
            }
        }
    }
    if (current_seq.len() > 0) && in_seq {
        content.insert(label.to_string(), current_seq.to_string());
    }
    content
}
