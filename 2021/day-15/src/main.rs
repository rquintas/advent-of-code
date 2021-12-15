use std::io;
use std::collections::HashMap;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    
    let heightmap = read_heightmap();

    // Copy height map 5 times
    let mut heightmap_unfolded: Vec<Vec<usize>> = 
        (0..heightmap.len() * 5).map(|_| (0..heightmap[0].len() * 5).map(|_| usize::MAX).collect()).collect();

    for i in 0..heightmap_unfolded.len() {
        let target_i = i % heightmap.len();
        for j in 0..heightmap_unfolded.len() {
            let target_j = j % heightmap[0].len();
            let additional_cost = i / heightmap.len() + j / heightmap[0].len();

            heightmap_unfolded[i][j] = heightmap[target_i][target_j] + additional_cost;

            // Reset above 9 cost.
            if heightmap_unfolded[i][j] > 9 {
                heightmap_unfolded[i][j] -= 9;
            }

        }
    }
    
    for i in 0..heightmap_unfolded.len() {
        println!("{:?}", heightmap_unfolded[i] );
    }

    dijkstra(heightmap_unfolded);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    cost: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(heightmap: Vec<Vec<usize>>) {

    let mut distance: Vec<Vec<usize>> = (0..heightmap.len()).map(|_| (0..heightmap[0].len()).map(|_| usize::MAX).collect()).collect();

    distance[0][0] = 0;

    let mut frontier = BinaryHeap::new();
    frontier.push(Point { cost: heightmap[0][1], x: 0, y: 1 });
    frontier.push(Point { cost: heightmap[1][0], x: 1, y: 0 });

    while let Some(Point { x, y, cost }) = frontier.pop() {

        // Avoid out of bounds.
        if x >= heightmap.len() { continue; }
        if y >= heightmap[0].len() { continue; }

        // Already found a better path.
        if cost > distance[x][y] { continue; }

        // Check if can still go down
        if y < heightmap[0].len() - 1 {
            let next1 = Point { cost: cost + heightmap[x][y+1], x: x , y: y + 1 };
            if next1.cost < distance[next1.x][next1.y] {
                frontier.push(next1);
                distance[next1.x][next1.y] = next1.cost;
            }
        }

        // Check if can still go up
        if y > 0 {
            let next1 = Point { cost: cost + heightmap[x][y-1], x: x , y: y - 1 };
            if next1.cost < distance[next1.x][next1.y] {
                frontier.push(next1);
                distance[next1.x][next1.y] = next1.cost;
            }
        }

        // Check if can still go left
        if x < heightmap.len() - 1 {         
            let next2 = Point { cost: cost + heightmap[x+1][y], x: x + 1 , y: y };
            if next2.cost < distance[next2.x][next2.y] {
                frontier.push(next2);
                distance[next2.x][next2.y] = next2.cost;
            }
        }

        // Check if can still go right
        if x > 0 {         
            let next2 = Point { cost: cost + heightmap[x-1][y], x: x - 1 , y: y };
            if next2.cost < distance[next2.x][next2.y] {
                frontier.push(next2);
                distance[next2.x][next2.y] = next2.cost;
            }
        }
    }

    println!("{:?}", distance[distance.len()-1][distance[0].len()-1]);
    
}

fn read_heightmap() -> Vec<Vec<usize>> {

    let stdin = io::stdin();

    let mut state :Vec<Vec<usize>> = vec![] ;

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
                let result: Vec<usize> = s.into_iter().map(|x| x.parse::<usize>().unwrap()).collect();

                state.push(result);

            }
            _ => { panic!("Error reading line."); }
        }
    
    }

    // println!("{:?}", state);

    return state;
}