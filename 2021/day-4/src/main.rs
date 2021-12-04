use std::io;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    
    let drawing_order: Vec<u16> = read_drawing_order();

    for n in 0 .. drawing_order.len() {
        println!("{}", drawing_order[n]);
    }

    println!("Reading a board:");

    let mut board_states: Vec<[[u16; 5]; 5]> = Vec::new();
    let mut board_masks: Vec<[[u16; 5]; 5]> = Vec::new();

    // Read all boards.
    let stdin = io::stdin();
    let mut eof = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {         
                // Just read a new line, can read a board state after.    
                
                let board_state = read_board();
                board_states.push(board_state);
                board_masks.push([[0u16; 5]; 5]);
                print_board(board_state);
            },
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    // Draw each number of the drawing order, and evaluate board states.

    let mut winner_boards: HashSet<usize> = HashSet::new();

    for n in 0..drawing_order.len() {
        println!("Draw {}", drawing_order[n]);

        for b in 0..board_states.len() {

            // Skip boards that have already won.
            if winner_boards.contains(&b) {
                continue;
            }

            set_board_mask_value(&mut board_masks[b], board_states[b], drawing_order[n]);

            // Check if current board is a solution:
            if evaluate_mask_value(board_masks[b]) {
                println!("Board {} is a winner!", b);
                print_board(board_states[b]);
                print_board(board_masks[b]);

                let sum = sum_board(board_masks[b], board_states[b]);

                println!("Sum of unmarked numbers: {}", sum);
                println!("Final score: {}", sum * drawing_order[n] as u32);

                winner_boards.insert(b);
            }
        }
        
    }

}

fn read_drawing_order() -> Vec<u16> {

    let stdin = io::stdin();
    let mut input = String::new();

    match stdin.read_line(&mut input) {
        Ok(_) => {         
            // Remove new line at end    
            input.pop();

            // Split numbers.
            let split = input.trim().split(",");
            let result: Vec<u16> = split.map(|x| x.parse::<u16>().unwrap()).collect();

            return result;
        }
        _ => { panic!("Error reading line."); }
    }

}


fn read_board() -> [[u16; 5]; 5] {

    let stdin = io::stdin();

    let mut state = [[0u16; 5]; 5];

    // Remove contiguous spaces.
    let re = Regex::new(r"\s+").unwrap();

    // Read 5 lines of the board
    for row in 0..5 {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(_) => {         
                // Split string.
                let input = re.replace_all(&mut buffer, " ");
                let split = input.trim().split(" ");

                // Convert to numbers.
                let result: Vec<u16> = split.map(|x| x.parse::<u16>().unwrap()).collect();

                for column in 0..5 {
                    state[row][column] = result[column];
                }

            }
            _ => { panic!("Error reading line."); }
        }
    }

    return state;
}

fn set_board_mask_value(mask: &mut [[u16; 5]; 5], board_state: [[u16; 5]; 5], value: u16) {

    for row in 0..5 {
        for column in 0..5 {
            if board_state[row][column] == value {
                mask[row][column] = 1;
            }
        }
    }
}

fn sum_board(mask: [[u16; 5]; 5], board_state: [[u16; 5]; 5]) -> u32 {

    let mut sum: u32 = 0 ;
    for row in 0..5 {
        for column in 0..5 {
            if mask[row][column] == 0 {
                sum += board_state[row][column] as u32;
            }
        }
    }
    return sum;
}

fn evaluate_mask_value(mask: [[u16; 5]; 5]) -> bool {

    // Check all 1 rows.
    for row in 0..5 {
        for column in 0..5 {
            if mask[row][column] != 1 {
                break;
            }
            if column == 4 {
                return true;
            }
        }
    }

    // Check all 1 columns.
    for column in 0..5 {
        for row in 0..5 {
            if mask[row][column] != 1 {
                break;
            }
            if row == 4 {
                return true;
            }
        }
    }

    return false;
}

fn print_board(board_state: [[u16; 5]; 5]) {

    for row in 0..5 {
        for column in 0..5 {
            print!("{} ", board_state[row][column]);
        }
        println!("");
    }
    println!("");

}
