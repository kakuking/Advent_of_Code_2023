use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
use num::integer::lcm;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");
    let file_path = cur_dir.join(file_name);
    fs::read_to_string(file_path).expect("Error reading file")
}

fn words_left(cur_words: &Vec<&str>) -> bool {
    for word in cur_words {
        if !word.ends_with('Z') {
            // println!("{} ends without a Z", word);
            return true;
        }
    }

    false
}

fn for_one_word(word: &str, commands: &str, left: &HashMap<&str, &str>, right: &HashMap<&str, &str>) -> u64 {
    let mut i: u64 = 0;
    let mut cur_word = word;
    'outer_loop: while ! word.ends_with('Z') {

        for command in commands.chars() {
            if cur_word.ends_with("Z") {
                break 'outer_loop;
            }

            match command {
                'L' => cur_word = left.get(cur_word).unwrap(),
                'R' => cur_word = right.get(cur_word).unwrap(),
                _ => println!("Unknown command"),
            }
    
            i += 1;

            // println!("{}", cur_word);
        }
    }

    i
}

fn lcm_multiple(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 1; // LCM of an empty list is 1
    }
    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }
    result
}

fn main() {
    let contents = read_input_file("input");
    let lines: Vec<&str> = contents.lines().collect();

    let mut left: HashMap<&str, &str> = HashMap::new();
    let mut right: HashMap<&str, &str> = HashMap::new();
    
    let pattern = Regex::new(r"\b\w{3}\b").unwrap();
    
    let commands = *(lines.get(lines.len() - 1).unwrap());

    let mut cur_words: Vec<&str> = Vec::new();

    for line in lines {
        let mut i = 0;
        let mut key: &str = "";
        let mut l: &str = "";
        let mut r: &str = "";
        for mat in pattern.find_iter(line) {
            match i {
                0 => {key = mat.as_str(); if key.ends_with('A') { cur_words.push(key); }},
                1 => l = mat.as_str(),
                2 => r = mat.as_str(),
                _ => println!("More than 3 matches in a line"),
            }
            i+=1;
        }

        left.insert(key, l);
        right.insert(key, r);
    }

    // let mut cur_word = "AAA";

    let mut cur_steps: Vec<u64> = Vec::new();

    for word in cur_words {
        cur_steps.push(for_one_word(word, commands, &left, &right));
        println!("{:?}", word);
        println!("{:?}", cur_steps);
    }

    println!("{:?}", cur_steps);

    let lcm = lcm_multiple(cur_steps);

    println!("{}", lcm);


    // println!("{:?}", left.get("HLT"));
    // println!("\n\n\n\n{:?}", right.get("HLT"));

}
