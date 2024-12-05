use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// Reading line by line from file 
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn difference_in_range(x: i32, y: i32) -> bool {
    let z: i32 = (x-y).abs();
    z > 0 && z < 4 
}

fn main() {
    let lines = lines_from_file("/home/landr0ver/Documents/Projects/Rust/AdventOfCode24/Day2/input.txt").expect("Could not load lines");
    let mut list: Vec<Vec<i32>> = vec![];
    
    // add left value to list1 and right value to list2 
    for line in lines {
        let list_string_line: Vec<&str> = line.split(" ").collect();
        let mut list_elem: Vec<i32> = vec![];
        for i in 0..list_string_line.len() {
            let x: i32 = list_string_line[i].parse::<i32>().unwrap();
            list_elem.push(x)
        }
        list.push(list_elem);
    };
    
    let mut safe_counter: i32 = 0;

    for (_i, row) in list.iter_mut().enumerate() {
        let mut ascending: bool = false;
        let mut row_counter: i32 = 0;
        println!("{:?} \n", row);
        if row[0] > row[1] { ascending = false};
        if row[0] < row[1] { ascending = true};
        if ascending {
            for n in 0..row.len()-1 {
                if row[n] < row[n+1] && difference_in_range(row[n], row[n+1]) {
                    row_counter += 1;
                };
            }
        }
        if !ascending {
            for n in 0..row.len()-1 {
                if row[n] > row[n+1] && difference_in_range(row[n], row[n+1]) {
                    row_counter += 1;
                }
            }
        }
        if row_counter == (row.len()-1).try_into().unwrap() {
            safe_counter += 1;
        }
    }

    println!("{}", safe_counter);
}

