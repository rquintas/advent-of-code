use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::collections::VecDeque;

fn hashset(data: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(data.iter().cloned())
}

fn update_mapping(mapping: &mut HashMap<char, HashSet<u8>>, value: &String, digit: &HashSet<u8> ) {
    for c in value.chars() {
        let set = &*mapping.get(&c).unwrap();
        let new_set = set.intersection(digit);
        mapping.insert(c, new_set.cloned().collect());
    }
}

fn get_overlap_count(value: &String, number: u8, mapping_chars: &HashMap<u8, String>) -> Option<u8> {
    if mapping_chars.contains_key(&number) {
        let repr = mapping_chars.get(&number).unwrap();

        // println!("{} {}", value, repr);

        let mut count_matches = 0;
        for c in value.chars() {
            if repr.contains(c) {
                count_matches += 1;
            }
        }
        return Some(count_matches);
    }

    return None;
}

fn overlap_rule(value: &String, source: u8, digit: &HashSet<u8>, number: u8, overlap: u8, mapping: &mut HashMap<char, HashSet<u8>>, mapping_chars: &mut HashMap<u8, String>, neg: bool) -> bool {
    match get_overlap_count(&value, number, &mapping_chars) {
        Some(x) => {
            if neg {
                if x != overlap {
                    return true;
                }
            } else {
                if x == overlap {
                    update_mapping(mapping, value, &digit);
                    mapping_chars.insert(source, value.to_string());
                    println!("Found {} {} against {} == {} => {}", source, value, x, overlap, number);
                    return true;
                }
            }

        },
        None => (),
    }
    return false;
}

fn main() {
    
    let results = read_input();

    let mut count = 0;

    let mut mapping: HashMap<char, HashSet<u8>>  = HashMap::new();
    let mut mapping_chars : HashMap<u8, String>  = HashMap::new();

    // Initialize mapping, every segment can be assigned to any letter.
    for c in "abcdefg".chars() {
        mapping.insert(c, (0..6).collect());
    }

    let digit_1 = hashset(&[2,5]);
    let digit_4 = hashset(&[0,2,5]);
    let digit_7 = hashset(&[1,2,3,5]);
    let digit_8 = hashset(&[0,1,2,3,4,5,6]);

    let digit_0_6_9 = hashset(&[0,1,2,3,4,5,6]);
    let digit_0 = hashset(&[0,1,2,4,5,6]);
    let digit_6 = hashset(&[0,1,3,4,5,6]);
    let digit_9 = hashset(&[0,1,2,3,5,6]);

    let digit_2_3_5 = hashset(&[0,1,2,3,4,5,6]);
    let digit_2 = hashset(&[0,2,3,4,6]);
    let digit_3 = hashset(&[0,2,3,5,6]);
    let digit_5 = hashset(&[0,1,3,5,6]);

    let mut full_sum = 0;

    for result in &results {

        let signal_patterns = &result[0];
        let input_values = &result[1];

        let mut queue = VecDeque::from_iter(signal_patterns);

        while !queue.is_empty() {
            let value = queue.pop_front().unwrap();

            match value.len() {
                2 => {
                    // 1
                    update_mapping(&mut mapping, value, &digit_1);
                    mapping_chars.insert(1, value.to_string());
                    println!("Found 1 {}", value);
                },
                3 => {
                    // 7
                    update_mapping(&mut mapping, value, &digit_7);
                    mapping_chars.insert(7, value.to_string());
                    println!("Found 7 {}", value);

                },
                4 => {
                    // 4
                    update_mapping(&mut mapping, value, &digit_4);
                    mapping_chars.insert(4, value.to_string());
                    println!("Found 4 {}", value);
                },
                7 => {
                    // 8
                    update_mapping(&mut mapping, value, &digit_8);
                    mapping_chars.insert(8, value.to_string());
                    println!("Found 8 {}", value);
                },
                5 => {
                    // 2,3,5
                    // if matches 1 => 3
                    if overlap_rule(value, 3, &digit_3, 1, 2, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    }

                    // if matches 7 => 3
                    if overlap_rule(value, 3, &digit_3, 7, 3, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    }

                    // if matches 4 - 1 => 5
                    if overlap_rule(value, 5, &digit_5, 4, 3, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    }

                    // else 2
                    if overlap_rule(value, 2, &digit_2, 4, 2, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    } 
                    
                    println!("Readding {}", value);
                    queue.push_back(value);
                },
                6 => {
                    // 0,6,9
                    if overlap_rule(value, 9, &digit_9, 4, 4, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    } 

                    if overlap_rule(value, 6, &digit_6, 1, 2, &mut mapping, &mut mapping_chars, true) 
                    {
                        if overlap_rule(value, 6, &digit_6, 4, 3, &mut mapping, &mut mapping_chars, false) {
                            continue;
                        }
                    } 

                    if overlap_rule(value, 0, &digit_0, 1, 2, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    } 
                    if overlap_rule(value, 0, &digit_0, 7, 3, &mut mapping, &mut mapping_chars, false) {
                        continue;
                    } 

                    println!("Readding {}", value);
                    queue.push_back(value);
                },
                _ => ()
            }

        }

        println!("{:?}", mapping);
        println!("{:?}", mapping_chars);

        // Part 1
        for value in input_values {

            decode(value, &mapping);

            match value.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => ()
            }
        }

        // Part 2
        let base = input_values.len() - 1;
        let mut base_count = 0;
        let mut converted_number = 0;

        for value in input_values {
            for (number, repr) in &mapping_chars {
                if repr.len() == value.len() {
                    if value.chars().all(|c| repr.contains(c)) {
                        converted_number += (*number as u32) * u32::pow(10, (base-base_count) as u32);
                        base_count += 1;

                        break;
                    }
                }
            }
        }
        println!("{}", converted_number);

        full_sum += converted_number;
    }

    println!("Total number of 1,4,7,8: {}", count);
    println!("Full sum: {}", full_sum);
}

