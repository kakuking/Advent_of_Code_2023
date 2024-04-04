use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn read_input_file(file_name: &str) -> String {
    let cur_dir = env::current_dir().expect("Error getting curdir");


    let file_path = cur_dir.join(file_name);

    fs::read_to_string(file_path).expect("Error reading file")
}

fn write_to_file(data: &Vec<Vec<char>>) -> std::io::Result<()>{
    let mut content = String::new();
    for row in data {
        for &ch in row {
            content.push(ch);
        }
        content.push('\n'); // Add a newline after each row
    }

    // Write the string to a file
    let mut file = File::create("output")?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn expand_universe(universe: &mut Vec<Vec<char>>, new_rows: &mut Vec<usize>, new_cols: &mut Vec<usize>, to_adjust: bool) {
    // For rows - 
    let row_len = universe[0].len();

    'row_traversal: for row_idx in 0..universe.len() {
        let row = universe.get_mut(row_idx).unwrap();
        for position in row {
            if position == &mut '#' {
                continue 'row_traversal;
            }
        }
        new_rows.push(row_idx);
    }

    // For columns

    'col_traversal: for col_idx in 0..row_len {
        for row_idx in 0..universe.len() {
            let row = universe.get(row_idx).unwrap();
            if row[col_idx] == '#' {
                continue 'col_traversal;
            }
        }
        
        new_cols.push(col_idx);
    }

    if !to_adjust {
        return;
    }

    let mut adjust = 0;
    for new_row in new_rows {
        let row = vec!['.'; row_len];

        println!("New row at {} with adjust {}", new_row, adjust);
        universe.insert(*new_row + adjust, row);

        adjust += 1;
    }

    adjust = 0;
    for new_col in new_cols {
        println!("New col at {} with adjust {}", new_col, adjust);
        for row_idx in 0..universe.len() {
            let row = universe.get_mut(row_idx).unwrap();

            row.insert(if *new_col + adjust < row.len()  {*new_col + adjust} else {row.len()}, '.');
        }
        adjust += 1;
    }
}

fn part_1(galaxies: Vec<(i32, i32)>){
    let mut total_lengths = 0;
    for first_idx in 0..(galaxies.len()) {
        for second_idx in (first_idx+1)..galaxies.len() {
            let first = galaxies.get(first_idx).unwrap();
            let second = galaxies.get(second_idx).unwrap();

            total_lengths += (first.0 - second.0).abs() + (first.1 - second.1).abs();
        }
    }

    println!("{}", total_lengths);
}

fn part_2(galaxies: Vec<(i32, i32)>, new_rows: Vec<usize>, new_cols: Vec<usize>){
    let mut total_lengths: i64 = 0;
    for first_idx in 0..(galaxies.len()) {
        for second_idx in (first_idx+1)..galaxies.len() {
            let first = galaxies.get(first_idx).unwrap();
            let second = galaxies.get(second_idx).unwrap();

            let mut new_rows_between = 0;
            let mut new_cols_between = 0;

            for new_row in &new_rows {
                let new_row = *new_row as i32;
                if (first.0 - new_row) * (second.0 - new_row) < 0 {
                    new_rows_between += 1;
                }
            }

            for new_col in &new_cols {
                let new_col = *new_col as i32;
                if (first.1 - new_col) * (second.1 - new_col) < 0 {
                    new_cols_between += 1;
                }
            }


            total_lengths += ((first.0 - second.0).abs() + (first.1 - second.1).abs() + new_rows_between * 999999 + new_cols_between * 999999) as i64;
        }
    }

    println!("{}", total_lengths);
}

fn main() {
    let filename = "input";
    let contents = read_input_file(filename);
    let lines: Vec<&str> = contents.lines().collect();

    let mut universe: Vec<Vec<char>> = Vec::new();

    for line in &lines {
        let char_vec: Vec<char> = line.chars().collect();
        universe.push(char_vec);
    }
    let mut new_rows: Vec<usize> = Vec::new();
    let mut new_cols: Vec<usize> = Vec::new();
    
    expand_universe(&mut universe, &mut new_rows, &mut new_cols, false);

    // write_to_file(&universe);

    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    for i in 0..universe.len(){
        let cluster = &universe[i];

        for j in 0..cluster.len() {
            if cluster[j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    // part_1(galaxies);  

    part_2(galaxies, new_rows, new_cols);

}

// 9431947 is too high
// 9174337 is too low