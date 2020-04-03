use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::fmt::Write;



struct Profile {
    a: Vec<usize>,
    g: Vec<usize>,
    t: Vec<usize>,
    c: Vec<usize>,
    len: usize,
}

impl Profile {
    fn add(&mut self, slice: &str) {
        let len = slice.len();
        self.a.push(len - slice.replace("A", "").len());
        self.g.push(len - slice.replace("G", "").len());
        self.t.push(len - slice.replace("T", "").len());
        self.c.push(len - slice.replace("C", "").len());
        self.len += 1;
    }
}

impl Profile {
    fn consensus(& self) -> String{
        let mut cons: String = "".to_string();
        for i in 0..self.len {
            let max_val: usize = cmp::max(cmp::max(self.a[i], self.g[i]), cmp::max(self.t[i], self.c[i]));
            if max_val == self.a[i] {
                cons.push('A');
            } else if max_val == self.g[i] {
                cons.push('G');
            } else if max_val == self.t[i] {
                cons.push('T');
            } else if max_val == self.c[i] {
                cons.push('C');
            }
        }
        cons
    }
}

impl Default for Profile {
    fn default() -> Profile {
        Profile {
            a: Vec::new(),
            g: Vec::new(),
            t: Vec::new(),
            c: Vec::new(),
            len: 0usize,
        }
    }
}


impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for n in &self.a {
            let _ = write!(&mut out, "{} ", n);
        }
        out.pop();
        let _ = write!(f, "A: {}\n", out);
        out.clear();
        for n in &self.c {
            let _ = write!(&mut out, "{} ", n);
        }
        out.pop();
        let _ = write!(f, "C: {}\n", out);
        out.clear();
        for n in &self.g {
            let _ = write!(&mut out, "{} ", n);
        }
        out.pop();
        let _ = write!(f, "G: {}\n", out);
        out.clear();
        for n in &self.t {
            let _ = write!(&mut out, "{} ", n);
        }
        out.pop();
        write!(f, "T: {}", out)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let profile = build_profile(fast_a_content);
    println!("{}", profile.consensus());
    println!("{}", profile);
}

fn build_profile(mut seqs: HashMap<String, String>) -> Profile {
    let mut profile = Profile::default();
    let mut last_len: usize = 1;
    let mut current_col = "".to_string();
    loop {
        for val in seqs.values_mut() {
            current_col.push(val.remove(0));
            last_len = cmp::min(last_len, val.len());
        }
        profile.add(&current_col);
        current_col.clear();
        if last_len == 0 {
            break;
        }
    }
    profile
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