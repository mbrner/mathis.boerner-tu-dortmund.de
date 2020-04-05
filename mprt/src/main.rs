extern crate reqwest;
extern crate regex;
use std::env;
use std::fs;
use url::Url;
use std::collections::HashMap;
use regex::Regex;
use std::collections::hash_map::Entry;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut pattern = "N{P}[ST]{P}".to_string();
    match &args.get(2) {
        Some(pat) => pattern = pat.to_string(),
        None => (),
    }
    println!("Getting positions of: {}", pattern);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let fast_a_content = get_components(&contents).await;
    let positions = find_positions(&fast_a_content, &pattern);
    for l in contents.lines() {
        if l.len() == 0 {
            continue
        }
        match positions.get(l) {
            Some(pos) => {
                println!("{}", l);
                let str_nums: Vec<String> = pos.iter() 
                    .map(|n| n.to_string())
                    .collect();
                println!("{}", str_nums.join(" "));
            },
            None => (),
        }
    }
}

fn find_positions(seqs: &HashMap<String, String>, pattern: &str) -> HashMap<String, Vec<usize>> {
    let regex_pat = pattern.replace("{", "[^").replace("}", "]");
    let mut positions: HashMap<String, Vec<usize>> = HashMap::new();
    let re = Regex::new(&regex_pat).unwrap();
    for (k, v) in seqs {
        for pos in re.find_iter(v) {
            let values: &mut Vec<usize> = match positions.entry(k.to_string()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(Vec::new())};
            values.push(pos.start()+1);
        }
    }
    positions
}


async fn get_components(content: &str) -> HashMap<String, String> {
    let mut fasta: HashMap<String, String> = HashMap::new();
    for l in content.lines() {
        if l.len() == 0 {
            continue
        }
        let mut l_string = l.to_string();
        if l_string.ends_with("\n") {
            l_string.pop();
        }
        let mut fasta_i:HashMap<String, String>  = HashMap::new();
        match &query_uniprot(l_string.to_string()).await {
            Ok(query_content) => fasta_i = parse_fasta(query_content),
            _ => println!("Error fetching {}", l_string),
        }
        if fasta_i.len() == 1 {
            match fasta_i.values().next() {
                Some(seq) => {
                    fasta.insert(l_string.to_string(), seq.to_string());
                },
                None => (),
            }
        } else {
            println!("Fount more than 1 objects in fasta.");
        }
    }
    fasta
}


async fn query_uniprot(id: String) -> Result<String, reqwest::Error> {
    let url = format!("https://www.uniprot.org/uniprot/{}.fasta", id);
    let url = Url::parse(&url).unwrap();
    let res = reqwest::get(url).await?;
    let content = res.text().await?;
    Ok(content)
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