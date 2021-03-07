use std::env;
use std::fs;
use std::collections::HashSet;

fn prepare_pair_block(s: &str) ->(Vec<u64>, Vec<u64>) {
    let ss: Vec<String> = s.split("\n").map(|x| x.to_string()).collect();
    let a: Vec<u64> = ss[0].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let b: Vec<u64> = ss[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    (a, b)
}

fn _possible_reversals(s: &Vec<u64>) -> HashSet<Vec<u64>> {
    let mut new_ss: HashSet<Vec<u64>> = HashSet::new();
    let l: usize = s.len();
    for i in 0..(l-1) {
        for j in (i+1)..(l) {
            let mut s_ij = s.clone();
            s_ij[i..j+1].reverse();
            new_ss.insert(s_ij);
        }
    }
    new_ss
}

fn rear(s_a: HashSet<Vec<u64>>, s_b: HashSet<Vec<u64>>, dist: u64) -> u64 {
    if &s_a.intersection(&s_b).collect::<HashSet<_>>().len() > &0 {
        return dist;
    }
    let mut new_s_a: HashSet<Vec<u64>> = HashSet::new();
    for s_i in &s_a {
        for s_i_new in _possible_reversals(s_i){
            new_s_a.insert(s_i_new);
        }
    }
    let mut new_s_b: HashSet<Vec<u64>> = HashSet::new();
    for s_i in &s_b {
        for s_i_new in _possible_reversals(s_i){
            new_s_b.insert(s_i_new);
        }
    }
    let new_dist: u64 = dist+2u64;
    if &s_a.intersection(&new_s_b).collect::<HashSet<_>>().len() > &0 {
        return new_dist-1;
    }
    if &new_s_a.intersection(&s_b).collect::<HashSet<_>>().len() > &0 {
        return new_dist-1;
    }
    if &new_s_a.intersection(&new_s_b).collect::<HashSet<_>>().len() > &0 {
        return new_dist;
    }
    rear(new_s_a, new_s_b, new_dist)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let pairs: Vec<&str> = content.split("\n\n").collect();
    let mut distances: Vec<u64> = Vec::new();
    for p in pairs {
        let (a, b) = prepare_pair_block(p);
        let mut s_a: HashSet<Vec<u64>> = HashSet::new();
        let mut s_b: HashSet<Vec<u64>> = HashSet::new();
        s_a.insert(a);
        s_b.insert(b);
        distances.push(rear(s_a, s_b, 0u64));
    }
    println!("{}", distances.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}