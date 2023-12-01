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

fn replace_num_words(s: &str) -> String {
    let string_cpy = String::from(s);
    let string_ver = string_cpy.clone();
    let mut ret = string_cpy.into_bytes();

    for (index, _) in string_ver.match_indices("one") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'1';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("two") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'2';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("three") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'3';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("four") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'4';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("five") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'5';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("six") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'6';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("seven") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'7';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("eight") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'8';  // Replace 'R' with 'W'
        }
    }
    for (index, _) in string_ver.match_indices("nine") {
        if let Some(byte) = ret.get_mut(index) {
            *byte = b'9';  // Replace 'R' with 'W'
        }
    }

    String::from_utf8(ret).expect("Invalid UTF-8")
}


fn main() {
    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    let mut sum: u32 = 0;

    for line in lines.iter(){
        let without_num = replace_num_words(line);
        println!("{line} ------ {without_num}");

        let first: u32 = match find_first_number_char(&without_num){
            Some(n) => n.to_digit(10u32).unwrap(),
            None => 0u32
        };
        let last: u32 = match find_last_number_char(&without_num){
            Some(n) => n.to_digit(10u32).unwrap(),
            None => 0u32
        };

        sum += 10*first + last;
    }

    println!("{}", sum);
}
