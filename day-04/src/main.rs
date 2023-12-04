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
    let mut scratch_portion: Vec<&str> = vec![];

    let mut winning_nums = vec![vec![]];
    let mut elfing_nums = vec![vec![]];

    let num_lines = lines.len();

    for line in lines{
        let parts: Vec<&str> = line.split(':').collect();
        if let Some(second_part) = parts.get(1) {
            scratch_portion.push(*second_part);
        }
    }
    for line in scratch_portion {
        let parts: Vec<&str> = line.split('|').collect();

        let win: &str = if let Some(win_part) = parts.get(0) {
            win_part
        } else {
            ""
        };
        let elf: &str = if let Some(elf_part) = parts.get(1) {
            elf_part
        } else {
            ""
        };

        let num_wins: Vec<&str> = win.split(' ').collect();
        let num_elf: Vec<&str> = elf.split(' ').collect();

        let mut winning_t = vec![];
        let mut elfing_t = vec![];

        for t in num_wins {
            match t.parse::<i32>() {
                Ok(number) => {
                    winning_t.push(number);
                }
                Err(_) => {}
            };
        }
        
        for t in num_elf {
            match t.parse::<i32>() {
                Ok(number) => {
                    elfing_t.push(number);
                }
                Err(_) => {}
            };
        }

        winning_nums.push(winning_t);
        elfing_nums.push(elfing_t);
    }

    winning_nums.remove(0);
    elfing_nums.remove(0);

    let mut num_of_wins: Vec<u32> = vec![];

    for (idx, winning_t) in winning_nums.iter().enumerate() {
        let elfing_t = elfing_nums.get(idx).unwrap();
        let mut count = 0;
        for elf_num in elfing_t{
            count += winning_t.iter().filter(|&&x| x == *elf_num).count() as u32;
        }        
        num_of_wins.push(count);
    }

    let mut each_scratch_car: Vec<u32> = vec![1; num_lines];

    println!("HEYYY {}", each_scratch_car.len());

    for (idx, win) in num_of_wins.iter().enumerate() {
        for i in 1..=*win {
            if idx + i as usize >= each_scratch_car.len() {
                break;
            }
            each_scratch_car[idx + i as usize]+=1*each_scratch_car[idx];
        }
    }

    let mut ret = 0;

    for (idx, num) in each_scratch_car.iter().enumerate() {
        println!("{idx}:  num wins: {} ------ occurences: {}", num_of_wins[idx], num);
        ret += num;
    }

    print!("{ret}")

}
