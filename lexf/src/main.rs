use std::env;
use std::fs;
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
    let mut lines = contents.lines();
    let alphabet_lines = lines.next().unwrap();
    let n: u8 = lines.next().unwrap().parse().unwrap();
    let mut alphabet: Vec<char> = Vec::new();
    for p in alphabet_lines.split_whitespace() {
        alphabet.push(p.chars().next().unwrap());
    }
    let words = build_words(&alphabet, n);
    let path = Path::new("result.txt");
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(&file);
    for (i, e) in words.iter().enumerate() {
        if i != words.len() - 1 {
            writeln!(&mut writer, "{}", e)?;
        } else {
            write!(&mut writer, "{}", e)?;
        }
    }
    println!("Result has been written to: {}", fs::canonicalize(&path).unwrap().to_str().unwrap());
    Ok(())
}

fn build_words(alphabet: &Vec<char>, depth: u8) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    words = build_words_rec("".to_string(), alphabet, depth, words);
    words
}

fn build_words_rec(prefix: String, alphabet: &Vec<char>, depth: u8, mut words: Vec<String>) -> Vec<String> {
    if depth > 0 {
        for c in alphabet {
            let new_word: String = format!("{}{}", prefix, c);
            words = build_words_rec(new_word, alphabet, depth-1, words);
        }
    } else {
        words.push(prefix);
    }
    words
}