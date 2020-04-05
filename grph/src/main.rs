use std::env;
use std::fs;
use std::collections::HashMap;
use std::fmt;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use std::path::Path;


struct Edge {
    start: String,
    stop: String,
}


impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.start, self.stop)
    }
}

const K: usize = 3usize;

fn main() -> std::io::Result<()> {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = parse_fasta(&contents);
    let edges = find_edges(fast_a_content);
    let path = Path::new("result.txt");
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(&file);
    for (i, e) in edges.iter().enumerate() {
        if i != edges.len() - 1 {
            writeln!(&mut writer, "{}", e)?;
        } else {
            write!(&mut writer, "{}", e)?;
        }
    }
    println!("Result has been written to: {}", fs::canonicalize(&path).unwrap().to_str().unwrap());
    Ok(())
}

fn find_edges(seqs: HashMap<String, String>) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for (k, v_s) in seqs.iter() {
        for (l, v_e) in seqs.iter() {
            if k == l {
                continue;
            } else {

                let slice_e = &v_e[v_e.len() - K..];
                if v_s.starts_with(slice_e) {
                    edges.push(Edge{start: l.to_string(), stop: k.to_string()});
                }
            }
        }
    }
    edges
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