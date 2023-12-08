use file_reader::read_from_files;
use std::{cmp::max, collections::HashMap, io};

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

#[derive(Debug)]
struct GameInfo {
    game_id: usize,
    draws: Vec<Vec<(String, i32)>>,
}

impl GameInfo {
    // Calculate the total count for each color within each draw
    fn calculate_color_totals(&self) -> Vec<HashMap<&str, i32>> {
        self.draws
            .iter()
            .map(|draw| {
                let mut color_totals: HashMap<&str, i32> = HashMap::new();
                for (color, count) in draw.iter() {
                    let entry = color_totals.entry(color).or_insert(0);
                    *entry += count;
                }
                color_totals
            })
            .collect()
    }

    fn compare_draw_with_total(&self) -> bool {
        let cubes = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, i32>>();
        let mut is_valid_draw: Vec<bool> = Vec::new();
        let color_totals = self.calculate_color_totals();
        color_totals
            .into_iter()
            .flat_map(|total| total)
            .for_each(|(color, count)| {
                if let Some(total_count) = cubes.get(color) {
                    is_valid_draw.push(count <= *total_count);
                }
            });
        is_valid_draw.iter().all(|&x| x)
    }

    fn find_min_cube_required(&self) -> HashMap<&str, i32> {
        let mut cubes = [("red", 0), ("green", 0), ("blue", 0)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, i32>>();
        let color_totals = self.calculate_color_totals();
        color_totals
            .into_iter()
            .flat_map(|total| total)
            .for_each(|(color, count)| {
                if let Some(current_count) = cubes.get_mut(color) {
                    *current_count = max(*current_count, count);
                }
            });

        cubes
    }
}

fn parse_input(input: &str) -> GameInfo {
    let parts: Vec<&str> = input.split(':').map(|s| s.trim()).collect();
    let game_id: usize = parts[0].trim_start_matches("Game ").trim().parse().unwrap();

    // Parse draws
    let draws = parts[1]
        .split(';')
        .map(|draw_str| {
            return draw_str
                .split(',')
                .map(|pair| {
                    let mut iter = pair.trim().split_whitespace();
                    let count: i32 = iter.next().unwrap().parse().unwrap();
                    let color = iter.next().unwrap().to_string();
                    (color, count)
                })
                .collect();
        })
        .collect();

    GameInfo { game_id, draws }
}

fn part1(lines: Vec<String>) -> i32 {
    let nums: Vec<i32> = lines
        .iter()
        .map(|line| {
            let game_info = parse_input(line);
            let is_valid_draw = game_info.compare_draw_with_total();

            if is_valid_draw {
                game_info.game_id as i32
            } else {
                0
            }
        })
        .collect();
    println!("{:?}", nums);
    return nums.iter().sum();
}

fn part2(lines: Vec<String>) -> i32 {
    let nums: Vec<i32> = lines
        .iter()
        .map(|line| {
            let game_info = parse_input(line);
            let max_draw = game_info.find_min_cube_required();
            let mut total = 1;
            for (_, count) in max_draw {
                total = total * count
            }
            total
        })
        .collect();
    println!("{:?}", nums);
    return nums.iter().sum();
}
