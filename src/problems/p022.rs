use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("data/p022.txt").unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    let mut names = BTreeSet::new();

    let mut in_word = false;
    let mut current_word = String::new();

    for c in file_string.chars() {
        if !in_word {
            if c == '"' {
                in_word = true;
            }
        } else {
            if c == '"' {
                in_word = false;
                names.insert(current_word);
                current_word = String::new();
            } else {
                current_word.push(c);
            }
        }
    }

    let mut total_score = 0;

    for (pos, name) in names.iter().enumerate() {
        if name == &"COLIN".to_string() {
            println!("{:?}", (pos, name));
            println!("{}", get_score(name));
        }
        total_score += (pos + 1) * get_score(name);
    }

    println!("{}", total_score);
}

fn get_score(s: &String) -> usize {
    let mut score = 0;
    for &c in s.as_bytes().iter() {
        score += c as usize - 'A' as usize + 1; // +1 because A is 1
    }

    score
}
