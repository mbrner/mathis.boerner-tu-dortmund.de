use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    let n_a = count_letters(&contents, 'A');
    let n_c = count_letters(&contents, 'C');
    let n_g = count_letters(&contents, 'G');
    let n_t = count_letters(&contents, 'T');
    println!("{} {} {} {}", n_a, n_c, n_g, n_t)

}

fn count_letters(text: &String, c_search: char) -> i32{
    let mut counter: i32 = 0;
    for c in text.chars() {
        if c == c_search {
            counter += 1;
        }
    }
    counter
}