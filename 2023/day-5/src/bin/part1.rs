use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn apply_mapping(ranges: &Vec<Vec<u64>>, seed: u64) -> u64 {
    let mut mapping = seed;
    for range in ranges {
        if range[1] <= seed && seed <= range[1] + range[2] {
            mapping = range[0] + (seed - range[1]);
            break;
        } 
    }
    mapping
}

fn read_ranges(line_iter: &mut std::str::Lines) -> Vec<Vec<u64>> {
    let mut all_ranges: Vec<Vec<u64>> = Vec::new();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let ranges = line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        all_ranges.push(ranges);
    }
    all_ranges
}

fn part1(input: &str) -> String {

    let mut seeds: Vec<u64> = Vec::new();
    let mut ranges_map: HashMap<&str, Vec<Vec<u64>>> = HashMap::new();

    let mut line_iter = input.lines();
    while true {
        let line = line_iter.next();

        match line {
            Some(line) => {
                match line {
                    line if line.starts_with("seeds: ") => {
                        println!("seeds: {}", line);
                        let seeds_str = line.split(": ").nth(1).unwrap();
                        seeds = seeds_str.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
                    },
                    line if line.starts_with("seed-to-soil map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("seed-to-soil", all_ranges);
                    },
                    line if line.starts_with("soil-to-fertilizer map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("soil-to-fertilizer", all_ranges);
                    },
                    line if line.starts_with("fertilizer-to-water map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("fertilizer-to-water", all_ranges);
                    },
                    line if line.starts_with("water-to-light map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("water-to-light", all_ranges);
                    },
                    line if line.starts_with("light-to-temperature map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("light-to-temperature", all_ranges);
                    },
                    line if line.starts_with("temperature-to-humidity map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("temperature-to-humidity", all_ranges);
                    },
                    line if line.starts_with("humidity-to-location map:") => {
                        let all_ranges = read_ranges(&mut line_iter);
                        ranges_map.insert("humidity-to-location", all_ranges);
                    },
                    &_ => {},
                }
            },
            None => {
                break;
            }
        }

    }

    let mut min_location = u64::MAX;

    for seed in seeds {
        let mut mapping = apply_mapping(&ranges_map["seed-to-soil"], seed);
        // println!("seed: {}, soil: {}", seed, mapping);
        mapping = apply_mapping(&ranges_map["soil-to-fertilizer"], mapping);
        // println!("seed: {}, fertilizer: {}", seed, mapping);
        mapping = apply_mapping(&ranges_map["fertilizer-to-water"], mapping);
        // println!("seed: {}, water: {}", seed, mapping);
        mapping = apply_mapping(&ranges_map["water-to-light"], mapping);
        // println!("seed: {}, light: {}", seed, mapping);
        mapping = apply_mapping(&ranges_map["light-to-temperature"], mapping);
        // println!("seed: {}, temperature: {}", seed, mapping);
        mapping = apply_mapping(&ranges_map["temperature-to-humidity"], mapping);
        // println!("seed: {}, humidity: {}", seed, mapping);
        mapping = apply_mapping(&ranges_map["humidity-to-location"], mapping);
        println!("seed: {}, location: {}", seed, mapping);

        if mapping < min_location {
            min_location = mapping;
        }
    }

    return min_location.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
");
        assert_eq!(result, "35".to_string());
    }
}