fn decode(value: &String, mapping: &HashMap<char, HashSet<u8>>) {
    let mut array = [0,0,0,0,0,0,0];
    
    for c in value.chars() {
        
        match mapping.get(&c).unwrap().iter().next() {
            Some(&v) => array[v as usize] = 1,
            _ => {}
        }
    }    

    match array {
        [1, 1, 1, 0, 1, 1, 1] => {
            println!("0");
        },
        [0, 0, 1, 0, 0, 1, 0] => {
            println!("1");
        },
        [1, 0, 1, 1, 1, 0, 1] => {
            println!("2");
        },
        [1, 0, 1, 1, 0, 1, 1] => {
            println!("3");
        },
        [0, 1, 1, 1, 0, 1, 0] => {
            println!("4");
        },
        [1, 1, 0, 1, 0, 1, 1] => {
            println!("5");
        },
        [1, 1, 0, 1, 1, 1, 1] => {
            println!("6");
        },
        [1, 0, 1, 0, 1, 0, 0] => {
            println!("7");
        },
        [1, 1, 1, 1, 1, 1, 1] => {
            println!("8");
        },
        [1, 1, 1, 1, 0, 1, 1] => {
            println!("9");
        },
        _ => ()
    }
}  


fn read_input() -> Vec<Vec<Vec<String>>> {
    let stdin = io::stdin();

    let mut results: Vec<Vec<Vec<String>>> = vec![];

    let mut eof = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                // Split numbers.
                let split = buffer.trim().split(" | ");
                let result: Vec<&str> = split.collect();


                let mut signal_patterns: Vec<String> = result[0].split(" ").map(|x| x.to_string()).collect();
                let output_values: Vec<String> = result[1].split(" ").map(|x| x.to_string()).collect();

                signal_patterns.sort_by(|a,b| a.chars().count().cmp(&b.chars().count()) );

                let mut entry = vec![];
                entry.push(signal_patterns);
                entry.push(output_values);

                results.push(entry);
            }
            _ => { panic!("Error reading line."); }
        }
    }

    return results;
}

