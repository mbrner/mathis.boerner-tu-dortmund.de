use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let substring = find_longest_substring(&fast_a_content);
    println!("{}", substring);
}

fn find_longest_substring(fast_a_content: &HashMap<String, String>) -> String {
    let base_key = find_shortest_seq(fast_a_content);
    let mut substring = "".to_string();
    let mut cache: HashMap<String, bool> = HashMap::new();
    let shortest_seq: &String = fast_a_content.get(&base_key).unwrap();
    // println!("{}", shortest_seq);
    for i in 0..shortest_seq.len() {
        let mut substring_i = "".to_string();
        for c in shortest_seq[i..].chars() {
            substring_i.push(c);
            // println!("{}", substring_i);
            let is_valid: bool = match cache.get(&substring_i) {
                Some(res) => {
                    // println!("Found value");
                    res.clone()},
                None => {
                    // println!("Check new");
                    let mut is_valid = true;
                    for (key, seq_i) in fast_a_content {
                        if key.to_string() != base_key {
                            if !seq_i.contains(&substring_i) {
                                if substring_i.len() > 0 {
                                    is_valid = false;
                                    cache.insert(substring_i.to_string(), false);
                                    substring_i = substring_i[..substring_i.len()-1].to_string();
                                }
                                break
                            } else {
                                cache.insert(substring_i.to_string(), true);
                            }
                        }
                    }
                    is_valid
                }
            };
            if !is_valid {
                break
            }
        }
        if substring_i.len() > substring.len() {
            substring = substring_i.to_string();
        }
    }
    substring
}



fn find_shortest_seq(content: &HashMap<String, String>) -> String {
    let mut key_shortest: String = "".to_string();
    let mut len_shortest = 0usize;
    for (key, seq) in content {
        if seq.len() < len_shortest || key_shortest == "" {
            len_shortest = seq.len();
            key_shortest = key.to_string();
        }
    }
    key_shortest
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
