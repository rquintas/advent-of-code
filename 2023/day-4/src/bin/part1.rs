fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

    let mut sum = 0;

    for line in input.lines() {
        println!("{}", line);
        let entries = line.split(":").nth(1).unwrap();
        let parts = entries.split(" | ").collect::<Vec<&str>>();

        let winners = parts[0];
        let winners_set = winners.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();

        let numbers = parts[1];
        let numbers_set = numbers.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();

        println!("{:?}", winners_set);
        println!("{:?}", numbers_set);

        let mut points = 0;
        for number in numbers_set {
            if winners_set.contains(&number) {
                if points == 0 {
                    points += 1;
                } else {
                    points = points*2;
                }
            }
        }
        println!("{}", points);

        sum += points;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");
        assert_eq!(result, "13".to_string());
    }
}