use std::env;
use std::fs;
use std::collections::HashMap;


#[derive(Eq, PartialEq, Hash, Clone)]
struct Key{
    n: i64,
    r: i64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut n: i64 = 0;
    let mut r: i64 = 0;
    for (i, p) in contents.split_whitespace().enumerate() {
        match i {
            0 => n = p.parse().unwrap(),
            1 => r = p.parse().unwrap(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let mut hash_map: HashMap<Key, i64> = HashMap::new();
    let key2 = Key{ n: 2, r: r};
    let key1 = Key{ n: 1, r: r};
    hash_map.insert(key2, 1);
    hash_map.insert(key1, 1);
    let result = rabbit_fib_hash(n, r, &mut hash_map);
    println!("Result (Recursion + HashMap) n={} r={}:\n{}", n, r, result);
    let result2 = rabbit_fib_it(n, r);
    println!("Result (Iterativ) n={} r={}:\n{}", n, r, result2);
}

fn rabbit_fib_hash (n: i64, r: i64,hash_map: &mut HashMap<Key, i64>) -> i64 {
    let key = Key{ n: n, r: r};
    if hash_map.contains_key(&key) {
        hash_map.get(&key).unwrap().clone()
    } else {
        let result: i64 =  rabbit_fib_hash(n - 1, r, hash_map) + r * rabbit_fib_hash(n - 2, r, hash_map);
        hash_map.insert(key.clone(), result);
        result.clone()
    }
    
}

fn rabbit_fib_it (n: i64, k: i64) -> i64 {
    let mut a: i64 = 1;
    let mut b: i64 = 1;
    for _ in 2..n {
        let a_t: i64 = b.clone();
        b = a * k + b;
        a = a_t.clone();
    }
    b
}