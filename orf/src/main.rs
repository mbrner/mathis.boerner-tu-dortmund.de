extern crate regex;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = env::args().collect();

    let alphabet = &args[1];
    let alphabet_content = fs::read_to_string(alphabet)
        .expect("Something went wrong reading the file");
    let alphabet = build_alphabet(& alphabet_content);

    let filename = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);

    let content = fast_a_content.values().next().unwrap().to_string();
    let orfs_frwd = find_orf(&content, &alphabet);
    let revc_content = revc(&content);
    let orfs_revc = find_orf(&revc_content, &alphabet);
    let mut orfs: HashSet<String> = HashSet::new();
    for o in &orfs_frwd {
        orfs.insert(o.to_string());
    }
    for o in &orfs_revc {
        orfs.insert(o.to_string());
    }
    for o in orfs {
        println!("{}", o);
    }
}



fn revc(dna: &String) -> String {
    let mut complement = String::from("");
    for c in dna.chars() {
        match c {
            'A' => complement.push('T'),
            'T' => complement.push('A'),
            'G' => complement.push('C'),
            'C' => complement.push('G'),
            _ => println!("Unexpected char {} occured -> ignored", c)
        }
    }
    complement.chars().rev().collect::<String>()
}


fn split_orf(orf: &String) -> Vec<String> {
    let mut orfs: Vec<String> = Vec::new();
    for (i, c) in orf.chars().enumerate() {
        if c == 'M' {
            orfs.push(orf[i..].to_string())
        }
    }
    orfs
}



fn find_orf(content: &String, alphabet: &HashMap<String, String>) -> Vec<String>{
    let mut orfs: Vec<String> = Vec::new();
    for offset in 0..3 {
        let mut i = 0usize;
        let mut in_frame: bool = false;
        let mut current_content: String = "".to_string();
        let content_offset = content[offset..].to_string();
        while i < content_offset.len() / 3 as usize {
            let start: usize = i * 3;
            let end: usize = i * 3 + 3;
            let condon = content_offset[start..end].to_string();
            let translation = alphabet.get(&condon).unwrap();
            if translation == "M" {
                if in_frame {
                    current_content.push_str(translation);
                } else {
                    in_frame = true;
                    current_content.clear();
                    current_content.push_str(translation);
                }
            } else if translation == "Stop" {
                in_frame = false;
                if current_content.len() > 0 {
                    orfs.extend(split_orf(&current_content));
                }
                current_content.clear();
            } else {
                if in_frame {
                    current_content.push_str(translation)
                }
            }
            i += 1;
        }
    }
    orfs
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
