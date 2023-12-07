use file_reader::read_from_files;
use std::{collections::HashMap, io};

fn main() {
    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    let file_path = file_path.trim();

    let lines = read_from_files(file_path).expect("Failed to read file");
    let p1_sum = part1(lines.clone());
    println!("The sum of the part1 is: {}", p1_sum);

    let p2_sum = part2(lines);
    println!("The sum of the part2 is: {}", p2_sum);
}

fn part1(lines: Vec<String>) -> i32 {
    println!("start part 1");
    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| {
            let number_string: String = line.chars().filter(|c| c.is_numeric()).collect();
            let first_num = number_string.chars().next().unwrap();
            let last_num = number_string.chars().last().unwrap();
            let number = (first_num.to_string() + &last_num.to_string())
                .parse::<i32>()
                .unwrap();
            number
        })
        .collect();
    numbers.iter().sum()
}

fn part2(lines: Vec<String>) -> i32 {
    println!("start part 2");
    let string_to_digit: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();
    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| {
            let mut number_string: String = String::new();
            let mut i: usize = 0;
            while i < line.len() {
                if line.chars().nth(i).unwrap().is_numeric() {
                    number_string.push(line.chars().nth(i).unwrap());
                    i += 1;
                } else {
                    let mut matched = false;
                    for sub_count in 3..6 {
                        if i + sub_count > line.len() {
                            break;
                        }
                        let sub_string = &line[i..i + sub_count];
                        if let Some((key, value)) = string_to_digit.get_key_value(sub_string) {
                            number_string.push_str(&value);
                            i = i + key.len() - 1;
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        i += 1;
                    }
                }
            }
            let first_num = number_string.chars().next().unwrap();
            let last_num = number_string.chars().last().unwrap();
            let number: i32 = (first_num.to_string() + &last_num.to_string())
                .parse::<i32>()
                .unwrap();
            number
        })
        .collect();

    // loop through the list of numbers and sum them
    return numbers.iter().sum();
}
