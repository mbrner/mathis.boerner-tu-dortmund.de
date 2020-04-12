use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::env;
use std::fs;

extern crate num;

use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::One;
use num::cast::ToPrimitive;



#[derive(Hash, Clone, Debug)]
struct Gen {
    allele_1: char,
    allele_2: char
}

impl Gen {
    fn new(a: &str) -> Gen {
        let mut a1: char = ' ';
        let mut a2: char = ' ';
        for (si, c) in a.chars().enumerate() {
            match si {
                1 => a2 = c,
                0 => a1 = c,
                _ => println!("Only 2 letters allowed!"),
            }

        }
        if a1 < a2 {
            return Gen{allele_1: a1, allele_2: a2};
        } else {
            return Gen{allele_1: a2, allele_2: a1};
        }
    }

    fn mate(&self, other: &Gen) -> Vec<Gen> {
        let mut set: HashSet<String> = HashSet::new();
        set.insert(format!("{}{}", self.allele_1, other.allele_1));
        set.insert(format!("{}{}", self.allele_1, other.allele_2));
        set.insert(format!("{}{}", self.allele_2, other.allele_1));
        set.insert(format!("{}{}", self.allele_2, other.allele_2));
        let mut offsprings: Vec<Gen> = Vec::new();
        for off in set {
            offsprings.push(Gen::new(&off));
        }
        offsprings
    }
}

impl PartialEq for Gen {
    fn eq(&self, other: &Gen) -> bool {
        self.allele_1 == other.allele_1 && self.allele_2 == other.allele_2
    }
}

impl Eq for Gen {}

#[derive(Hash, Debug, Clone)]
struct Organism {
    gen_a: Gen,
    gen_b: Gen,
}


impl PartialEq for Organism {
    fn eq(&self, other: &Organism) -> bool {
        self.gen_a == other.gen_a && self.gen_b == other.gen_b
    }
}

impl Eq for Organism {}

impl Organism {
    fn new(a: &str, b: &str) -> Organism {
        let gen_a = Gen::new(a);
        let gen_b = Gen::new(b);
        Organism{gen_a: gen_a, gen_b: gen_b}
    }
    
    fn mate(& self, other: &Organism) -> HashMap<Organism, f64> {
        let mut new_pop: HashMap<Organism, f64> = HashMap::new();
        let mut counter = 0;
        for g_a in self.gen_a.mate(&other.gen_a) {
            for g_b in self.gen_b.mate(&other.gen_b) {
                *new_pop.entry(Organism{gen_a: g_a.clone(), gen_b: g_b.clone()}).or_insert(0f64) += 1f64;
                counter += 1;
            }
        }
        for (_, v) in new_pop.iter_mut() {
            *v /= counter as f64;
        }
        new_pop

    }
}


impl fmt::Display for Organism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}-{}{}", self.gen_a.allele_1, self.gen_a.allele_2, self.gen_b.allele_1, self.gen_b.allele_2)
    }
}

fn mate(pop: HashMap<Organism, f64>, player: Organism) -> HashMap<Organism, f64> {
    let mut new_pop: HashMap<Organism, f64> = HashMap::new();
    for (org_a, prop) in pop {
        let new_gen = org_a.mate(&player.clone());
        for (org_i, prop_i) in new_gen {
            *new_pop.entry(org_i.clone()).or_insert(0f64) += prop * prop_i;
        }
    }
    new_pop
}

fn binom(n: u64, k: u64) -> BigInt {
    let mut res = BigInt::one();
    for i in 0..k {
        res = (res * (n - i).to_bigint().unwrap()) /
              (i + 1).to_bigint().unwrap();
    }
    res
}

fn binom_prop(n: u64, k: u64, prop: &f64) -> f64 {
    binom(n, k).to_f64().unwrap() * prop.powf(k as f64) * (1f64 - prop).powf((n-k) as f64)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut n: i64 = 0;
    let mut k: i64 = 0;
    for (i, p) in contents.split_whitespace().enumerate() {
        match i {
            0 => k = p.parse().unwrap(),
            1 => n = p.parse().unwrap(),
            _ => println!("Got more than 2 numbers in the input txt.")
        }
    }
    let org_ref = Organism::new("Aa", "Bb");
    let mut pop: HashMap<Organism, f64> = HashMap::new();
    pop.insert(org_ref.clone(), 1f64);
    for _ in 1..k {
        pop = mate(pop, org_ref.clone());
    }
    let prop_binom = &pop.get(&org_ref).unwrap();
    let mut prop = 0f64;
    let n_tot: u64 = 2u64.pow(k as u32) as u64;
    for i in 0..n {
        prop += binom_prop(n_tot, i as u64, *prop_binom);
    }
    prop = 1f64 - prop;
    println!("{:.3}", prop);
}