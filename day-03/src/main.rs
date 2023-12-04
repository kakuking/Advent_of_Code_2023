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

    let num_lines = lines.len();
    let line_length = lines[0].len();

    // Initialize a 2D Vec with the same number of rows and columns
    let mut numbers_at_idx: Vec<Vec<u32>> = vec![vec![0; line_length]; num_lines];

    let mut ret: u32 = 0;

    // find_num_at(7, 2, lines.clone());

    for (index, line) in lines.iter().enumerate() {
        for (index_char, c) in line.chars().enumerate() {
            if numbers_at_idx[index][index_char] != 0{
                continue;
            }
            
            if c.is_numeric() {
                let mut num: String = String::new(); 

                let mut cur_idx = index_char;
                let mut c_next = line.chars().nth(cur_idx).unwrap();

                let mut indices: Vec<usize> = vec![];
                
                while c_next.is_numeric() && cur_idx < line.len() {
                    num.push(c_next);
                    indices.push(cur_idx);
                    
                    cur_idx+=1;
                    if cur_idx >= line.len() {
                        break;
                    }
                    c_next = line.chars().nth(cur_idx).unwrap();
                }
                
                let number_from_char = num.parse::<u32>().unwrap();
                for idx in indices {
                    numbers_at_idx[index][idx] = number_from_char;
                }
            }
        }
    }

    for (index, line) in lines.iter().enumerate() {
        for (index_char, c) in line.chars().enumerate() {
            if !c.is_numeric() && c!= '.' {
                
                let mut temp_nums: Vec<u32> = vec![];
                for i in -1..=1 {
                    for j in -1..=1 {
                        if temp_nums.len() == 0 {
                            temp_nums.push(numbers_at_idx[(index as i32 + i) as usize][(index_char as i32 +j) as usize]);
                        } else {
                            let t = numbers_at_idx[(index as i32 + i) as usize][(index_char as i32 +j) as usize];
                            if temp_nums[temp_nums.len() - 1] != t {
                                temp_nums.push(t);
                            }
                        }
                    }
                }
                
                temp_nums.retain(|&x| x != 0);

                if temp_nums.len() == 2 {
                    ret += temp_nums[0] * temp_nums[1];
                }

                // for t in temp_nums {
                //     ret += t;
                // }
            }
        }
    }

    // for numbers in numbers_at_idx {
    //     for num in numbers {
    //         print!("{}, ", num);
    //     }
    //     println!("");
    // }

    println!("{ret}");
}
