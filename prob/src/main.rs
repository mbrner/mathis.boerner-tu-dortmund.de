use std::env;
use std::fs;
use std::vec;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let seq = lines.next().unwrap();
    let probs_str = lines.next().unwrap();
    let mut probs: Vec<f64> = Vec::new();
    for p in probs_str.split_whitespace() {
        probs.push(p.parse().unwrap());
    }
    let prob_rnd = calc_rnd_probs(&seq, &probs);
    for (i, p) in prob_rnd.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{:.3}", p);
    }
    println!("");

}

fn calc_rnd_probs(seq: &str, probs: &Vec<f64>) -> Vec<f64> {
    let mut prob_rnd: Vec<f64> = Vec::new();
    for p in probs {
        let mut prob: f64 = 0f64;
        for c in seq.chars() {
            if (c == 'G') || (c == 'C') {
                prob += (p / 2f64).log10();
            } else {
                prob += ((1f64-p) / 2f64).log10();
            }
            
        }
        prob_rnd.push(prob);
    }
    prob_rnd
}