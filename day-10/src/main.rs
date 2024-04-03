use std::env;
use std::fs;

use std::cmp;
use std::collections::HashMap;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn part_1(
    map: &Vec<Vec<char>>, 
    start: (u32, u32),
    compatible_to_the_left: &HashMap<char, Vec<char>>,
    compatible_to_the_right: &HashMap<char, Vec<char>>,
    compatible_to_the_top: &HashMap<char, Vec<char>>,
    compatible_to_the_bottom: &HashMap<char, Vec<char>>,
    level: u32
) -> i32 {

    

    // if level == 50 {
    //     return 0;
    // }

    
    let mut top = 0;
    let mut bottom = 0;
    let mut left = 0;
    let mut right = 0;

    let mut length = 0;
    
    let mut cur_char = map[start.0 as usize][start.1 as usize];
    
    // println!("{} - {}", level, cur_char);

    let mut prev_pos = (0, 0);
    let mut cur_pos = start.clone();
    
    while true {
        if cur_pos == start && length > 0 {
            break;
        }

        cur_char = map[cur_pos.0 as usize][cur_pos.1 as usize];

        println!("Currently - {:?} ;; Previously - {:?} ;; Started - {:?} ;; Length - {}", cur_pos, prev_pos, start, length);

        // prev is not top
        if cur_pos.0 > 0 && prev_pos != (cur_pos.0 - 1, cur_pos.1) {
            let new_char = map[(cur_pos.0-1) as usize][cur_pos.1 as usize];
            if compatible_to_the_top.get(&cur_char).unwrap().contains(&new_char) {
                // println!("Top");
                // top = dfs(map, cur_pos, (cur_pos.0-1, cur_pos.1), start,  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom, level + 1);
                prev_pos = cur_pos;
                cur_pos = (cur_pos.0 - 1, cur_pos.1);
                length += 1;
    
                continue;
            }
        }
    
        // prev is not bottom
        if cur_pos.0 < map.len() as u32 && prev_pos != (cur_pos.0 + 1, cur_pos.1) {
            let new_char = map[(cur_pos.0+1) as usize][cur_pos.1 as usize];
            if compatible_to_the_bottom.get(&cur_char).unwrap().contains(&new_char) {
                // println!("Bottom");
                // bottom = dfs(map, cur_pos, (cur_pos.0+1, cur_pos.1), start,  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom, level + 1);
                prev_pos = cur_pos;
                cur_pos = (cur_pos.0 + 1, cur_pos.1);
                length += 1;
    
                // return bottom + 1;
                continue;
            }
        }
    
        // prev is not left
        if cur_pos.1 > 0 as u32 && prev_pos != (cur_pos.0, cur_pos.1 - 1) {
            let new_char = map[cur_pos.0 as usize][(cur_pos.1 - 1) as usize];
            if compatible_to_the_left.get(&cur_char).unwrap().contains(&new_char) {
                // println!("Left");
                // left = dfs(map, cur_pos, (cur_pos.0, cur_pos.1 - 1), start,  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom, level + 1);
                prev_pos = cur_pos;
                cur_pos = (cur_pos.0, cur_pos.1 - 1);
                length += 1;

                continue;
                // return left + 1;
            }
        }
    
        // prev is not right
        if cur_pos.1 < map[0].len() as u32 && prev_pos != (cur_pos.0, cur_pos.1 + 1) {
            let new_char = map[cur_pos.0 as usize][(cur_pos.1 + 1) as usize];
            if compatible_to_the_right.get(&cur_char).unwrap().contains(&new_char) {
                // println!("Right");
                // right = dfs(map, cur_pos, (cur_pos.0, cur_pos.1 + 1), start,  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom, level + 1);
                prev_pos = cur_pos;
                cur_pos = (cur_pos.0, cur_pos.1 + 1);
                length += 1;
    
                continue;
                // return right  + 1;
            }
        }

        println!("NOthing compatible");
    }

    return length
    // cmp::max(cmp::max(left, right), cmp::max(bottom, top))
}

fn main() {

    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in &lines {
        let char_vec: Vec<char> = line.chars().collect();
        map.push(char_vec);
    }

    // | - L J 7 F . S           S is an L

    let mut compatible_to_the_left: HashMap<char, Vec<char>> = HashMap::new();
    let mut compatible_to_the_right: HashMap<char, Vec<char>> = HashMap::new();
    let mut compatible_to_the_top: HashMap<char, Vec<char>> = HashMap::new();
    let mut compatible_to_the_bottom: HashMap<char, Vec<char>> = HashMap::new();

    compatible_to_the_left.insert('|', vec![]);
    compatible_to_the_left.insert('-', vec!['L','F','-']);
    compatible_to_the_left.insert('L', vec![]);
    compatible_to_the_left.insert('J', vec!['L','F','-']);
    compatible_to_the_left.insert('7', vec!['L','F','-']);
    compatible_to_the_left.insert('F', vec![]);

    compatible_to_the_right.insert('|', vec![]);
    compatible_to_the_right.insert('-', vec!['7','J','-']);
    compatible_to_the_right.insert('L', vec!['7','J','-']);
    compatible_to_the_right.insert('J', vec![]);
    compatible_to_the_right.insert('7', vec![]);
    compatible_to_the_right.insert('F', vec!['7','J','-']);

    compatible_to_the_top.insert('|', vec!['7','F','|']);
    compatible_to_the_top.insert('-', vec![]);
    compatible_to_the_top.insert('L', vec!['7','F','|']);
    compatible_to_the_top.insert('J', vec!['7','F','|']);
    compatible_to_the_top.insert('7', vec![]);
    compatible_to_the_top.insert('F', vec![]);

    compatible_to_the_bottom.insert('|', vec!['L','J','|']);
    compatible_to_the_bottom.insert('-', vec![]);
    compatible_to_the_bottom.insert('L', vec![]);
    compatible_to_the_bottom.insert('J', vec![]);
    compatible_to_the_bottom.insert('7', vec!['L','J','|']);
    compatible_to_the_bottom.insert('F', vec!['L','J','|']);

    let mut start_i = 0;
    let mut start_j = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start_i = i;
                start_j = j;

                map[i][j] = 'J';
            }
        }
    }

    
    let ret = part_1(
        &map, 
        (start_i as u32, start_j as u32), 
        &compatible_to_the_left, 
        &compatible_to_the_right, 
        &compatible_to_the_top, 
        &compatible_to_the_bottom,
        0);

    println!("{}", ret);


}
