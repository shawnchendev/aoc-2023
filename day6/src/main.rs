use std::io;

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

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_input(line: &Vec<String>) -> Vec<Race> {
    let time = line.iter().next().unwrap();
    let disance = line.iter().last().unwrap();
    let mut races = Vec::new();
    let time_arr = parse_line(time, "Time: ");
    let distance_arr = parse_line(disance, "Distance: ");

    for (i, t) in time_arr.iter().enumerate() {
        races.push(Race {
            time: *t,
            distance: distance_arr[i],
        })
    }

    races
}

fn parse_input_p2(line: &Vec<String>) -> Race {
    let time = line.iter().next().unwrap();
    let disance = line.iter().last().unwrap();
    let t = parse_line_as_num(time, "Time: ");
    let d = parse_line_as_num(disance, "Distance: ");

    Race {
        time: t,
        distance: d,
    }
}

fn parse_line(line: &str, trim: &str) -> Vec<i64> {
    return line
        .trim_start_matches(trim)
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s: &str| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

fn parse_line_as_num(line: &str, trim: &str) -> i64 {
    let line_num = line
        .trim_start_matches(trim)
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<String>();
    let num: i64 = match line_num.parse() {
        Ok(p) => p,
        Err(_) => 0,
    };
    num
}

fn calculate_strategies(races: &Vec<Race>) -> i32 {
    let mut total = <Vec<i32>>::new();
    for race in races {
        let count = get_win_count(race);
        total.push(count as i32);
    }
    total.iter().product()
}

fn get_win_count(race: &Race) -> usize {
    let mut win_num = Vec::new();
    for i in 1..race.time {
        let remining_time = race.time - i;
        let distance_travel = i * remining_time;
        win_num.push(distance_travel > race.distance);
    }
    let count = win_num.iter().filter(|b| **b).count();
    count
}
fn part1(line: Vec<String>) -> i32 {
    let races = parse_input(&line);
    let total = calculate_strategies(&races);
    total
}

fn part2(line: Vec<String>) -> i64 {
    let race = parse_input_p2(&line);
    get_win_count(&race) as i64
}
