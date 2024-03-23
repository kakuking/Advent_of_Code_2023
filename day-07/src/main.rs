use std::env;
use std::fs;
use std::collections::HashMap;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn hand_strength(
    hand: &str
) -> u64 {
    let mut card_number: HashMap<char,u64> = HashMap::new();
    let mut value_J = 0;

    let mut max_value = 0;
    let mut max_char = '0';

    for card in hand.chars() {
        if card_number.contains_key(&card) {
            if let Some(value) = card_number.get_mut(&card) {
                *value += 1;

                if *value > max_value {
                    max_value = *value;
                    max_char = card;
                }
            }
        } else {
            card_number.insert(card, 1);

            if 1 > max_value {
                max_value = 1;
                max_char = card;
            }
        }

        if card == 'J' {
            value_J += 1;
        }
    }

    let value_max = card_number.get_mut(&max_char).unwrap();
    *value_max += value_J;

    if card_number.len() > 1 {
        card_number.remove(&'J');
    }

    if card_number.len() == 5 {
        return 1;               // High cards
    }

    if card_number.len() == 4 {
        return 2;               // One Pair
    }

    if card_number.len() == 3 {
        for (k, v) in card_number {
            if v == 2 {
                return 3;       // Two Pair
            }
        }

        return 4;               // Three of a kind (no 2s)
    }

    if card_number.len() == 2 {
        for (k, v) in card_number {
            if v == 2 {
                return 5;       // Full House  
            }
        }

        return 6;               // Four of a kind (no 2s)
    }

    7                           // Five of a kind (everything is unique)
}

// True if first_hand > second_hand else False
fn hand_order(first_hand: &str, second_hand: &str) -> bool {
    let first_chars: Vec<char> = first_hand.chars().collect();
    let second_chars: Vec<char> = second_hand.chars().collect();

    let mut start_index: usize = 0;

    while first_chars[start_index] == second_chars[start_index]{
        start_index += 1;
    }

    // if first_chars[start_index] == 'A' {
    //     return true;
    // }

    // if second_chars[start_index] == 'A' {
    //     return false;
    // }

    let first_changed = first_hand.replace('A',"Z")
                                    .replace('K', "Y")
                                    .replace('Q',"X")
                                    .replace('J', "0")
                                    .replace('T', "V");

    let second_changed = second_hand.replace('A',"Z")
                                    .replace('K', "Y")
                                    .replace('Q',"X")
                                    .replace('J', "0")
                                    .replace('T', "V");
                                
    return first_changed[start_index..first_hand.len()] > second_changed[start_index..first_hand.len()]
}

fn main() {
    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    let mut strength_map: HashMap<u64, Vec<(&str,u64)>> = HashMap::new();

    for i in 1..=7 {
        let vec:Vec<(&str,u64)> = Vec::new();
        strength_map.insert(i, vec);
    }

    for line in lines {
        let mut parts: Vec<&str> = line.split(' ').collect();
        let bid = parts[1].parse::<u64>().unwrap();

        let hand = parts.get_mut(0).unwrap();

        // let new_hand = hand.replace('A',"Z").replace('K', "Y")
        //                             .replace('Q',"X")
        //                             .replace('J', "W")
        //                             .replace('J', "V")
        //                             .replace('T', "U");

        if let Some(value) = strength_map.get_mut(&hand_strength(&hand)) {
            value.push((&hand,bid));
        }
    }

    for j in (1 as u64)..=(7 as u64){
        if let Some(value) = strength_map.get_mut(&j) {
            value.sort_by(|a, b| hand_order(a.0, b.0).cmp(&true));
        }
    }

    let mut current_returns = 0;
    let mut current_rank = 0;
    for i in 1..=7 {
        if let Some(value) = strength_map.get(&i) {
            for hand in value {
                current_rank += 1;
                current_returns += current_rank * hand.1;

                println!("{}. {} - {}", current_rank, hand.0, hand.1);
            }
        }
    }

    
    // for (k, v) in strength_map {
    //     println!("{k} - {:?}", v);
    // }
    
    // println!("{}", current_rank);
    println!("{}", current_returns);
}


// Not 250996603
// answer is 247815719