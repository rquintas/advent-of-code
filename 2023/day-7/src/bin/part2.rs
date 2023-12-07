use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: usize,
    class: String,
}

impl Eq for Hand {

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        let self_class = match self.class.as_str() {
            "five of a kind" => 0,
            "four of a kind" => 1,
            "full house" => 2,
            "three of a kind" => 3,
            "two pairs" => 4,
            "one pair" => 5,
            "highest-card" => 6,
            _ => panic!("unknown hand"),
        };

        let other_class = match other.class.as_str() {
            "five of a kind" => 0,
            "four of a kind" => 1,
            "full house" => 2,
            "three of a kind" => 3,
            "two pairs" => 4,
            "one pair" => 5,
            "highest-card" => 6,
            _ => panic!("unknown hand"),
        };

        if self_class != other_class {
            // reversed
            return other_class.cmp(&self_class);
        }

        let self_cards = self.cards.chars().collect::<Vec<char>>();
        let other_cards = other.cards.chars().collect::<Vec<char>>();

        for i in 0..self_cards.len() {
            let self_card = self_cards[i];
            let other_card = other_cards[i];

            let self_card_value = match self_card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => self_card.to_digit(10).unwrap(),
            };

            let other_card_value = match other_card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => other_card.to_digit(10).unwrap(),
            };

            if self_card_value != other_card_value {
                return self_card_value.cmp(&other_card_value);
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }    
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

fn classify(cards: &str) -> String {

    let mut counter = HashMap::new();
    let mut jokers = 0;
    for card in cards.chars() {

        if card == 'J' {
            jokers += 1;
            continue;
        }

        counter.entry(card).or_insert(0);
        *counter.get_mut(&card).unwrap() += 1;
    }

    let mut values = counter.values().map(|v| *v).collect::<Vec<u32>>();
    values.sort();
    values.reverse();

    if values.len() == 0 {
        // 5 jokers
        return "five of a kind".to_string();
    } else {
        // add jokers to highest value
        values[0] += jokers;
    }

    return match values[..] {
        [5] => "five of a kind".to_string(),
        [4, 1] => "four of a kind".to_string(),
        [3, 2] => "full house".to_string(),
        [3, 1, 1] => "three of a kind".to_string(),
        [2, 2, 1] => "two pairs".to_string(),
        [2, 1, 1, 1] => "one pair".to_string(),
        [1, 1, 1, 1, 1] => "highest-card".to_string(),
        _ => panic!("unknown hand"),
    };
}

fn part2(input: &str) -> String {

    let line_iter = input.lines();
    let mut hands = Vec::new();

    for line in line_iter {
        let values = line.split(" ").collect::<Vec<&str>>();
        
        hands.push(Hand {
            cards: values[0].to_string(),
            bet: values[1].parse::<usize>().unwrap(),
            class: classify(values[0]),
        });
    }
    
    hands.sort();


    let mut total_winnings = 0;
    for i in 0..hands.len() {
        let hand = &hands[i];
        total_winnings += hand.bet * (i+1);
    }

    dbg!(&hands);

    total_winnings.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
");
        assert_eq!(result, "5905".to_string());
    }
}