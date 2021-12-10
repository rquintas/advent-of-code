use std::io;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {

    // let opening_chars = "([{<";
    let closing_chars = ")]}>";

    let mut mapping: HashMap<char,char> = HashMap::new();
    mapping.insert(')', '(');
    mapping.insert(']', '[');
    mapping.insert('}', '{');
    mapping.insert('>', '<');

    let mut close_mapping: HashMap<char,char> = HashMap::new();
    close_mapping.insert('(', ')');
    close_mapping.insert('[', ']');
    close_mapping.insert('{', '}');
    close_mapping.insert('<', '>');

    let mut score_map: HashMap<char,u32> = HashMap::new();
    score_map.insert(')', 3);
    score_map.insert(']', 57);
    score_map.insert('}', 1197);
    score_map.insert('>', 25137);

    let mut score_map_2: HashMap<char,u64> = HashMap::new();
    score_map_2.insert(')', 1);
    score_map_2.insert(']', 2);
    score_map_2.insert('}', 3);
    score_map_2.insert('>', 4);

    let program: Vec<Vec<char>> = read_input();

    let mut score = 0;

    // Part 1
    for line in program.clone() {

        let mut stack: VecDeque<char> = VecDeque::new();

        println!("{:?}", line);

        for token in line {
            if closing_chars.contains(token) {
                if stack.back().unwrap() == mapping.get(&token).unwrap() {
                    stack.pop_back();
                } else {
                    println!("Corrupted line. Illegal char: {}", token);
                    score += score_map.get(&token).unwrap();
                    break;
                }
            } else {
                stack.push_back(token);
            }
        }
    }

    println!("Total score: {}", score);

    // Part 2
    let mut scores: Vec<u64> = vec![];

    for line in program {

        let mut stack: VecDeque<char> = VecDeque::new();

        println!("{:?}", line);

        for token in line {
            if closing_chars.contains(token) {
                if stack.back().unwrap() == mapping.get(&token).unwrap() {
                    stack.pop_back();
                } else {
                    println!("Corrupted line. Illegal char: {}. Skipping", token);
                    stack.clear();
                    break;
                }
            } else {
                stack.push_back(token);
            }
        }

        if ! stack.is_empty() {
            println!("Incomplete line: {:?}", stack);
            println!("Repairing line:");

            let mut score: u64 = 0;
            while ! stack.is_empty() {
                let token = stack.pop_back().unwrap();
                let closing_token = close_mapping.get(&token).unwrap();
                print!("{}", closing_token);

                score *= 5;
                score += score_map_2.get(closing_token).unwrap();

            }

            scores.push(score);
            println!("");
        }

    }

    scores.sort();
    
    println!("Total score autocomplete: {}", scores[scores.len()/2]);
}

fn read_input() -> Vec<Vec<char>> {
    let stdin = io::stdin();

    let mut results: Vec<Vec<char>> = vec![];

    let mut eof = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                // Split numbers.
                let split = buffer.trim().split("").filter(|x| x.len() == 1);
                let result: Vec<char> = split.map(|x| x.chars().nth(0).unwrap() ).collect();
                results.push(result);
            }
            _ => { panic!("Error reading line."); }
        }
    }

    return results;
}

