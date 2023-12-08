use std::{collections::HashMap, io};

use file_reader::read_from_files;

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

#[derive(Debug, Clone)]
struct CardInfo {
    card_id: usize,
    winning_numbers: Vec<i32>,
    betting_numbers: Vec<i32>,
}
impl CardInfo {
    fn get_num_of_winning_numbers(&self) -> usize {
        return self
            .betting_numbers
            .iter()
            .filter_map(|bn| -> Option<i32> {
                if self.winning_numbers.contains(&bn) {
                    Some(*bn)
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>()
            .len();
    }
}

fn parse_input(line: String) -> CardInfo {
    let split_line: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
    let card_id: usize = split_line[0]
        .trim_start_matches("Card ")
        .trim()
        .parse()
        .unwrap();

    let numbers = split_line[1].split(" | ").collect::<Vec<&str>>();

    let winning_numbers = numbers[0]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let betting_numbers = numbers[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    CardInfo {
        card_id: card_id,
        winning_numbers,
        betting_numbers,
    }
}

fn part1(lines: Vec<String>) -> i32 {
    let global_total = lines
        .iter()
        .map(|line| {
            let card_info = parse_input(line.to_string());
            let numbers = card_info.get_num_of_winning_numbers();
            if numbers == 0 {
                return 0;
            }
            let base = 2;

            // Using pow method for integers
            let total = i32::pow(base, numbers as u32 - 1);
            total
        })
        .sum::<i32>();

    global_total
}
fn store_card_info(lines: &Vec<String>) -> HashMap<usize, CardInfo> {
    let mut card_db = <HashMap<usize, CardInfo>>::new();
    for line in lines {
        let card_info = parse_input(line.to_string());
        card_db.insert(card_info.card_id, card_info);
    }
    card_db
}

fn part2(lines: Vec<String>) -> i32 {
    let mut all_totals = 0;
    let mut count_db = HashMap::<usize, usize>::new();
    let mut card_db = store_card_info(&lines);
    let mut all_cards = lines
        .iter()
        .map(|line| parse_input(line.to_string()))
        .collect::<Vec<CardInfo>>();
    while !all_cards.is_empty() {
        if let Some(first_element) = all_cards.pop() {
            all_totals += 1;
            let value = count_db
                .entry(first_element.card_id)
                .or_insert_with(|| first_element.get_num_of_winning_numbers());
            for i in 0..*value {
                let index = first_element.card_id + i + 1;
                if let Some(card) = card_db.get_mut(&index) {
                    all_cards.push(card.clone());
                }
            }
        }
    }
    return all_totals;
}
