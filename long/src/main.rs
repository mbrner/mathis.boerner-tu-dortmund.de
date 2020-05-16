use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let final_seq = merge_seqs(&fast_a_content);
    println!("{}", final_seq);
}

fn check_seq(seq1: &String, seq2: &String) -> Result<String, String> {
    let mut seq_long = seq1;
    let mut seq_short = seq2;
    if seq1.len() < seq2.len() {
        seq_long = seq2;
        seq_short = seq1;
    }
    let mut pointer = (seq_short.len() as f64 / 2f64 + 0.5f64) as usize + 1usize;
    while pointer < seq_short.len() {
        if seq_long[..pointer] == seq_short[seq_short.len() - pointer..] {
            return Ok(seq_short[..seq_short.len() - pointer].to_string() + seq_long);
        } else if seq_long[seq_long.len() - pointer..] == seq_short[..pointer] {
            return Ok(seq_long.to_string() + &seq_short[pointer..]);
        }
        pointer += 1;
    }
    return Err("No match!".to_string());
}


fn merge_seqs(seqs: &HashMap<String, String>) -> String {
    let mut final_seq: String = "".to_string();
    let mut pointer: usize = 0;
    let mut keys: Vec<String> = Vec::new();
    for k in seqs.keys() {
        keys.push(k.to_string());
    }
    while keys.len() > 0usize {
        if final_seq.len() == 0usize {
            final_seq.push_str(&seqs[&keys[0]]);
            keys.remove(pointer);
        } else {
            match check_seq(&final_seq, &seqs[&keys[pointer]]) {
                Ok(new_seq) => {
                    final_seq = new_seq;
                    keys.remove(pointer);
                },
                Err(_) => pointer += 1,
            }
        }
        if pointer >= keys.len() {
                pointer = 0;
        }
    }
    final_seq
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