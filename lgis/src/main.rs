use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let _n: u16 = lines.next().unwrap().parse().unwrap();
    let permutation_line = lines.next().unwrap();
    let mut permutation: Vec<u16> = permutation_line.split_whitespace().collect::<Vec<&str>>().iter().map(|n| n.parse().unwrap()).collect();
    let longest_dec = get_inc_seq_dp(permutation.clone());
    permutation.reverse();
    let mut longest_inc = get_inc_seq_dp(permutation);
    longest_inc.reverse();
    for (i, c) in longest_inc.iter().enumerate() {
        if i == longest_inc.len() -1 {
            println!("{}", c);
        } else {
            print!("{} ", c);
        }
    };
    for (i, c) in longest_dec.iter().enumerate() {
        if i == longest_dec.len() -1 {
            println!("{}", c);
        } else {
            print!("{} ", c);
        }
    }

}



fn get_inc_seq_dp(permutation: Vec<u16>) -> Vec<u16> {
    let mut l: Vec<Vec<u16>> = Vec::new();
    for i in 0..permutation.len() {
        let mut new_vec = Vec::new();
        if i == 0 {
            new_vec.push(permutation[0].clone());
        }
        l.push(new_vec);
    }
    for (i, val) in permutation.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let mut j = 0usize;
        let mut max = 0usize;
        let mut idx = -1i64;
        while j < i {
            let arr_j = &l[j];
            if (val < arr_j.last().unwrap()) && (arr_j.len() > max) {
                max = arr_j.len();
                idx = j as i64;
            }
            j += 1;
        }
        let mut new_vec = Vec::new();
        if idx >= 0 {
            new_vec.extend(l[idx as usize].iter().copied());
        }
        new_vec.push(val.clone());
        l[i] = new_vec;


            
    }
    let mut max = 0usize;
    let mut idx = 0usize;
    for (i, arr) in l.iter().enumerate() {
        if arr.len() > max {
            max = arr.len();
            idx = i.clone();
        }
    }
    l[idx].clone()
}





fn get_inc_seq_rec(permutation: Vec<u16>) -> Vec<u16> {
    let mut longest: Vec<u16> = Vec::new();
    longest = get_inc_seq_rec_helper(permutation, longest);
    longest.reverse();
    longest
}


fn get_inc_seq_rec_helper(mut permutation: Vec<u16>, mut prev_row: Vec<u16>) -> Vec<u16> {
    let last_perm: u16 = match permutation.pop() {
        None => 0,
        Some(x) => x.clone(),
    };
    if last_perm > 0 {
        let last_in_seq: u16 = match prev_row.last() {
            None => 0,
            Some(x) => x.clone(),
        };
        if last_in_seq == 0 {
            prev_row.push(last_perm);
            prev_row = get_inc_seq_rec_helper(permutation, prev_row);
        } else if last_in_seq > last_perm {
            let mut not_add = prev_row.clone();
            not_add = get_inc_seq_rec_helper(permutation.clone(), not_add);
            prev_row.push(last_perm);
            prev_row = get_inc_seq_rec_helper(permutation, prev_row);
            if &not_add.len() > &prev_row.len() {
                prev_row = not_add.clone()
            }
        } else {
            let mut new_start: Vec<u16> = Vec::new();
            new_start = get_inc_seq_rec_helper(permutation.clone(), new_start);
            prev_row = get_inc_seq_rec_helper(permutation, prev_row);
            if &new_start.len() > &prev_row.len() {
                prev_row = new_start.clone()
            }
        }
    }
    prev_row
}