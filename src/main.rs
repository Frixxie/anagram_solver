use std::collections::HashMap;
use std::env;
use std::fs;
use std::char;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: program <dict> <phrase>");
    }

    let dict = fs::read_to_string(&args[1]).unwrap();

    let primes: Vec<bool> = seive_of_eratosthenes(103);
    /*
    let mut val: i32 = 0;
    for i in primes.iter() {
        if i == &true {
            println!("{}", val)
        }
        val += 1;
    }
    */

    let lettermap = create_lettermap(primes);

    for i in lettermap.keys().zip(lettermap.values()) {
        let (key, value) = i;
        println!("{}:{}", key, value)
    }

    println!("{}", translate_word_to_num("abc".to_string(), lettermap))

}

fn create_lettermap(primes: Vec<bool>) -> HashMap<char, i8> {
    let mut lettermap: HashMap<char, i8> = HashMap::new();
    let mut j: usize = 2;
    for i in 0..26 {
        if primes[j] == true { 
            lettermap.insert(char::from_digit((i + 10) as u32, 36).unwrap(), j as i8);
            j += 1
        }
        while primes[j] != true {
            j += 1
        }
    }
    lettermap
}

fn create_lookuptable(dict: String, lettermap: HashMap<char, i8>) -> HashMap<i32,Vec<String>> {
    let mut lookuptable: HashMap<i32,Vec<String>> = HashMap::new();
    for word in dict.split_whitespace() {
        let val = translate_word_to_num(word.as_string(), lettermap);
        let list = lookuptable.get(&val);
    }
}

fn translate_word_to_num(word: String, lettermap: HashMap<char, i8>) -> i32 {
    let mut sum: i32 = 1;
    for i in word.chars() {
        sum *= *(lettermap.get(&i).unwrap()) as i32;
    }
    sum
}

fn seive_of_eratosthenes(n: i32) -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; (n + 1) as usize];
    primes[0] = false;
    primes[1] = false;
    let mut i: usize = 2;
    while i*i <= n as usize {
        if primes[i] == true {
            let mut j = i*i;
            while j <= n as usize {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    primes
}
