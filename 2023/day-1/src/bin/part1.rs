fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut first_digit: i32 = -1;
    let mut current_digit: i32 = -1;

    let mut sum = 0;
    // iterate each character
    for c in input.chars() {
        match c {
            '0'..='9' => {
                if first_digit == -1 {
                    first_digit = c.to_digit(10).unwrap() as i32;
                }
                current_digit = c.to_digit(10).unwrap() as i32;                
            },
            '\n' => {
                let value = first_digit * 10 + current_digit;
                print!("value: {}\n", value);

                first_digit = -1;   
                current_digit = -1;

                sum += value;
            },
            _ => {},
        }

    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
");
        assert_eq!(result, "142".to_string());
    }
}