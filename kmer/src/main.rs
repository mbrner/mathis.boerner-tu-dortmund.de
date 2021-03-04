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
    let matchings = kmer(&seq, 4);
    println!("{}", matchings.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + " "));
}

fn kmer(seq: &str, k: u64) -> Vec<u64> {
    let mut kmers: Vec<String> = vec!{"".to_string(); 4usize.pow(k as u32)};
    for i in 0..k {
        let inner_n_times = 4u64.pow((k-i-1u64) as u32);
        for j in 0..kmers.len() {
            let l = (j as u64 / inner_n_times) % 4;
            kmers[j].push_str(match l {
                0 => "A",
                1 => "C",
                2 => "G",
                3 => "T",
                _ => panic!("Unexpected value: {}!", j)
            });
        }

    }
    let mut counts = vec!{0u64; kmers.len()};
    let mut lookup: HashMap<String, u64> = HashMap::new();
    for (i, kmer) in kmers.iter().enumerate() {
        lookup.insert(kmer.to_string(), i as u64);
    }
    for i in 0..(seq.len()-(k as usize)+1usize) {
        match lookup.get(&seq[i..i+(k as usize)].to_string()) {
            Some(c) => counts[(*c) as usize] += 1,
            None => panic!("Unexpected key {}", k)
        };
    }
    counts
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
