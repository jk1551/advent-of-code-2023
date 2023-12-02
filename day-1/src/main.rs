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
        let total = 0;
        for line in lines {
            if let Ok(word) = line {
                for letter in word.chars() {
                    let first: i32 = 0;
                    let last: i32 = 0;
                    // Set first number to first
                    // As you parse the numbers, set last number to last
                    println!("{}", letter);
                    println!("{}", first);
                    println!("{}", last);
                }
            }
        }
        return total;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
