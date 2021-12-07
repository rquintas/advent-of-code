use std::io;

fn main() {
    let crabs_list = read_input();

    let N = crabs_list.len();
    let M: usize = *crabs_list.iter().max().unwrap() as usize;
    let mut fuel_cost: Vec<Vec<i32>> = vec![vec![0; N]; M];

    for c in &crabs_list {
        println!("{}", c);
    }

    println!("Matrix size: {} x {}", M, N);

    for crab_idx in 0..N {
        // println!("Filling crab: {}", crab_idx);
        for cost_idx in 0..M {
            // println!("Filling cost: {}", cost_idx);
            let cost = compute_cost2(crabs_list[crab_idx], cost_idx as i32); 
            fuel_cost[cost_idx][crab_idx] = cost;
        }
    }

    let mut summations: Vec<i32> = Vec::new();
    for cost_idx in 0..M {
        summations.push(fuel_cost[cost_idx].iter().sum());
    }

    let minimum_cost = summations.iter().min();
    println!("Minimum fuel cost: {}", *minimum_cost.unwrap());
}

fn compute_cost(crab_value: i32, target_position: i32) -> i32 {
    let delta = crab_value - target_position;
    let cost = i32::abs(delta);
    return cost;
}

fn compute_cost2(crab_value: i32, target_position: i32) -> i32 {
    let delta = crab_value - target_position;
    let cost = i32::abs(delta);
    return ( cost * (cost + 1)) / 2;
}

fn read_input() -> Vec<i32> {

    let stdin = io::stdin();
    let mut input = String::new();

    match stdin.read_line(&mut input) {
        Ok(_) => {
            // Split numbers.
            let split = input.trim().split(",");
            let result: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).collect();

            return result;
        }
        _ => { panic!("Error reading line."); }
    }

}
