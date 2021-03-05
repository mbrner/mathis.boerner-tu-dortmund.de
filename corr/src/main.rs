use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let seqs = parse_fasta(&contents);
    let corrections = corr(seqs);
    for (s1, s2) in corrections {
        println!("{}->{}", s1, s2);
    }
}

fn corr(seqs: HashMap<String, String>) -> Vec<(String, String)> {
    let mut lookup: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut incorrect: Vec<String> = Vec::new();
    let mut corrections: Vec<(String, String)> = Vec::new();
    for (_, seq) in &seqs {
        lookup.entry(get_rev_tuple(seq)).or_insert(Vec::new()).push(seq.to_string());
    }
    //let keys: Vec<&(String, String)> = &lookup.keys().collect();
    let keys = lookup.keys().cloned().collect::<Vec<_>>();
    for k in keys {
        if lookup[&k].len() < 2 {
            match lookup.remove(&k) {
                Some(v) => {for i in v {
                                incorrect.push(i);
                            }},
                None => panic!("this should not happen!"),
            };

        }
    }
    let keys = lookup.keys().cloned().collect::<Vec<_>>();
    for seq in &incorrect {
        let mut stop: bool = false;
        for k in &keys {
            match try_match(seq, k) {
                Some(c) => {lookup.entry(get_rev_tuple(seq)).or_insert(Vec::new()).push(seq.to_string());
                            corrections.push((seq.to_string(), c));
                            stop = true;
                        },
                None => (),
            };
            if stop {
                break
            }
        }

    }
    corrections
}

fn try_match(seq: &str, seq_key: &(String, String)) -> Option<String> {
    let (seq_key1, seq_key2) = seq_key;
    let mut dist = 0u64;
    for (c1, c2) in seq.chars().zip(seq_key1.chars()) {
        if c1 != c2 {
            dist += 1u64;
        }
    }
    if dist <= 1 {
        return Some(seq_key1.to_string());
    }
    dist = 0u64;
    for (c1, c2) in seq.chars().zip(seq_key2.chars()) {
        if c1 != c2 {
            dist += 1u64;
        }
    }
    if dist <= 1 {
        return Some(seq_key2.to_string());
    } else {
        return None;
    }
}

fn get_rev_tuple(seq: &str) -> (String, String) {
    let rv_comp: String = seq.chars().rev().map(|x| match x { 
        'A' => 'T', 
        'T' => 'A', 
        'G' => 'C', 
        'C' => 'G',
        _ => panic!("Unexpected char; {}!", x)}).collect();
    if seq > &rv_comp {
        return (rv_comp, seq.to_string());
    } else {
        return (seq.to_string(), rv_comp);
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
