use std::io;

fn main() {
    
    let stdin = io::stdin();
    let mut eof = false;

    let mut vectors: Vec<Vec<u32>> = Vec::new();
    
    while !eof {
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {             

                let binary_vec = read_binary_line(&mut buffer);
                for bin in &binary_vec {
                    print!("{}",bin);
                }
                println!();

                vectors.push(binary_vec);
            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }
    
    let mut gamma : u32 = 0;
    let mut epsilon : u32 = 0;

    let mut oxygen_generator_rating_vectors = vectors.clone();
    let mut co2_scrubber_rating_vectors = vectors.clone();

    for column in 0..vectors[0].len() {
        let mut checked_value = get_most_common(&vectors, column);

        let mut tied = false;
        if checked_value == -1 {
            tied = true;
            checked_value = 1;
        }

        let most_common = checked_value as u32;
        let least_common = 1 - most_common;

        println!("{} {} {}", column, most_common, tied);

        gamma += most_common * u32::pow(2, (vectors[0].len() as u32) -(column+1) as u32);
        epsilon += least_common * u32::pow(2, (vectors[0].len() as u32) - (column+1) as u32);

        // Decision is local
        if oxygen_generator_rating_vectors.len() > 1 {

            let mut checked_value = get_most_common(&oxygen_generator_rating_vectors, column);

            let mut tied = false;
            if checked_value == -1 {
                tied = true;
                checked_value = 1;
            }
    
            let most_common = checked_value as u32;

            if tied {
                oxygen_generator_rating_vectors = 
                    oxygen_generator_rating_vectors.into_iter().filter(|vector| vector[column] == 1).collect();
            } else {
                oxygen_generator_rating_vectors = 
                    oxygen_generator_rating_vectors.into_iter().filter(|vector| vector[column] == most_common).collect();
            }

        }

        // Decision is local
        if co2_scrubber_rating_vectors.len() > 1 {
            let mut checked_value = get_most_common(&co2_scrubber_rating_vectors, column);

            let mut tied = false;
            if checked_value == -1 {
                tied = true;
                checked_value = 1;
            }
    
            let most_common = checked_value as u32;
            let least_common = 1 - most_common;

            if tied {
                co2_scrubber_rating_vectors = 
                    co2_scrubber_rating_vectors.into_iter().filter(|vector| vector[column] == 0).collect();
            } else {
                co2_scrubber_rating_vectors = 
                    co2_scrubber_rating_vectors.into_iter().filter(|vector| vector[column] == least_common).collect();
            }
        }
    }

    println!("What is the power consumption of the submarine?");
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Power Consumption: {}", gamma * epsilon);

    println!("What is the life support rating of the submarine?");

    let oxygen_generator_rating = convert_binary(&oxygen_generator_rating_vectors[0]);
    let co2_scrubber_rating = convert_binary(&co2_scrubber_rating_vectors[0]);

    for bin in &oxygen_generator_rating_vectors[0] {
        print!("{}",bin);
    }
    println!();
    for bin in &co2_scrubber_rating_vectors[0] {
        print!("{}",bin);
    }
    println!();

    println!("Oxygen Generator Rating: {}", oxygen_generator_rating);
    println!("C02 Scrubber Rating: {}", co2_scrubber_rating);
    println!("Life support rating: {}", oxygen_generator_rating * co2_scrubber_rating);

}

fn read_binary_line(buffer: &mut String) -> Vec<u32> {
    // Split numbers.
    let split = buffer.trim().chars();
    let result: Vec<u32> = split.map(|x| x.to_digit(10).unwrap()).collect();
    return result;
}

fn get_most_common(vectors: &Vec<Vec<u32>>, column: usize) -> i32 {
    let mut n0 = 0;
    let mut n1 = 0;
    for v in 0..vectors.len() {
        if vectors[v][column] == 1 {
            n1 += 1;
        } else {
            n0 += 1;
        }
    }
    if n1 > n0 {
        return 1;
    }
    if n0 > n1 {
        return 0;
    }
    return -1;
}

fn convert_binary(vector: &Vec<u32>) -> u32 {
    let mut result = 0;
    for n in 0..vector.len() {
        result += vector[n] * u32::pow(2, (vector.len() as u32) - (n+1) as u32);
    }
    return result;
}