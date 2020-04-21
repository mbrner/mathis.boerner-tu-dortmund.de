use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let n = contents.parse::<u32>().unwrap();
    let list = generate_permutations(&n);
    println!("{}", list.len());
    for k in list {
        let str_vec: Vec<String> = k.into_iter().map(|i| i.to_string()).collect();
        println!("{}", str_vec.join(" "));
    }
}



fn generate_permutations(n: &u32) -> Vec<Vec<u32>> {
    let v: Vec<u32> = (1u32..*n+1).map(u32::from).collect();
    let r = rec(&Vec::new(), v);
    r
}


fn rec(s: &Vec<u32>, remaining: Vec<u32>) -> Vec<Vec<u32>> {
    let mut sets: Vec<Vec<u32>> = Vec::new();
    if remaining.len() > 0 {
        for (i, num) in remaining.iter().enumerate() {
            let mut vec_i = s.clone();
            vec_i.push(num.clone());
            let mut vec_j = remaining.clone();
            vec_j.remove(i);
            let new_setss = rec(&vec_i, vec_j);
            for r in new_setss.iter() {
                sets.push(r.clone());
            }
        } 
    } else {
        sets.push(s.clone());
    }
    sets
}