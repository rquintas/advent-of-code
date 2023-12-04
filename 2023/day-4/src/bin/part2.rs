fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {

    let mut cards_list: Vec<[Vec<&str>;2]> = vec![];

    for line in input.lines() {
        println!("{}", line);

        let mut card = line.split(":");
        
        let mut id = card.nth(0).unwrap();
        id = &id.replace("Card ", "");

        let entries = card.nth(0).unwrap();

        let parts = entries.split(" | ").collect::<Vec<&str>>();

        let winners = parts[0];
        let winners_set = winners.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();

        let numbers = parts[1];
        let numbers_set = numbers.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();

        cards_list.push([winners_set.clone(), numbers_set.clone()]);
    }

    let mut frontier: Vec<usize> = vec![];
    for i in 0..cards_list.len() {
        frontier.push(i);
    }
    

    let mut iterations = 0;
    while frontier.len() > 0 {

        let elem = frontier.pop().unwrap();

        let card = &cards_list[elem];
        let winners_set = &card[0];
        let numbers_set = &card[1];

        let mut matches = 1;
        for number in numbers_set {
            if winners_set.contains(&number) {
                if elem+matches < cards_list.len() {
                    frontier.push(elem+matches);
                    matches += 1;
                }
            }
        }
        iterations += 1;
    }

    iterations.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");
        assert_eq!(result, "30".to_string());
    }
}