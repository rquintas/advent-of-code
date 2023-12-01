fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Debug)]
struct Number {
    value: i32,
    pos: i32,
}

fn find_first_number(line: &str) -> Number {
    let mut pos = 0;
    for c in line.chars() {
        match c {
            '0'..='9' => {
                return Number {
                    value: c.to_digit(10).unwrap() as i32,
                    pos: pos,
                };
            },           
            _ => {},
        }
        pos += 1;
    }

    return Number {
        value: -1,
        pos: i32::MAX,
    };
}

fn find_last_number(line: &str) -> Number {
    let mut pos = 0;
    
    let mut last_number = Number {
        value: -1,
        pos: -1,
    };

    for c in line.chars() {
        match c {
            '0'..='9' => {
                last_number = Number {
                    value: c.to_digit(10).unwrap() as i32,
                    pos: pos,
                };
            },           
            _ => {},
        }
        pos += 1;
    }

    return last_number;
}

fn find_string_matches(line: &str) -> Vec<Number> {
    let mut NUMBERS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut first_match = Number {
        value: -1,
        pos: i32::MAX,
    };

    let mut last_match = Number {
        value: -1,
        pos: -1,
    };

    for number in NUMBERS.iter() {
        let re = regex::Regex::new(number).unwrap();
        for m in re.find_iter(line) {

            let number = match m.as_str() {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => "",
            };

            let start = m.start() as i32;
            let end = m.end() as i32;

            if start < first_match.pos {
                first_match = Number {
                    value: number.parse::<i32>().unwrap(),
                    pos: start as i32,
                };
            }

            if start > last_match.pos {
                last_match = Number {
                    value: number.parse::<i32>().unwrap(),
                    pos: start as i32,
                };
            }
        }
    }

    return [first_match, last_match].to_vec();
}

fn part2(input: &str) -> String {
    let mut first_digit: i32 = -1;
    let mut current_digit: i32 = -1;
 
    let mut sum = 0;

    let mut copy = String::from(input);
    for line in copy.lines() {
        let first_number = find_first_number(line);
        let last_number = find_last_number(line);
        
        let mut first_digit = first_number.value;
        let mut last_digit = last_number.value;

        let string_matches = find_string_matches(line);

        if string_matches[0].pos < first_number.pos {
            first_digit = string_matches[0].value;
        }

        if string_matches[1].pos > last_number.pos {
            last_digit = string_matches[1].value;
        }
        
        let value = first_digit * 10 + last_digit;

        sum += value;
    }


    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        assert_eq!(result, "281".to_string());
    }
}