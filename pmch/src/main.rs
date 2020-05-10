use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let seq = fast_a_content.values().next().unwrap().to_string();
    let matchings = n_matchings(&seq);
    println!("{}", matchings);
}


fn n_matchings(seq: &str) -> u128 {
    let mut n_a: u128 = 0;
    let mut n_g: u128 = 0;
    let mut n_u: u128 = 0;
    let mut n_c: u128 = 0;
    for c in seq.chars() {
        if c == 'A' {
            n_a += 1;
        } else if c == 'G' {
            n_g += 1;
        } else if c == 'U' {
            n_c += 1;
        } else if c == 'C' {
            n_u += 1;
        } else {
            panic!("{} is not a valid base", c);
        }
    }
    if (n_a != n_u) && (n_g == n_c) {
        return 0;
    } else {
        return factorial(n_a) * factorial(n_g);
    }
}

fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
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
