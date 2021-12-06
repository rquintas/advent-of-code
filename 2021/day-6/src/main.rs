use std::io;

fn main() {
    let number_of_days = 256;
    
    let lantern_fish: Vec<u16> = read_input();

    // Build state array.
    // State array keeps counter of number of fish on each stage of development.
    // u64 because numbers can get really big.
    let mut state: [u64; 10] = [0u64; 10];

    print!("Initial state: ");
    for fish_idx in 0..lantern_fish.len() {
        print!("{},", lantern_fish[fish_idx]);

        // Initialize counters by increasing counter for each fish found according
        // to it's development stage.
        state[lantern_fish[fish_idx] as usize] += 1;
    }
    println!();

    
    // Simulate for each day.
    for day in 0..number_of_days {

        for development_stage in 0..state.len() {
        
            match development_stage {
                0 => {
                    // Create new fish on 9 because it will already 
                    // iterate once this cycle, and move down to 8. 
                    state[9] += state[0];
                    // Fish on development stage 0, have created new fish
                    // now they move to 7, and will be brought to 6 already on this cycle.
                    state[7] += state[0];
                    state[0] = 0;
                },
                _ => {
                    // Each development stage adds it's counter to next stage down.
                    // And reset.
                    state[development_stage-1] += state[development_stage];
                    state[development_stage] = 0;
                }
            }
        }

        print!("After {} days: ", day);
        for idx in 0..state.len() {
            print!("{}({}),", idx, state[idx]);
        }
        println!();

    }

    // Solution sum.
    let mut sum = 0;
    for idx in 0..state.len() {
        sum += state[idx];
    }

    println!("Number of fish: {}", sum);
}

fn read_input() -> Vec<u16> {

    let stdin = io::stdin();
    let mut input = String::new();

    match stdin.read_line(&mut input) {
        Ok(_) => {
            // Split numbers.
            let split = input.trim().split(",");
            let result: Vec<u16> = split.map(|x| x.parse::<u16>().unwrap()).collect();

            return result;
        }
        _ => { panic!("Error reading line."); }
    }

}
