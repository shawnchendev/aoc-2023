use file_reader::read_from_files;
use std::io;

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

fn parse_input(lines: Vec<String>) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|line| line.chars().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn symbol_finder(grid: &Vec<Vec<String>>) -> Vec<(i32, i32)> {
    let mut symbols: Vec<(i32, i32)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            if symbol != "." && !symbol.chars().next().unwrap().is_numeric() {
                symbols.push((i as i32, j as i32));
            }
        }
    }
    symbols
}

fn gear_finder(grid: &Vec<Vec<String>>) -> Vec<(i32, i32)> {
    let mut symbols: Vec<(i32, i32)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            if symbol == "*" {
                symbols.push((i as i32, j as i32));
            }
        }
    }
    symbols
}

fn find_near_by_numbers(grid: &Vec<Vec<String>>, symbols: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut near_by_numbers: Vec<(i32, i32)> = Vec::new();
    let moves = [
        (0, 1),   // right
        (1, 0),   // down
        (0, -1),  // left
        (-1, 0),  // up
        (1, 1),   // down right
        (-1, -1), // up left
        (-1, 1),  // up right
        (1, -1),  // down left
    ];

    for (i, j) in symbols {
        for (x, y) in &moves {
            let new_i = i + x;
            let new_j = j + y;
            if new_i >= 0 && new_j >= 0 {
                if let Some(row) = grid.get(new_i as usize) {
                    if let Some(symbol) = row.get(new_j as usize) {
                        if symbol.chars().next().unwrap().is_numeric() {
                            near_by_numbers.push((new_i as i32, new_j as i32));
                        }
                    }
                }
            }
        }
    }
    near_by_numbers
}

fn find_near_by_gear_numbers(
    grid: &Vec<Vec<String>>,
    symbols: &Vec<(i32, i32)>,
) -> Vec<Vec<(i32, i32)>> {
    let mut near_by_numbers_all: Vec<Vec<(i32, i32)>> = Vec::new();
    let moves = [
        (0, 1),   // right
        (1, 0),   // down
        (0, -1),  // left
        (-1, 0),  // up
        (1, 1),   // down right
        (-1, -1), // up left
        (-1, 1),  // up right
        (1, -1),  // down left
    ];

    for (i, j) in symbols {
        let mut near_by_numbers: Vec<(i32, i32)> = Vec::new();

        for (x, y) in &moves {
            let new_i = i + x;
            let new_j = j + y;
            if new_i >= 0 && new_j >= 0 {
                if let Some(row) = grid.get(new_i as usize) {
                    if let Some(symbol) = row.get(new_j as usize) {
                        if symbol.chars().next().unwrap().is_numeric() {
                            near_by_numbers.push((new_i as i32, new_j as i32));
                        }
                    }
                }
            }
        }
        near_by_numbers_all.push(near_by_numbers);
    }
    near_by_numbers_all
}

fn get_full_number_by_position(
    number_cord: (i32, i32),
    grid: &Vec<Vec<String>>,
) -> (i32, Vec<(i32, i32)>) {
    let mut number = grid[number_cord.0 as usize][number_cord.1 as usize].to_string();
    let mut left = number_cord.1 - 1;
    let mut right = number_cord.1 + 1;
    let mut visited_cords: Vec<(i32, i32)> = Vec::new();
    visited_cords.push(number_cord);
    // find the other digit by moving left and right from the current position and combine it into a number
    while left >= 0
        && grid[number_cord.0 as usize][left as usize]
            .chars()
            .next()
            .unwrap()
            .is_numeric()
    {
        number = grid[number_cord.0 as usize][left as usize].to_string() + &number;
        visited_cords.push((number_cord.0, left));
        left -= 1;
    }
    while right < grid[number_cord.0 as usize].len() as i32
        && grid[number_cord.0 as usize][right as usize]
            .chars()
            .next()
            .unwrap()
            .is_numeric()
    {
        number = number + &grid[number_cord.0 as usize][right as usize].to_string();
        visited_cords.push((number_cord.0, right));
        right += 1;
    }
    (number.parse::<i32>().unwrap(), visited_cords)
}

fn part1(lines: Vec<String>) -> i32 {
    let grid = parse_input(lines);
    let symbols = symbol_finder(&grid);

    let nearbys: Vec<(i32, i32)> = find_near_by_numbers(&grid, &symbols);
    let mut all_visited_cords: Vec<(i32, i32)> = Vec::new();
    let numbers = nearbys
        .iter()
        .map(|(i, j)| {
            let contains_coordinate = all_visited_cords.contains(&(*i, *j));
            if contains_coordinate {
                return 0;
            }
            let (number, visited_cord) = get_full_number_by_position((*i, *j), &grid);
            all_visited_cords.extend(visited_cord);
            number
        })
        .collect::<Vec<i32>>();
    numbers.iter().sum()
}

fn part2(lines: Vec<String>) -> i32 {
    let grid = parse_input(lines);
    let gears = gear_finder(&grid);
    let nearbys: Vec<Vec<(i32, i32)>> = find_near_by_gear_numbers(&grid, &gears);
    let mut total_gear_ratios: Vec<i32> = Vec::new();
    for near_by in nearbys {
        let mut all_visited_cords: Vec<(i32, i32)> = Vec::new();
        let numbers: Vec<i32> = near_by
            .iter()
            .filter_map(|(i, j)| {
                let contains_coordinate = all_visited_cords.contains(&(*i, *j));
                if contains_coordinate {
                    None // Filter out elements with a value of 0
                } else {
                    let (number, visited_cord) = get_full_number_by_position((*i, *j), &grid);
                    all_visited_cords.extend(visited_cord);
                    Some(number)
                }
            })
            .collect();

        let mut gear_ratios = 1;
        if numbers.len() == 1 {
            continue;
        }
        for number in numbers {
            gear_ratios = gear_ratios * number;
        }
        total_gear_ratios.push(gear_ratios);
    }
    total_gear_ratios.iter().sum()
}
