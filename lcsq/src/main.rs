use std::env;
use std::fs;
use std::cmp;
use std::collections::HashMap;

#[derive(cmp::PartialEq, Clone)]
enum Traceback {
    Top,
    Left,
    Diagonal,
    Unknown,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let (seqs, order) = parse_fasta(contents);
    let longest_subsequence = lcsq(&seqs[&order[0]], &seqs[&order[1]]);
    println!("{}", longest_subsequence);
}

fn lcsq(seq1: &str, seq2: &str) -> String {
    let mut matrix: Vec<Vec<u64>> = vec!{vec!{0; seq2.len()+1}; seq2.len()+1};
    let mut traceback: Vec<Vec<Traceback>> = vec!{vec!{Traceback::Unknown; seq2.len()+1}; seq2.len()+1};
    let mut longest_subsequence: String = "".to_string();
    let mut max_score: u64 = 0;
    let (mut max_pos_x, mut max_pos_y): (usize, usize) = (0, 0);
    for i in 1..(seq1.len()+1) {
        for j in 1..(seq2.len()+1) {
            let mut score_diag = 0u64;
            if seq1.chars().nth(i-1).unwrap() == seq2.chars().nth(j-1).unwrap()  {
                score_diag = matrix[i-1][j-1] + 1;
            }
            let score_left = matrix[i-1][j];
            let score_top = matrix[i][j-1];
            matrix[i][j] = cmp::max(score_left, cmp::max(score_top, score_diag));
            if max_score < matrix[i][j] {
                max_score = matrix[i][j];
                max_pos_x = i;
                max_pos_y = j;
            }
            if score_diag == matrix[i][j] {
                traceback[i][j] = Traceback::Diagonal;
            } else if score_left == matrix[i][j] {
                traceback[i][j] = Traceback::Left;
            } else if score_top == matrix[i][j] {
                traceback[i][j] = Traceback::Top;
            } else {
                panic!("This should never happen!");
            }
        }
    }
    while max_pos_x > 0 && max_pos_y > 0 {
        if traceback[max_pos_x][max_pos_y] == Traceback::Diagonal {
            longest_subsequence.push(seq1.chars().nth(max_pos_x-1).unwrap());
            max_pos_x -= 1;
            max_pos_y -= 1;
        } else if traceback[max_pos_x][max_pos_y] == Traceback::Top {
            max_pos_y -= 1;
        } else if traceback[max_pos_x][max_pos_y] == Traceback::Left {
            max_pos_x -= 1;
        } else {
            panic!("Traceback::Unknown found! Either the matrix was build incorrect or traceback is stopping to late!");
        }
    }
    longest_subsequence.chars().rev().collect()
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
