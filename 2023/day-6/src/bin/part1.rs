use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {


    let mut line_iter = input.lines();

    let times_line = line_iter.next().unwrap();
    let times = times_line.split(":").nth(1).unwrap().split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let distance_line = line_iter.next().unwrap();
    let distance = distance_line.split(":").nth(1).unwrap().split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    println!("{:?}", times);
    println!("{:?}", distance);

    let mut result = 1;
    for i in 0..times.len() {
        let time = times[i];
        let record = distance[i];

        let mut counter = 0;
        for hold in 1..time {
            let distance = hold * ( time - hold );
            if distance > record {
                println!("hold: {}, distance: {}", hold, distance);
                counter += 1;
            }
            if counter > 1 && distance < record {
                // early stop
                break;
            }
        }

        if counter > 0 {
            result *= counter;
        }
        println!("result: {}", result);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("Time:      7  15   30
Distance:  9  40  200     
");
        assert_eq!(result, "288".to_string());
    }
}