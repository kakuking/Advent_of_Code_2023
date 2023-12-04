use std::env;
use std::fs;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn main() {
    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    for line in lines{
        println!("{line}");
    }
}
