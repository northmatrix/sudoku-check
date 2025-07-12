use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        let data: String = fs::read_to_string(file_path)?;
        let digits: Vec<u32> = data.trim().chars().filter_map(|x| x.to_digit(10)).collect();
        let rows: bool = check_rows(&digits);
        let cols: bool = check_cols(&digits);
        let grids: bool = check_grids(&digits);
        if rows && cols && grids {
            println!("The Sudoku board is valid");
        } else {
            println!("The Sudoku board is invalid");
        }
        Ok(())
    } else {
        eprintln!("No file path supplied.");
        std::process::exit(1);
    }
}

fn check_rows(digits: &Vec<u32>) -> bool {
    let required: HashSet<u32> = (1..=9).collect();
    for x in 0..9 {
        let row = &digits[(x * 9)..(x * 9) + 9];
        let actual: HashSet<u32> = row.iter().copied().collect();
        if actual != required {
            return false;
        }
    }
    true
}

fn check_cols(digits: &Vec<u32>) -> bool {
    let required: HashSet<u32> = (1..=9).collect();
    for x in 0..9 {
        let mut actual: HashSet<u32> = HashSet::new();
        for y in 0..9 {
            actual.insert(digits[(y * 9) + x]);
        }
        if actual != required {
            return false;
        }
    }
    return true;
}

fn check_grids(digits: &Vec<u32>) -> bool {
    let required: HashSet<u32> = (1..=9).collect();
    for grid_row in 0..3 {
        for grid_col in 0..3 {
            let mut actual: HashSet<u32> = HashSet::new();
            for row in 0..3 {
                for col in 0..3 {
                    let idx = (grid_row * 3 + row) * 9 + (grid_col * 3 + col);
                    actual.insert(digits[idx]);
                }
            }
            if actual != required {
                return false;
            }
        }
    }
    return true;
}
