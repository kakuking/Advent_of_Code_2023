use std::env;
use std::fs;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");
    let file_path = cur_dir.join(file_name);
    fs::read_to_string(file_path).expect("Error reading file")
}

fn reduce_vector(cur_vec: &Vec<i64>) -> (Vec<i64>, bool) {
    let mut ret: Vec<i64> = Vec::new();

    let mut ret_bool = true;

    for i in 1..cur_vec.len() {
        let cur_diff = cur_vec[i] - cur_vec[i-1];

        if cur_diff != 0 {
            ret_bool = false;
        }

        ret.push(cur_diff);
    }

    (ret, ret_bool)
}

fn part_one(history: Vec<i64>) -> i64 {
    let mut reduced_vec: Vec<Vec<i64>> = Vec::new();

    reduced_vec.push(history);

    let mut all_zeros = false;

    while !all_zeros {
        let (new_vec, new_all_zeros) = reduce_vector(&reduced_vec[reduced_vec.len() - 1]);

        reduced_vec.push(new_vec);
        all_zeros = new_all_zeros;
    }
    let last_idx = reduced_vec.len() - 1;

    if let Some(lowest_level) = reduced_vec.get_mut(last_idx) {
        lowest_level.push(0);
    }

    for i in (0..reduced_vec.len()-1).rev() {
        let new_element = reduced_vec[i][reduced_vec[i].len() - 1] + reduced_vec[i+1][reduced_vec[i+1].len() - 1];
        if let Some(cur_level) = reduced_vec.get_mut(i) {
            cur_level.push(new_element);
        }
    }

    reduced_vec[0][reduced_vec[0].len() - 1]
}

fn part_two(history: Vec<i64>) -> i64 {
    let mut reduced_vec: Vec<Vec<i64>> = Vec::new();

    reduced_vec.push(history);

    let mut all_zeros = false;

    while !all_zeros {
        let (new_vec, new_all_zeros) = reduce_vector(&reduced_vec[reduced_vec.len() - 1]);

        reduced_vec.push(new_vec);
        all_zeros = new_all_zeros;
    }

    let last_idx = reduced_vec.len() - 1;

    if let Some(lowest_level) = reduced_vec.get_mut(last_idx) {
        lowest_level.push(0);
    }

    for i in (0..reduced_vec.len()-1).rev() {
        let new_element = reduced_vec[i][0] - reduced_vec[i+1][0];
        if let Some(cur_level) = reduced_vec.get_mut(i) {
            cur_level.insert(0, new_element);
        }
    }

    reduced_vec[0][0]
}

fn main() {
    let content = read_input_file("input");
    let lines: Vec<&str> = content.lines().collect();

    let mut all_histories: Vec<Vec<i64>> = Vec::new();
    
    for line in lines {
        let splits: Vec<&str> = line.split(' ').collect();

        let mut history: Vec<i64> = Vec::new();
        for split in splits {
            let parsed_split = split.parse::<i64>().unwrap();

            history.push(parsed_split);
        }

        all_histories.push(history);
    }

    let mut sum = 0;
    for history in all_histories {
        sum += part_two(history);
    }

    println!("{}", sum);
}