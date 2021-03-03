use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let (seqs, order) = parse_fasta(contents);
    let matrix = get_dist_matrix(seqs, order);
    let len = matrix.len();
    for i in 0..len {
        let mut current_line = Vec::new();
        for j in 0..len {
            current_line.push(format!("{:.5}", matrix[i][j]));
        }
        println!("{}", current_line.join(" "));
    }    

}

fn get_dist_matrix(seqs: HashMap<String, String>, order: Vec<String>) -> Vec<Vec<f64>> {
    let len = order.len();
    let mut matrix = vec![vec![0.0f64; len]; len];
    for i in 0..len {
        for j in 0..(i+1) {
            let name_1 =  match order.get(i) {
                Some(name) => name,
                None => panic!("Somthing is very wrong her")
            };
            let name_2 =  match order.get(j) {
                Some(name) => name,
                None => panic!("Somthing is very wrong her")
            };
            let seq1 = match seqs.get(name_1) {
                Some(seq) => seq,
                None => panic!("Somthing is very wrong her")
            };
            let seq2 = match seqs.get(name_2) {
                Some(seq) => seq,
                None => panic!("Somthing is very wrong her")
            };
            let dist = distance(seq1, seq2);
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    matrix
}

fn distance(s1: &str, s2: &str) -> f64 {
    let mut score = 0.0f64;
    for (i, j) in s1.chars().zip(s2.chars()){
        if i != j {
            score += 1.0;
        }
    }
    score / (s1.chars().count() as f64)
}


fn parse_fasta(input_str: String) -> (HashMap<String, String>, Vec<String>) {
    let mut current_seq = "".to_string();
    let mut label = "".to_string();
    let mut in_seq = false;
    let mut content: HashMap<String, String> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
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
                        order.push(label.to_string());
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
        order.push(label.to_string());
    }
    (content, order)
}
