use std::io;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let mut matrix = read_input();

    println!("{:?}", matrix);

    let N = matrix.len() as u32;
    let M = matrix[0].len() as u32;

    pad(&mut matrix);
    println!("{:?}", matrix);

    let mut sum = 0;
    print_state(&matrix);
    println!("{}", sum);
    for s in 1..500 {
        let new_flashes = step(&mut matrix);
        sum += new_flashes;

        // Part 1
        if s == 100 {
            println!("Sum at 100: {}", sum);
        }

        // Part 2
        if new_flashes == N * M {
            println!("Synched: {}, {}", s, new_flashes);
        }

    }
}

fn print_state(state: &VecDeque<VecDeque<u8>>) {
    for i in 1..state.len()-1 {
        for j in 1..state[0].len()-1 {
            print!("{}", state[i][j]);
        }
        println!();
    }
}

fn pad(state: &mut VecDeque<VecDeque<u8>>) {

    let mut padding: VecDeque<u8> = VecDeque::with_capacity(state[0].len());
    for _i in 0..state[0].len() {
        padding.push_back(9);
    }

    state.push_back(padding.clone());
    state.push_front(padding);

    for row in state {
        row.push_back(9);
        row.push_front(9);
    }

}

fn step(state: &mut VecDeque<VecDeque<u8>>) -> u32 {

    let mut flash_count: u32 = 0;
    let mut flash_track: HashSet<u32> = HashSet::new();

    // First, the energy level of each octopus increases by 1.
    for i in 1..state.len()-1 {
        for j in 1..state[0].len()-1 {
            state[i][j] += 1;
        }
    }

    // Then, any octopus with an energy level greater than 9 flashes. 
    // This increases the energy level of all adjacent octopuses by 1, 
    // including octopuses that are diagonally adjacent. 
    // If this causes an octopus to have an energy level greater than 9, 
    // it also flashes. This process continues as long as new octopuses 
    // keep having their energy level increased beyond 9. 
    // (An octopus can only flash at most once per step.)
    let mut updated = true;
    while updated {
        updated = false;

        for i in 1..state.len()-1 {
            for j in 1..state[0].len()-1 {

                let id: u32 = i as u32 * state.len() as u32 + j as u32;

                // Skip flashing if already flashed this step
                if flash_track.contains(&id) {
                    continue;
                }

                if state[i][j] > 9 {
                    updated = true;
                    flash_track.insert(id);

                    flash_count += 1;

                    // Update all states around
                    state[i-1][j-1] += 1;
                    state[i-1][j] += 1;
                    state[i][j-1] += 1;
                    state[i+1][j-1] += 1;
                    state[i-1][j+1] += 1;
                    state[i+1][j+1] += 1;
                    state[i][j+1] += 1;
                    state[i+1][j] += 1;
                }

            } 
        }
    }

    // Finally, any octopus that flashed during this step has 
    // its energy level set to 0, as it used all of its energy to flash.

    // Y = (int)(index / Width)
    // X = index - (Y * Width)

    for id in flash_track {
        let i = id / state.len() as u32;
        let j = id - ( i * state.len() as u32);
        
        state[i as usize][j as usize] = 0;

    }

    return flash_count;
}

fn read_input() -> VecDeque<VecDeque<u8>> {
    let stdin = io::stdin();

    let mut results: VecDeque<VecDeque<u8>> = VecDeque::new();

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
                let result: VecDeque<u8> = split.map(|x| x.parse().unwrap() ).collect();
                results.push_back(result);
            }
            _ => { panic!("Error reading line."); }
        }
    }

    return results;
}
