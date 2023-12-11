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

#[derive(Debug)]
struct Farm {
    seeds: Vec<i64>,
    seed_to_locations: HashMap<String, Vec<SourceToTarget>>,
}
impl Farm {
    fn seeds_location_finder(&self, seeds: &Vec<i64>) -> Vec<i64> {
        let keys = [
            "seed-to-soil map:",
            "soil-to-fertilizer map:",
            "fertilizer-to-water map:",
            "water-to-light map:",
            "light-to-temperature map:",
            "temperature-to-humidity map:",
            "humidity-to-location map:",
        ];
        let mut seeds_location = Vec::new();
        for seed in seeds {
            let mut source = *seed;
            for where_to_find in keys {
                source = self.get_target_from_source(source, where_to_find);
            }
            seeds_location.push(source as i64);
        }
        seeds_location
    }

    fn seed_pairs_location_finder(&self) -> Vec<i64> {
        let seeds = &self
            .seeds
            .chunks_exact(2)
            .map(|chunk| chunk.to_vec())
            .flat_map(|range| {
                let start = range[0];
                let len = range[1];
                start..start + len
            })
            .collect();
        let seeds_location = self.seeds_location_finder(seeds);

        seeds_location
    }

    fn get_target_from_source(&self, source: i64, where_to_find: &str) -> i64 {
        let source_to_target = self.seed_to_locations.get(where_to_find).unwrap();
        let mut target = source;
        for stt in source_to_target {
            if let Some(target_temp) = stt.get_target_from_source(source) {
                target = target_temp;
                break;
            }
        }
        target
    }
}

#[derive(Debug)]
struct SourceToTarget {
    source: i64,
    target: i64,
    range: i64,
}

impl SourceToTarget {
    fn get_target_from_source(&self, source: i64) -> Option<i64> {
        let max_source = self.source + self.range;
        if (self.source..=max_source).contains(&source) {
            let diff = source - self.source;
            Some(self.target + diff)
        } else {
            None
        }
    }
}

fn parse_input(lines: &Vec<String>) -> Farm {
    // split the lines into an array of string by the empty line
    let mut result = lines
        .split(|v| v.is_empty())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_vec())
        .collect::<Vec<_>>();

    // get all the seeds from the first line and convert them into a vector of i64
    let seeds = result
        .get(0)
        .and_then(|element| element.get(0))
        .map(|fe| {
            fe.trim_start_matches("seeds: ")
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap();
    // remove the first line
    result.remove(0);

    let mut seed_to_locations = <HashMap<String, Vec<SourceToTarget>>>::new();
    for r in result {
        let first_element = r.get(0).unwrap();
        let mut source_target_arr = <Vec<SourceToTarget>>::new();
        for i in 1..r.len() {
            let r_e = r
                .get(i)
                .map(|element| {
                    let arr = element
                        .split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    SourceToTarget {
                        source: arr[1],
                        target: arr[0],
                        range: arr[2],
                    }
                })
                .unwrap();
            source_target_arr.push(r_e);
        }
        seed_to_locations.insert(first_element.to_string(), source_target_arr);
    }

    Farm {
        seeds,
        seed_to_locations,
    }
}

fn part1(lines: Vec<String>) -> i64 {
    let farm = parse_input(&lines);
    let seeds_location = farm.seeds_location_finder(&farm.seeds);
    seeds_location.into_iter().min().unwrap()
}

fn part2(lines: Vec<String>) -> i64 {
    let farm = parse_input(&lines);
    let seeds_location = farm.seed_pairs_location_finder();
    seeds_location.into_iter().min().unwrap()
}
