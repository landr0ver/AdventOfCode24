use std::fs;

fn read_file_content_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let string_content = fs::read_to_string(path)?;
    Ok(string_content)
}

// function where only the numbers of the multiplications are added 
// so you need to multiply first and second, third and fourth and so on 
fn split_into_vec(s: &String) -> Vec<&str> {
    // split into line for readability
    let parts = s.split("mul"); 
    let vec_lines = parts.collect::<Vec<&str>>();
    vec_lines
}

fn clean_input(s: String) -> String {
    let mut result_s: String = String::from("");
    let my_chars: Vec<_> = s.chars().collect();
    for i in 1..my_chars.len()-1 {
        let c = my_chars[i];
        match c {
            'm' => {
                if my_chars[i+1] == 'u' && my_chars[i+2] == 'l' {
                    result_s.push(c);
                }
            },
            'u' => {
                if my_chars[i-1] == 'm' && my_chars[i+1] == 'l' {
                    result_s.push(c);
                }
            },
            'l' => {
                if my_chars[i-2] == 'm' && my_chars[i-1] == 'u' {
                    result_s.push(c);
                }
            },
            '(' => {
                if my_chars[i+1].is_numeric() {
                    result_s.push(c);
                }
            },
            ')' => {
                if my_chars[i-1].is_numeric() {
                    result_s.push(c);
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                result_s.push(c);
            },
            ',' => {
                if my_chars[i-1].is_numeric() && my_chars[i+1].is_numeric() {
                    result_s.push(c);
                }
            }
            _ => continue,
        }
    } 
    result_s
}

fn main() {
    let s = read_file_content_as_string("/home/landr0ver/Documents/Projects/Rust/AdventOfCode24/Day3/input.txt");
    // modifie s so that only the characters which are valid are in the input text 
    let binding = &clean_input(s.expect("Failed to parse input File to String"));
    let vec = split_into_vec(binding);
    /*for el in &vec {
        println!("{:?}", el);
    };*/
    let mut score: i64 = 0;
    for elem in vec {
        if elem.len() > 9 || elem.len() < 5 || !elem.contains(',') {
            continue;
        }
        // elem which is valid needs to be parsed into nuber x and y
        let mut e_copy = elem.to_string();
        e_copy.pop();
        e_copy.remove(0);
        let parts = e_copy.split(',');
        let vec_number = parts.collect::<Vec<&str>>();
        // multiplication and score adding
        let mult = vec_number[0].parse::<i64>().unwrap() * vec_number[1].parse::<i64>().unwrap();
        score += mult;
        println!("x: {} | y: {} | mult: {}", vec_number[0], vec_number[1], mult);
    }
    println!("The score is: {}", score);
}
