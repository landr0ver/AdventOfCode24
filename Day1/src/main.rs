use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    collections::HashMap,
};

// Reading line by line from file 
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("/home/landr0ver/Documents/Projects/Rust/AdventOfCode24/Day1/input.txt").expect("Could not load lines");
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    
    // add left value to list1 and right value to list2 
    for line in lines {
        let list: Vec<&str> = line.split("   ").collect();
        let x: i32 = list[0].parse::<i32>().unwrap();
        let y: i32 = list[1].parse::<i32>().unwrap();
        list1.push(x);
        list2.push(y);
    }
    list1.sort();
    list2.sort();
    // calculate distance from Task1
    let i = list1.len(); 
    let mut distance: i32 = 0;
    for n in 0..i {
        distance += (list1[n] - list2[n]).abs();
    }
     println!("Distance: {distance}");

    // TASK 2
    // Generate Map Frequencies of the Values in the right list 
    let frequencies = list2 
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val|{
            map.entry(val)
                .and_modify(|frq|*frq+=1)
                .or_insert(1);
            map 
        });

    // Match the values from the left list and multiply them by the occurence factor from the right
    // list 
    let mut similarity: i64 = 0;
    for n in 0..i {
        // check if list1[n] is in the second list (HashMap)
        if(frequencies.contains_key(&list1[n])) {
            similarity += (list1[n] * frequencies.get(&list1[n]).unwrap()) as i64;
        }
    }
    println!("{similarity}");
}
