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
    let mut seq_s: &String = &seq_1;
    let mut seq_t: &String = &seq_2;
    if seq_1.len() < seq_2.len() {
        seq_s = &seq_2;
        seq_t = &seq_1;
    }
    match find_motif(&seq_s, &seq_t) {
        Ok(positions) => {
            let str_vec: Vec<String> = positions.iter().map(|x| format!("{}", x)).collect();
            let joined = str_vec.join(" ");
            println!("{}", joined);
        },
        Err(msg) => println!("{}", msg),
    }
}



fn find_motif(seq_s: &String, seq_t: &String) -> Result<Vec<usize>, String>{
    let mut positions: Vec<usize> = Vec::new();
    let mut seq_t_iter = seq_t.chars();
    let mut char_t = seq_t_iter.next().unwrap().clone();
    for (i, c) in seq_s.chars().enumerate() {
        if char_t == c {
            positions.push(i + 1);
            match seq_t_iter.next() {
                Some(c) => char_t = c.clone(),
                None => break,
            }
        }
    }
    if positions.len() == seq_t.len() {
        return Ok(positions);
    } else {
        return Err("No motif found".to_string());
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