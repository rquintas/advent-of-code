use std::io;
use std::collections::VecDeque;

fn main() {
    part2();
}

fn part1() {

    let mut previous: i32 = -1;

    // Skip counting first entry.
    let mut count: i32 = -1;
    
    let stdin = io::stdin();
    let mut eof = false;

    while !eof {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {                
                let current: i32 = input.trim().parse().unwrap();

                if previous < current {
                    count += 1;
                }
                
                previous = current;
            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    println!("{}", count)
}

fn part2() {
    let mut previous = VecDeque::new();

    let mut count: i32 = 0;
    
    let stdin = io::stdin();
    let mut eof = false;

    let sliding_window_size: usize = 3;

    while !eof {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {                
                let current: i32 = input.trim().parse().unwrap();

                let mut previous_sum: i32 = -1;
                if previous.len() == sliding_window_size {
                    previous_sum = previous.iter().sum();
                    // Keep only sliding_window_size entries on the queue, consume front.
                    previous.pop_front();
                }

                // Add to queue.
                previous.push_back(current);

                let mut current_sum: i32 = -1;

                // Compute new value if possible.
                if previous.len() == sliding_window_size {
                    current_sum = previous.iter().sum();
                }

                // If values initialized compute count.
                if previous_sum > 0 && current_sum > 0 {
                    if previous_sum < current_sum {
                        count += 1;
                    }
                }
            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    println!("{}", count)
}