use std::io;

fn main() {

    // Skip counting first entry.
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;
    
    let stdin = io::stdin();
    let mut eof = false;

    while !eof {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {                
                let split = input.trim().split(" ");
                let vec: Vec<&str> = split.collect();
                let modifier: i32 = vec[1].parse().unwrap();

                match vec[0] {
                    "forward" => {
                        horizontal += modifier;
                        depth = depth + aim * modifier;
                    },
                    "down" => aim += modifier,
                    "up" => aim -= modifier,
                    _ => panic!("not accepted command"),
                }

            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    println!("{}", depth*horizontal);
}

fn part1() {

    // Skip counting first entry.
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    
    let stdin = io::stdin();
    let mut eof = false;

    while !eof {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {                
                let split = input.trim().split(" ");
                let vec: Vec<&str> = split.collect();
                let modifier: i32 = vec[1].parse().unwrap();

                match vec[0] {
                    "forward" => horizontal += modifier,
                    "down" => depth += modifier,
                    "up" => depth -= modifier,
                    _ => panic!("not accepted command"),
                }

            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    println!("{}", depth*horizontal);
}