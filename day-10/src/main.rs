use std::env;
use std::fs;

use std::cmp;
use std::collections::HashMap;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn dfs(
    map: &Vec<Vec<char>>, 
    prev_pos: (u32,u32), 
    cur_pos: (u32,u32), 
    visited: Vec<(u32,u32)>,
    compatible_to_the_left: &HashMap<char, Vec<char>>,
    compatible_to_the_right: &HashMap<char, Vec<char>>,
    compatible_to_the_top: &HashMap<char, Vec<char>>,
    compatible_to_the_bottom: &HashMap<char, Vec<char>>
) -> i32 {
    if visited.contains(&cur_pos) {
        return -1;
    }

    let mut new_visited = visited.clone();
    new_visited.push(cur_pos);

    let mut top = 0;
    let mut bottom = 0;
    let mut left = 0;
    let mut right = 0;

    let cur_char = map[cur_pos.0 as usize][cur_pos.1 as usize];
    
    // prev is not top
    if cur_pos.0 > 0 && prev_pos != (cur_pos.0 - 1, cur_pos.1) {
        let new_char = map[(cur_pos.0-1) as usize][cur_pos.1 as usize];
        if compatible_to_the_top.get(&cur_char).unwrap().contains(&new_char) {
            top = dfs(map, cur_pos, (cur_pos.0-1, cur_pos.1), new_visited.clone(),  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom);
        }
    }

    // prev is not bottom
    if cur_pos.0 < map.len() as u32 && prev_pos != (cur_pos.0 + 1, cur_pos.1) {
        let new_char = map[(cur_pos.0+1) as usize][cur_pos.1 as usize];
        if compatible_to_the_bottom.get(&cur_char).unwrap().contains(&new_char) {
            bottom = dfs(map, cur_pos, (cur_pos.0+1, cur_pos.1), new_visited.clone(),  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom);
        }
    }

    // prev is not left
    if cur_pos.1 > 0 as u32 && prev_pos != (cur_pos.0, cur_pos.1 - 1) {
        let new_char = map[cur_pos.0 as usize][(cur_pos.1 - 1) as usize];
        if compatible_to_the_left.get(&cur_char).unwrap().contains(&new_char) {
            left = dfs(map, cur_pos, (cur_pos.0, cur_pos.1 - 1), new_visited.clone(),  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom);
        }
    }

    // prev is not right
    if cur_pos.1 < map[0].len() as u32 && prev_pos != (cur_pos.0, cur_pos.1 + 1) {
        let new_char = map[cur_pos.0 as usize][(cur_pos.1 + 1) as usize];
        if compatible_to_the_right.get(&cur_char).unwrap().contains(&new_char) {
            right = dfs(map, cur_pos, (cur_pos.0, cur_pos.1 + 1), new_visited.clone(),  compatible_to_the_left, compatible_to_the_right, compatible_to_the_top, compatible_to_the_bottom);
        }
    }


    cmp::max(cmp::max(left, right), cmp::max(bottom, top))
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

    



}
