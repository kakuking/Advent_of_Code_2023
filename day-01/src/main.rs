use std::env;
use std::fs;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn find_first_number_char(s: &str) -> Option<char> {
    for c in s.chars() {
        if c.is_numeric() {
            return Some(c);
        }
    }
    None
}

fn find_last_number_char(s: &str) -> Option<char> {
    for c in s.chars().rev() {
        if c.is_numeric() {
            return Some(c);
        }
    }
    None
}

fn main() {
    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    let mut sum: u32 = 0;

    for line in lines.iter().take(5){
        let first = match find_first_number_char(line) {
            Some(n) => n.to_digit(10u32).unwrap(),
            None => 0u32
        };
        let last = match find_last_number_char(line) {
            Some(n) => n.to_digit(10u32).unwrap(),
            None => 0u32
        };

        sum += 10*first + last;
    }

    println!("{}", sum);
}
