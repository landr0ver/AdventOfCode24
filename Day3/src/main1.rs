use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

fn part2(input_string: &str) -> i32 {
    let do_regex_string = Regex::new(r"do\(\)").unwrap();
    let dont_regex_string = Regex::new(r"don't\(\)").unwrap();
    let mul_regex_string = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    // to keep track of the do and don't status
    let mut enabled = true;

    for cap in do_regex_string.find_iter(input_string)
        .chain(dont_regex_string.find_iter(input_string))
        .chain(mul_regex_string.captures_iter(input_string)) {

        if let Some(m) = do_regex_string.find(cap.as_str()) {
            enabled = true;
        } else if let Some(m) = dont_regex_string.find(cap.as_str()) {
            enabled = false;
        } else if enabled {
            if let Some(captures) = mul_regex_string.captures(cap.as_str()) {
                let x: i32 = captures[1].parse().unwrap();
                let y: i32 = captures[2].parse().unwrap();
                total += x * y;
            }
        }
    }

    total 

}

fn main() -> io::Result<()> {
    // Read the contents of the input file     
    let mut file = File::open("/home/landr0ver/Documents/Projects/Rust/AdventOfCode24/Day3/input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Remove newlines from the data
    let data = data.replace('\n', "");

    // Create a regex to find "mul(x, y)" patterns 
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    // Iterate over all matches
    for cap in re.captures_iter(&data) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        println!("x : {} | y : {}", a, b);
        total += a * b;
    }

    // Print the total mult score 
    println!("Part1 Total : {}", total);
    println!("Part2 Total : {}", part2(&data));
    Ok(())
}
