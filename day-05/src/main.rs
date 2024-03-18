use std::collections::HashMap;
use std::env;
use std::fs;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");
    let file_path = cur_dir.join(file_name);
    fs::read_to_string(file_path).expect("Error reading file")
}

fn string_to_map(strings: String) -> HashMap<(u64,u64),u64> {
    let lines: Vec<&str> = strings.lines().collect();
    let mut ret: HashMap<(u64,u64),u64> = HashMap::new();

    for line in lines{
        let nums: Vec<&str> = line.split(' ').collect();
        let target_start = nums[0].parse::<u64>().unwrap();
        let source_start = nums[1].parse::<u64>().unwrap();
        let range = nums[2].parse::<u64>().unwrap();

        ret.insert((source_start,source_start+range - 1), target_start);
    }

    ret
}

fn is_in_map(map: &HashMap<(u64,u64),u64>, key: u64) -> Option<u64> {
    for (k, v) in map.iter() {
        if k.0 <= key && key <= k.1 {
            return  Some(key - k.0 + v);
        }
    }

    None
}

fn tup_in_hashmap(tuples: Vec<(u64,u64)>, map: &HashMap<(u64,u64),u64>) -> Vec<(u64,u64)> {
    let mut ret: Vec<(u64,u64)> = Vec::new();

    for tup in tuples {
        for (k, v) in map {
            if k.0 <= tup.0 && tup.0 <= k.1 {
                if tup.1 <= k.1 {
                    // ret.push((tup.0, tup.1));
                    ret.push((v + tup.0 - k.0, v + tup.1 - k.0));
                } else {
                    // ret.push((tup.0, k.1));
                    ret.push((v + tup.0 - k.0, v + k.1 - k.0));
                }
            } else if tup.0 <= k.0 && k.0 <= tup.1 {
                if k.1 <= tup.1 {
                    // ret.push((k.0, k.1));
                    ret.push((v + k.0 - k.0, v + k.1 - k.0));
                } else {
                    // ret.push((k.0, tup.1));
                    ret.push((v + k.0 - k.0, v + tup.1 - k.0));
                }
            }
        }
    }

    ret
}

fn part_1(
    seed_arr: Vec<u64>,
    sts_map: HashMap<(u64,u64),u64>,
    stf_map: HashMap<(u64,u64),u64>,
    ftw_map: HashMap<(u64,u64),u64>,
    wtl_map: HashMap<(u64,u64),u64>,
    ltt_map: HashMap<(u64,u64),u64>,
    tth_map: HashMap<(u64,u64),u64>,
    htl_map: HashMap<(u64,u64),u64>){
    let mut min_loc: u64 = std::u64::MAX;
    let mut min_loc_seed: u64 = 0;

    for seed in seed_arr{

        let soil = match is_in_map(&sts_map, seed) {
            Some(num) => num,
            None => {continue;}
        };
        let fert = match is_in_map(&stf_map, soil) {
            Some(num) => num,
            None => {continue;}
        };
        let water = match is_in_map(&ftw_map, fert) {
            Some(num) => num,
            None => {continue;}
        };
        let light = match is_in_map(&wtl_map, water) {
            Some(num) => num,
            None => {continue;}
        };
        let temp = match is_in_map(&ltt_map, light) {
            Some(num) => num,
            None => {continue;}
        };
        let humid = match is_in_map(&tth_map, temp) {
            Some(num) => num,
            None => {continue;}
        };
        let loc = match is_in_map(&htl_map, humid) {
            Some(num) => num,
            None => {continue;}
        };

        if loc < min_loc {
            min_loc = loc;
            min_loc_seed = seed;
        }
            
    }


    println!("{min_loc}");
    println!("{min_loc_seed}");
}

fn part_2(
    seeds: Vec<(u64, u64)>,
    sts_map: HashMap<(u64,u64),u64>,
    stf_map: HashMap<(u64,u64),u64>,
    ftw_map: HashMap<(u64,u64),u64>,
    wtl_map: HashMap<(u64,u64),u64>,
    ltt_map: HashMap<(u64,u64),u64>,
    tth_map: HashMap<(u64,u64),u64>,
    htl_map: HashMap<(u64,u64),u64>
) -> u64 {
    let soil = tup_in_hashmap(seeds, &sts_map);
    let fert = tup_in_hashmap(soil, &stf_map);
    let water = tup_in_hashmap(fert, &ftw_map);
    let light = tup_in_hashmap(water, &wtl_map);
    let temp = tup_in_hashmap(light, &ltt_map);
    let humid = tup_in_hashmap(temp, &tth_map);
    let locs = tup_in_hashmap(humid, &htl_map);

    let mut cur_min_loc: u64 = std::u64::MAX;

    for loc in &locs {
        if loc.0 < cur_min_loc {
            cur_min_loc = loc.0;
        }

        if loc.1 < cur_min_loc {
            cur_min_loc = loc.1;
        }

        println!("{}", loc.0);
    }

    println!("\n\n{cur_min_loc}");

    
    cur_min_loc
}

fn main() {
    let seeds  = read_input_file("input/seeds");
    let seed_to_soil  = read_input_file("input/seed-to-soil");
    let soil_to_fertilizer  = read_input_file("input/soil-to-fertilizer");
    let fertilizer_to_water  = read_input_file("input/fertilizer-to-water");
    let water_to_light  = read_input_file("input/water-to-light");
    let light_to_temp  = read_input_file("input/light-to-temp");
    let temp_to_humidity  = read_input_file("input/temp-to-humidity");
    let humidity_to_location  = read_input_file("input/humidity-to-location");
    
    let seed_iter: Vec<&str> = seeds.split(' ').collect();
    // let seed_arr: Vec<u64> = seed_iter.iter().map(|s| s.parse::<u64>().unwrap()).collect();          // Part 1
    let seed_arr: Vec<(u64, u64)> = seed_iter
        .chunks(2)
        .map(|chunk| {
            let first_value = chunk[0].parse::<u64>().unwrap();
            let second_value = chunk[1].parse::<u64>().unwrap();
            let sum = first_value + second_value - 1;
            (first_value, sum)
        })
        .collect();

    // println!("{:?}", seed_arr);

    // print!("{:?}",seed_arr);

    let sts_map = string_to_map(seed_to_soil);
    let stf_map = string_to_map(soil_to_fertilizer);
    let ftw_map = string_to_map(fertilizer_to_water);
    let wtl_map = string_to_map(water_to_light);
    let ltt_map = string_to_map(light_to_temp);
    let tth_map = string_to_map(temp_to_humidity);
    let htl_map = string_to_map(humidity_to_location);

    // part_1(seed_arr, sts_map, stf_map, ftw_map, wtl_map, ltt_map, tth_map, htl_map);
    let ans = 26714516;

    for (k, v) in &htl_map {
        if &ans > v && &ans - v <= k.1 - k.0 {
            println!("YAHOOOOO");
        // } else {
        //     // println!("NAOOOOOO");
        }
    }

    part_2(seed_arr, sts_map, stf_map, ftw_map, wtl_map, ltt_map, tth_map, htl_map);
}
