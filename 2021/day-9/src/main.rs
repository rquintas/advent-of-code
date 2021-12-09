use std::io;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
    basin: usize,
}

fn main() {
    let state = read_heightmap();

    let N = state.len();
    let M = state[0].len();

    let mut lowest: Vec<u8> = vec![];
    let mut frontier: VecDeque<Point> = VecDeque::new();

    for i in 0..N {
        for j in 0..M {
            if is_lowest(i,j,&state) {
                lowest.push(state[i][j]);
                frontier.push_back(Point{ x: i, y: j, basin: lowest.len()});
            }
        }
    }

    println!("Lowest points: {:?}", lowest);
    let sum: u32 = lowest.into_iter().map(|x| (x + 1) as u32).sum();
    println!("Risk level: {}", sum);



    let mut basin_counter: HashMap<usize, u32> = HashMap::new();
    let mut searched_points: HashSet<Point> = HashSet::new();

    while ! frontier.is_empty() {
        let point = frontier.pop_front().unwrap();
        // don't search this again
        if searched_points.contains(&point) {
            continue;
        }

        // add entry to basin tracking
        *basin_counter.entry(point.basin).or_default() += 1;

        // expand frontier
        if point.x > 0 {
            if state[point.x - 1][point.y] != 9 {
                let new_point = Point{ x: point.x - 1, y: point.y, basin: point.basin};
                frontier.push_back(new_point);
            }
        }
    
        if point.x < state.len() - 1 {
            if state[point.x + 1][point.y] != 9 {
                let new_point = Point{ x: point.x + 1, y: point.y, basin: point.basin};
                frontier.push_back(new_point);
            }
        }
    
        if point.y > 0 {
            if state[point.x][point.y - 1] != 9 {
                let new_point = Point{ x: point.x, y: point.y - 1, basin: point.basin};
                frontier.push_back(new_point);
            }
        }
    
        if point.y < state[0].len() - 1 {
            if state[point.x][point.y + 1] != 9 {
                let new_point = Point{ x: point.x, y: point.y + 1, basin: point.basin};
                frontier.push_back(new_point);
            }
        }

        searched_points.insert(point);
    } 

    let mut counter: Vec<u32> = basin_counter.values().cloned().collect();

    counter.sort_by(|a, b| b.cmp(a));

    let mut mult = 1;
    for v in &counter[0..3] {
        mult = mult * v;
    }

    println!("Multiplication of basins: {:?}", mult);
}

fn is_lowest(i: usize, j: usize, state: &Vec<Vec<u8>>) -> bool {

    let mut result = true;

    if i > 0 {
        result = result && state[i][j] < state[i-1][j];
    }

    if i < state.len() - 1 {
        result = result && state[i][j] < state[i+1][j];
    }

    if j > 0 {
        result = result && state[i][j] < state[i][j-1];
    }

    if j < state[0].len() - 1 {
        result = result && state[i][j] < state[i][j+1];
    }

    return result;
}

fn read_heightmap() -> Vec<Vec<u8>> {

    let stdin = io::stdin();

    let mut state :Vec<Vec<u8>> = vec![] ;

    let mut eof = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {         
                // Split string.
                let split = buffer.trim().split("").filter(|x| x.len() == 1);
                
                let s: Vec<&str> = split.collect();
                
                // Convert to numbers.
                let result: Vec<u8> = s.into_iter().map(|x| x.parse::<u8>().unwrap()).collect();

                state.push(result);

            }
            _ => { panic!("Error reading line."); }
        }
    
    }

    // println!("{:?}", state);

    return state;
}