use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Insert {
    value: char,
    index: usize
}

fn main() {

    // Read Input 
    let mut template: String = String::new();
    let mut grammar: HashMap<char, HashMap<char, char>> = HashMap::new();
    let mut grammar2: HashMap<String, char> = HashMap::new();

    let stdin = io::stdin();
    let mut eof = false;
    let mut read_grammar = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(1) => {
                read_grammar = true;
            }
            Ok(_) => {           
                
                if ! read_grammar {
                    // Split numbers.
                    template = buffer.trim().to_string();

                } else {
                    let split: Vec<char> = buffer.trim().replace(" -> ", "").chars().collect();
                    grammar.entry(split[0]).or_default().insert(split[1], split[2]);
                    grammar2.insert(format!("{}{}", split[0], split[1]), split[2]);
                }
                
            },
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    println!("Template: {}", template);
    println!("Grammar: {:?}", grammar);

    // Part 1
    // part1(template, &grammar);

    // Part 2
    part2(template, &grammar2);

}

// Part 2: Transition table counts.
fn part2(template: String, grammar: &HashMap<String, char>) {

    let mut counter: HashMap<char, i64> = HashMap::new();
    let mut transitions: HashMap<String, i64> = HashMap::new();

    // Initialization
    let mut previous: char = ' ';
    for c in template.chars() {
        *counter.entry(c).or_default() += 1;

        if previous != ' ' {
            *transitions.entry(format!("{}{}", previous, c)).or_default() += 1;
        }

        previous = c;
    }

    println!("{:?}", transitions);

    for s in 1..=40 {

        let mut insertions: HashMap<String, i64> = HashMap::new();

        for rule in grammar.keys() {
            let generated: char = *grammar.get(rule).unwrap();

            if transitions.contains_key(rule) {
                let number_of_transitions: i64 = *transitions.get(rule).unwrap();

                // Increase counter for character
                *counter.entry(generated).or_default() += number_of_transitions;

                // Decrease transition count, as they are splitted by new entries
                *insertions.entry(rule.to_string()).or_default() -= number_of_transitions;

                // Transitions to append.
                let first_char = rule.chars().nth(0).unwrap();
                let second_char = rule.chars().nth(1).unwrap();
                *insertions.entry(format!("{}{}", first_char, generated)).or_default() += number_of_transitions;
                *insertions.entry(format!("{}{}", generated, second_char)).or_default() += number_of_transitions;
            }
        }

        for insertion in insertions.keys() {
            *transitions.entry(insertion.to_string()).or_default() += insertions.get(insertion).unwrap();
        }

        println!("Step {}", s);
        println!("{:?}", transitions);
        println!("{:?}", counter);

    }



    let result = counter.values().max().unwrap() - counter.values().min().unwrap();

    println!("Final Score: {}" , result);
}

// Part 1: Naive String expansion.
fn part1(template: String, grammar: &HashMap<char, HashMap<char, char>>) {
    let mut new_string = step(template, &grammar);

    println!("After step {}: {:?}", 1, new_string);
     
    for s in 2..=40 {
        new_string = step(new_string, &grammar);
        println!("After step {}", s);
    }
    println!("Final Score: {}" , score(new_string));
}

fn step(template: String, grammar: &HashMap<char, HashMap<char, char>>) -> String {
    let mut previous: char = ' ';
    let mut index = 0;

    let mut insertions: Vec<Insert> = vec![];

    for current in template.chars() {

        match grammar.get(&previous) {
            Some(entry) => match entry.get(&current) {
                Some(generated) => {
                    insertions.push(Insert{ value: *generated, index: index + insertions.len() })
                },
                _ => ()
            } ,
            _ => ()
        }

        previous = current;
        index += 1;
    }

    let mut new_string: String = template.clone();
    for insertion in &insertions {
        new_string.insert(insertion.index, insertion.value);
    }

    return new_string;
}

fn score(polymer: String) -> i64 {

    let mut count: HashMap<char, i64> = HashMap::new();

    for c in polymer.chars() {
        *count.entry(c).or_default() += 1;
    }

    println!("{:?}", count);

    let result = count.values().max().unwrap() - count.values().min().unwrap();

    return result;
}