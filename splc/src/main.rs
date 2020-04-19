use std::env;
use std::fs;
use std::collections::HashMap;


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

    let spliced_rna = splice_dna(&fast_a_content, &alphabet);
    println!("{}", spliced_rna);


}


fn splice_dna(content: &HashMap<String, String>, alphabet: &HashMap<String, String>) -> String {
    let (seq, introns) = find_introns(content);
    let mut base_content = content.get(&seq).unwrap().to_string();
    for key in introns {
        base_content = base_content.replace(content.get(&key).unwrap(), "");
    }
    translate(&base_content, alphabet)
}

fn translate(seq: &String, alphabet: &HashMap<String, String>) -> String {
    let mut i = 0usize;
    let mut in_frame: bool = false;
    let mut current_content: String = "".to_string();        
    while i < seq.len() / 3 as usize {
        let start: usize = i * 3;
        let end: usize = i * 3 + 3;
        let condon = seq[start..end].to_string();
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
            break
        } else {
            if in_frame {
                current_content.push_str(translation)
            }
        }
        i += 1;
    }
    current_content
}



fn find_introns(content: &HashMap<String, String>) -> (String, Vec<String>) {
    let mut key_longest: String = "".to_string();
    let mut len_longest = 0usize;
    let mut introns: Vec<String> = Vec::new();
    for (key, seq) in content {
        if seq.len() > len_longest {
            if key_longest != "" {
                introns.push(key_longest.to_string());
            }
            key_longest = key.to_string();
            len_longest = seq.len();
        } else {
            introns.push(key.to_string());
        }
    }
    (key_longest, introns)
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
