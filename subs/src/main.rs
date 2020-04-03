use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut seq = "".to_string();
    let mut part = "".to_string();
    for (i, l) in contents.lines().enumerate() {
        match i {
            0 => seq.push_str(l),
            1 => part.push_str(l),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let vec = find_motifs(&seq, &part);
    let vec_str: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
    println!("{}", vec_str.join(" "));
    //str_nums.join(sep)
}

fn find_motifs(seq: &str, part: &str) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut mut_seq = seq.to_string();
    for i in 0..seq.len() {
        let mut counterpart = mut_seq.to_string();
        counterpart.truncate(part.len());
        if counterpart == part {
            vec.push((i+1) as i32);
        }
        mut_seq.remove(0);
        if mut_seq.len() <= part.len() || mut_seq.len() == 0 {
            break;
        }
    }
    vec
}