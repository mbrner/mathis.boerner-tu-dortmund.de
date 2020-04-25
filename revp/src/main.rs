use std::env;
use std::fs;
use std::collections::HashMap;


const MIN_LEN: usize = 4;
const MAX_LEN: usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);

    let content = fast_a_content.values().next().unwrap().to_string();
    let result = find_revp2(&content, &MIN_LEN, &MAX_LEN);
    for (s, l) in result {
        println!("{}\t{}", s, l);
    }
}



fn find_revp2(seq: &str, min_len: &usize, max_len: &usize) -> Vec<(usize, usize)> {
    let mut list: Vec<(usize, usize)> = Vec::new();
    let char_seq:Vec<char> = seq.chars().collect();
    for i in 0..char_seq.len() {
        for l in min_len/2..max_len/2+1 {
            let mut lower_p = i+l-1;
            let mut upper_p = i+l;
            let mut is_revp = true;
            if i+l*2 > char_seq.len() {
                break;
            }
            //let s: String = char_seq[i..i+l*2].into_iter().collect();
            //print!("i={},l={}->({},{}) -> {}", i, l*2, lower_p, upper_p, s);
            while lower_p >= i && upper_p <= char_seq.len() {
                let comp = match char_seq[lower_p] {
                    'A' => 'T',
                    'T' => 'A',
                    'C' => 'G',
                    'G' => 'C',
                    _ => 'X',
                };
                if char_seq[upper_p] != comp {
                    is_revp = false;
                    break
                }
                if lower_p == 0 {
                    break
                }
                lower_p -= 1;
                upper_p += 1;
            }
            //println!(" ({})", is_revp);
            if is_revp && (l*2 >= *min_len) && (l*2 <= *max_len) && (i+(l*2) <= char_seq.len()) {
                list.push((i+1, l*2));
            }
        }
    }
    list
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

