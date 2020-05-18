use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let mut iter = fast_a_content.values();
    let seq_1: String = iter.next().unwrap().to_string();
    let seq_2: String = iter.next().unwrap().to_string();
    let ratio = calc_ratio(&seq_1, &seq_2);
    println!("{:.11}", ratio);
}


fn partner_acid(base_1: &char) -> char {
    let base_2 = match base_1 {
        'A' => 'G',
        'G' => 'A',
        'T' => 'C',
        'C' => 'T',
        _ => panic!("Invalid base"),
    };
    base_2
}


fn calc_ratio(seq_s: &String, seq_t: &String) -> f64{
    let mut transition_count = 0u64;
    let mut transversion_count = 0u64;
    for (base_1, base_2) in seq_s.chars().zip(seq_t.chars()) {
        let base_transition: char = partner_acid(&base_2);
        if base_1 == base_transition {
            transition_count += 1;
        } else if base_1 != base_2 {
            transversion_count += 1;
        };
    };
    transition_count as f64 / transversion_count as f64
}

fn parse_fasta(input_str: &str) -> HashMap<String, String> {
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