use std::env;
use std::fs;
use std::fmt;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use std::path::Path;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let n = contents.parse::<u64>().unwrap();
    let list = generate_permutations(n);

    let path = Path::new("result.txt");
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(&file);

    writeln!(&mut writer, "{}", list.len());
    for k in list {
        let str_vec: Vec<String> = k.into_iter().map(|i| i.to_string()).collect();
        writeln!(&mut writer, "{}", str_vec.join(" "));
    }
    Ok(())

}



fn generate_permutations(n: u64) -> Vec<Vec<i64>> {
    let v: Vec<i64> = (1..(n+1) as i64).map(i64::from).collect();
    let mut r = rec(&Vec::new(), v);
    r = apply_signs(r, n);
    r
}


fn rec(s: &Vec<i64>, remaining: Vec<i64>) -> Vec<Vec<i64>> {
    let mut sets: Vec<Vec<i64>> = Vec::new();
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

fn apply_signs(perm: Vec<Vec<i64>>, n: u64) -> Vec<Vec<i64>> {
    let mut signs: Vec<Vec<i64>> = Vec::new();
    signs.push(vec![1]);
    signs.push(vec![-1]);
    for _i in 1..n {
        let mut new_signs: Vec<Vec<i64>> = Vec::new();
        for arr in &signs {
            let mut t_minus = arr.clone();
            t_minus.push(-1i64);
            let mut t_plus = arr.clone();
            t_plus.push(1i64);
            new_signs.push(t_plus);
            new_signs.push(t_minus);
        }
        signs = new_signs;
    }
    let mut new_perm: Vec<Vec<i64>> = Vec::new();
    for arr_a in &perm {
        for arr_s in &signs {
            let mut multiplied: Vec<i64> = Vec::new();
            for (v, s) in arr_s.iter().zip(arr_a.iter()) {
                multiplied.push(v * s);
            }
            new_perm.push(multiplied);
        }
    }
    new_perm
}