use std::env;
use std::fs;

const FACTORS: [f32; 6] = [2f32, 2f32, 2f32, 1.5f32, 1f32, 0f32];


fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut prob = 0f32;
    for (f, n) in FACTORS.iter().zip(contents.split_whitespace()) {
        let exp_i: f32 = n.parse().unwrap();
        prob +=  exp_i * f;
    }
    println!("{}", prob);
}
