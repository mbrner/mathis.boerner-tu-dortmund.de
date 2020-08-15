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
    let matchings = n_noncrossing_matchings(&seq);
    println!("{}", matchings % 1000000);
}


fn n_noncrossing_matchings(seq: &str) -> u128 {
    let mut lookup: HashMap<String, u128> = HashMap::new();
    catalan(seq, &mut lookup)
}


fn perfect_matching_possible(seq: &str) -> bool {
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
            n_u += 1;
        } else if c == 'C' {
            n_c += 1;
        } else {
            panic!("{} is not a valid base", c);
        }
    }
    (n_a == n_u) && (n_g == n_c)
}



fn catalan(seq: &str, lookup: &mut HashMap<String, u128>) -> u128{
    let mut n = 0u128;
    if lookup.contains_key(seq) {
        n = lookup.get(seq).unwrap().clone()
    } else {
        if perfect_matching_possible(seq) {
            let n_chars = seq.chars().count();
            if n_chars == 2 {
                n = 1;
            } else if n_chars == 0 {
                n = 1;
            } else {
                for i in (1..=n_chars).step_by(2) {
                    let catalan_a = catalan(&seq[1..i], lookup);
                    let catalan_b = catalan(&seq[i+1..], lookup);
                    let mut split_pair = (seq.as_bytes()[0] as char).to_string();
                    split_pair.push(seq.as_bytes()[i] as char);
                    let catalan_split = catalan(&split_pair, lookup);
                    n += catalan_a * catalan_split * catalan_b;
                }
            }
        }
        lookup.insert(seq.to_string(), n);
    }
    n
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
