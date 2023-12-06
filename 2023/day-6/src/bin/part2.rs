use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        delta = 0.0;
    }
    x1 = ( -b + f64::sqrt(delta) ) / ( 2.0 * a );
    x2 = ( -b - f64::sqrt(delta) ) / ( 2.0 * a );
    (x1, x2)
}

fn part2(input: &str) -> String {
    let mut line_iter = input.lines();

    let times_line = line_iter.next().unwrap();
    let time = times_line.split(":").nth(1).unwrap().split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>().join("").parse::<f64>().unwrap();

    let distance_line = line_iter.next().unwrap();
    let record = distance_line.split(":").nth(1).unwrap().split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>().join("").parse::<f64>().unwrap();

    let (x1, x2) = quadratic(-1.0, time, -record);

    let lower_bound = f64::ceil(x1);
    let upper_bound = f64::floor(x2);

    let result = ( upper_bound - lower_bound + 1.0 ) as u64;

    println!("result: {}", result);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("Time:      7  15   30
Distance:  9  40  200     
");
        assert_eq!(result, "71503".to_string());
    }
}