use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let mut gc: HashMap<String, f64> = HashMap::new();
    for (key, value) in &fast_a_content {
        gc.insert(key.to_string(), calc_gc(value));
    }
    let mut max_gc: f64 = 0f64;
    let mut max_key = "".to_string();
    for (key, value) in &gc {
        if value > &max_gc {
            max_gc = *value;
            max_key = key.to_string();
        }
    }
    println!("{}", max_key);
    println!("{}", max_gc);

}

fn calc_gc(seq: &str) -> f64 {
    let total = seq.len();
    let mut counter: i64 = 0;
    for c in seq.chars() {
        if c == 'G' || c == 'C' {
            counter = counter + 1;
        }
    }
    counter as f64 / total as f64 * 100f64
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
