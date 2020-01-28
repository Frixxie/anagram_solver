use regex::Regex;
use std::char;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: program <dict> <phrase>");
    }

    let dict_raw = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"([^a-zA-Z])+").unwrap();
    let dict_list: Vec<&str> = re.split(dict_raw.as_str()).collect();
    let phrase = &args[2];

    let mut dict: String = String::new();

    dict = create_dict(dict_list.to_owned(), dict, 0, 1);
    //dict = create_dict(dict.to_owned(), dict.to_owned(), 0, 1);

    println!("{}", dict);

    let primes: Vec<bool> = seive_of_eratosthenes(103);

    let lettermap = create_lettermap(primes);

    /*
    for i in lettermap.keys().zip(lettermap.values()) {
        let (key, value) = i;
        println!("{}:{}", key, value)
    }
    */

    let lookuptable = create_lookuptable(dict_list.to_owned(), lettermap.to_owned());

    /*
    for i in lookuptable.keys().zip(lookuptable.values()) {
        let (key, value) = i;
        println!("{}:{:?}", key, value)
    }
    //println!("{}", lookuptable.len());
    */

    if lookuptable.contains_key(&translate_word_to_num(
        phrase.to_string().to_lowercase(),
        lettermap.to_owned(),
    )) {
        let list: Vec<String> = lookuptable
            .get(&translate_word_to_num(
                phrase.to_string().to_lowercase(),
                lettermap.to_owned(),
            ))
            .unwrap()
            .to_vec();
        println!(
            "{}:{}:{:?}",
            phrase,
            translate_word_to_num(phrase.to_string().to_lowercase(), lettermap.to_owned()),
            list
        );
    } else {
        println!("No match for {}", phrase)
    }
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

fn create_dict(dict: Vec<&str>, mut new_dict: String, current_depth: i8, max_depth: i8) -> String {
    println!("{}", dict.len());
    if new_dict == "" {
        for word in &dict {
            if word.len() > 1 {
                new_dict.push_str(format!("{} ", word).as_str());
            }
        }
        //new_dict = create_dict(dict.to_owned(), new_dict.to_owned(), current_depth + 1, max_depth);
    }
    if current_depth >= max_depth {
        return new_dict;
    }
    {
        let copy = new_dict.clone();
        for word in copy.split_whitespace() {
            for other_word in &dict {
                if (word.len() > 1 && other_word.len() > 1) && &word != other_word {
                    new_dict.push_str(format!("{}{} ", word, other_word).as_str());
                    //println!("{}", i);
                    //println!("{} {} {}", word, other_word, current_depth);
                }
            }
        }
    }
    new_dict = create_dict(
        dict.to_owned(),
        new_dict,
        current_depth + 1,
        max_depth,
    );
    new_dict
}

fn create_lookuptable(
    dict: Vec<&str>,
    lettermap: HashMap<char, i8>,
) -> BTreeMap<i128, Vec<String>> {
    let mut lookuptable: BTreeMap<i128, Vec<String>> = BTreeMap::new();

    for word in &dict {
        let val = translate_word_to_num(word.to_string().to_lowercase(), lettermap.to_owned());
        if !lookuptable.contains_key(&val) {
            let mut list: Vec<String> = Vec::new();
            list.push(word.to_string().to_lowercase());
            lookuptable.insert(val, list);
        } else {
            let mut list: Vec<String> = lookuptable.get(&val).unwrap().to_vec();
            if !list.contains(&word.to_string().to_lowercase()) {
                list.push(word.to_string().to_lowercase());
                lookuptable.insert(val, list);
            }
        }
    }
    lookuptable
}

fn translate_word_to_num(word: String, lettermap: HashMap<char, i8>) -> i128 {
    let mut sum: i128 = 1;
    for i in word.chars() {
        if i.is_alphabetic() {
            sum *= *(lettermap.get(&i).unwrap()) as i128;
        }
    }
    sum
}

fn seive_of_eratosthenes(n: i32) -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; (n + 1) as usize];
    primes[0] = false;
    primes[1] = false;
    let mut i: usize = 2;
    while i * i <= n as usize {
        if primes[i] == true {
            let mut j = i * i;
            while j <= n as usize {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    primes
}
