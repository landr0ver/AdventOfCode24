use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn part2(puzzle_input: &str) -> i32 {
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    
    let mut total = 0;
    let mut enabled = true;

    for cap in do_regex.find_iter(puzzle_input)
        .chain(dont_regex.find_iter(puzzle_input))
        .chain(mul_regex.captures_iter(puzzle_input)) {
        
        if let Some(m) = do_regex.find(cap.as_str()) {
            enabled = true;
        } else if let Some(m) = dont_regex.find(cap.as_str()) {
            enabled = false;
        } else if enabled {
            if let Some(captures) = mul_regex.captures(cap.as_str()) {
                let num1: i32 = captures[1].parse().unwrap();
                let num2: i32 = captures[2].parse().unwrap();
                total += num1 * num2;
            }
        }
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("/home/landr0ver/Documents/Projects/Rust/AdventOfCode24/Day3/input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let data = data.replace('\n', "");
    let result = part2(&data);
    println!("Total: {}", result);
    Ok(())
}

