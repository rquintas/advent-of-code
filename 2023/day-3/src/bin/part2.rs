fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Debug)]
struct Identifier {
    value: i32,
    row: i32,
    start: i32,
    end: i32,
    valid: bool,
}

#[derive(Clone, Debug)]
struct Symbol {
    value: char,
    row: i32,
    start: i32,
    end: i32,
}

fn part2(input: &str) -> String {

    let mut line_number = 0;
    let mut identifiers: Vec<Identifier> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for line in input.lines() {
        let mut char_number = 0;

        let mut reading_digit = false;
        let mut new_id = Identifier{value: 0, row: 0, start: 0, end: 0, valid: false};

        for c in line.chars() {
            match c {
                '0'..='9' => {
                    if !reading_digit {
                        new_id.row = line_number;
                        new_id.start = char_number;
                    }
                    reading_digit = true;

                    new_id.value = new_id.value*10 + c.to_digit(10).unwrap() as i32;     
                },
                '.' => {

                    if reading_digit {
                        new_id.end = char_number;
                        print!("{:?}\n",new_id);
                        identifiers.push(new_id);
                        new_id = Identifier{value: 0, row: 0, start: 0, end: 0, valid: false};
                    }

                    reading_digit = false;
                },
                s => {
                    if reading_digit { 
                        new_id.end = char_number;
                        print!("{:?}\n",new_id);
                        identifiers.push(new_id);
                        new_id = Identifier{value: 0, row: 0, start: 0, end: 0, valid: false};
                    }

                    reading_digit = false;
                    print!("{}",s);

                    symbols.push(Symbol{value:s, row: line_number, start: char_number, end: char_number+1})
                }
            }
            char_number += 1;
        }

        if reading_digit { 
            new_id.end = char_number;
            print!("{:?}\n",new_id);
            identifiers.push(new_id);
        }

        line_number += 1;
    }

    let mut gear_ratio = 0;

    for symbol in &symbols {

        // skip if not a possible gear
        if symbol.value != '*' {
            continue;
        }

        let mut adjacent_pieces: Vec<i32> = vec![];

        for mut identifier in &mut identifiers {
            // above
            if identifier.row == symbol.row - 1 {
                if identifier.start-1 <= symbol.start && identifier.end+1 >= symbol.end {
                    identifier.valid = true;
                    adjacent_pieces.push(identifier.value);
                }
            }
            //below
            if identifier.row == symbol.row + 1 {
                if identifier.start-1 <= symbol.start && identifier.end+1 >= symbol.end {
                    identifier.valid = true;
                    adjacent_pieces.push(identifier.value);
                }
            }
            //adjacent on same row 
            if identifier.row == symbol.row {
                if identifier.start == symbol.end {
                    identifier.valid = true;
                    adjacent_pieces.push(identifier.value);
                }
                if identifier.end == symbol.start {
                    identifier.valid = true;
                    adjacent_pieces.push(identifier.value);
                }
            }
        }

        // is a gear
        if adjacent_pieces.len() == 2 {
            let mut acc = 1;
            for value in adjacent_pieces {
                acc *= value;
            }
            gear_ratio += acc;
        }
    }

    return gear_ratio.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");
        assert_eq!(result, "467835".to_string());
    }
}