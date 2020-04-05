use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let weight_table_f = &args[1];
    let filename = &args[2];
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let weight_table_content = fs::read_to_string(weight_table_f)
        .expect("Something went wrong reading the file");
    let weight_table = build_weight_table(& weight_table_content);
    let result = translate_protein(&content, &weight_table);
    println!("{:.3}", result);
}

fn translate_protein(contents: &str, weight_table: &HashMap<String, f64>) -> f64 {
    let mut w = 0f64; 
    for c in contents.chars() {
        match weight_table.get(&c.to_string()) {
            Some(w_i) => w += w_i,
            None => println!("No weight found for {}", c)
        }
    }  
    w
}



fn build_weight_table(contents: &str) -> HashMap<String, f64> {
    let mut weight_table: HashMap<String, f64> = HashMap::new();
    let mut key = "".to_string();
    let mut value = String::new();
    for line in contents.lines() {
        for (i, p) in line.split_whitespace().enumerate() {
            match i % 2 {
                0 => key = p.to_string(),
                1 => value = p.to_string(),
                _ => println!("This should happen!")
            }
            if (&key != "") && (&value != "") {
                weight_table.insert(key.to_string(), value.parse::<f64>().unwrap());
                key.clear();
                value.clear();
            }
        }
    }
    weight_table
}
