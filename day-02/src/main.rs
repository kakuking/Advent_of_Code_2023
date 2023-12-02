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

    let mut ret: u32 = 0;

    for (_, line) in lines.iter().enumerate() {
        let colon_parts: Vec<&str> = line.split(":").collect();

        let sets: Vec<&str> = colon_parts[1].split(";").collect();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in sets{
            let each_color: Vec<&str> = set.split(",").collect();
            for color in each_color{
                let num_c: Vec<&str> = color.split(" ").collect();

                
                if num_c[2] == "red"{
                    let r = num_c[1].parse::<u32>().unwrap();
                    max_red = if r > max_red { r } else { max_red };
                }
                if num_c[2] == "green" {
                    let g = num_c[1].parse::<u32>().unwrap();
                    max_green = if g > max_green { g } else { max_green };

                }
                if num_c[2] == "blue" {
                    let b = num_c[1].parse::<u32>().unwrap();
                    max_blue = if b > max_blue { b } else { max_blue };
                }
            }
        }

        ret += max_red * max_green * max_blue ;
    }

    println!("{ret}")
}
