use std::collections::HashMap;
use std::env;
use std::fs;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn for_a_spring<'a>(spring: &Vec<char>, group: &Vec<u32>, cache: &mut HashMap<(&Vec<char>, &Vec<u32>), u32>) -> u32 {

    // println!("Current sprng - {:?} CURRENT GROUPS - {:?}", spring, group);
    if spring.len() == 0 {
        return if group.len() == 0 { 1 } else { 0 };
    }

    if group.len() == 0 {
        return if spring.contains(&'#') { 0 } else { 1 }
    }

    let is_spring = vec!['?', '#'];
    let is_not = vec!['?', '.'];
    
    // if cache.contains_key(&(spring, group)) {
    //     return *cache.get(&(spring, group)).unwrap();
    // }

    let mut result = 0;

    if is_not.contains(&spring[0]) {
        result += for_a_spring(&spring[1..].to_vec(), group, cache);
    }


    if is_spring.contains(&spring[0]) {
        if 
        spring.len() >= group[0] as usize && 
        !spring[0..(group[0] as usize)].contains(&'.') &&
        (spring.len() == group[0] as usize || spring[group[0] as usize] != '#') 
        {
            result += if spring.len() > group[0] as usize {for_a_spring(&spring[((group[0] + 1) as usize)..].to_vec(), &group[1..].to_vec(), cache)} else {
                let new_vec: Vec<char> = Vec::new();
                for_a_spring(&new_vec, &group[1..].to_vec(), cache)
            };
        }
    }


    result
}

fn part_1 (springs: Vec<Vec<char>>, groups: Vec<Vec<u32>>) {
    let num_problems = springs.len();
    

    let mut final_result = 0;
    for i in 0..num_problems {
        let mut cache: HashMap<(&Vec<char>, &Vec<u32>), u32> = HashMap::new();
        let current_result = for_a_spring(springs.get(i).unwrap(), &groups[i], &mut cache);

        println!("{:?} --> {:?} ======= {}", springs[i], groups[i], current_result);

        final_result += current_result;
    }

    println!("FINAL RESULT: {}", final_result)
}


fn main() {
    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    let mut springs: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();

        let spring_temp: Vec<char> = split[0].chars().collect();
        // springs_str.push(spring.trim_start_matches('.').trim_end_matches('.'));
        let mut spring: Vec<char> = Vec::new();
        let repitition = 5;

        for _ in 0..repitition {
            for ch in &spring_temp {
                spring.push(*ch);
            }
        }


        springs.push(spring);

        let split_groups: Vec<&str> = split[1].split(',').collect();
        let groups_to_u32: Vec<u32> = split_groups
                                                            .iter()
                                                            .map(|&s| s.parse::<u32>().expect("Failure"))
                                                            .collect();

        let mut new_group: Vec<u32> = Vec::new();

        for _ in 0..repitition {
            for num in &groups_to_u32 {
                new_group.push(*num);
            }
        }
        groups.push(new_group);
    }

    part_1(springs, groups);
}


// 10089 too high