fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_diffs(numbers: &[i32]) -> Vec<i32> {

    let mut diffs = vec![];

    for (i, number) in numbers.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let diff = number - numbers[i - 1];
        diffs.push(diff);
    }

    diffs
}

fn check_zeros(diffs: &[i32]) -> bool {
    for diff in diffs {
        if *diff != 0 {
            return false;
        }
    }
    true
}

fn solve(input: &str) -> i32 {

    let numbers = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut all_diffs = vec![];
    all_diffs.push(numbers.clone());
    

    let mut diffs = get_diffs(&numbers);
    all_diffs.push(diffs.clone());

    while !check_zeros(&diffs) {
        diffs = get_diffs(&diffs);
        all_diffs.push(diffs.clone());
    }

    let mut last_value = 0;

    // reverse iter
    for i in (0..all_diffs.len()).rev() {
        let diffs = &all_diffs[i];
        last_value = diffs[diffs.len()-1] + last_value;
    }

    last_value
}

fn part1(input: &str) -> String {

    let mut sum = 0;
    for line in input.lines() {
        sum += solve(line);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
");
        assert_eq!(result, "114".to_string());
    }    
}