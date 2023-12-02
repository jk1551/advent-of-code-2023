// Objectives:
// Parse first and last number from each line
// Store the combined value in a total value
// Return the total value

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Things to note - some lines only have 1 value which means its the first and the last

fn main() {
    // Cheeck to make sure file is readable per Rust docs
    if let Ok(lines) = read_lines("./puzzle-input.txt") {
        // Create variable to store total
        let mut _total: i32 = 0;
        for line in lines {
            if let Ok(word) = line {
                let mut _first: Option<i32> = None; 
                let mut _last: Option<i32> = None; 
                for letter in word.chars() {
                    // Set first number to first
                    // As you parse the numbers if there is more than 1, set last number to last
                    if letter.is_numeric() {
                        if let Some(digit_value) = letter.to_digit(10) {
                            match _first {
                                Some(_value) => {
                                    println!("{}", digit_value as i32);
                                    _last = Some(digit_value as i32);
                                }
                                None => {
                                    println!("{}", digit_value);
                                    _first = Some(digit_value as i32);
                                    _last = Some(digit_value as i32);
                                }
                            }
                        }
                    }
                }
                // Concat the first and last value into a string so that it is added like
                // 2 + 2 = 22
                // Not 2 + 2 = 4
                let concatenated_sum = match (_first, _last) {
                    (Some(first_str), Some(last_str)) => format!("{}{}", first_str, last_str),
                    (Some(first_str), None) => format!("{}{}", first_str, first_str),
                    _ => String::new(),
                };
        
                if let Ok(sum) = concatenated_sum.parse::<i32>() {
                    // convert the string back to int32 so that we can add to the total
                    _total += sum;
                    println!("Total: {}", _total);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
