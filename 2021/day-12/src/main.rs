use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

// number of distinct paths that start at start, end at end

// visit small caves at most once
// visit big caves any number of times.

// How many paths through this cave system are there that visit small caves at most once?

fn main() {
    let input = read_input();
    println!("{:?}", input);
    let graph = create_graph_map(&input);
    println!("{:?}", graph);

    let solutions = walk(graph, "start".to_string(), "end".to_string());

    // println!("{:?}", solutions);

    for solution in &solutions {
        println!("{:?}", solution.path);
    }

    println!("Number of solutions: {:?}", solutions.len());

}

#[derive(Debug, Clone)]
struct Path {
    next: String,
    path: Vec<String>,
    transitions: HashSet<String>,
    small_visited: HashSet<String>,
    small_cave_visited_twice: bool,
}

fn walk(graph: HashMap<String, Vec<String>>, start: String, end: String) -> Vec<Path> {

    let mut frontier: VecDeque<Path> = VecDeque::new();

    let mut start_path: Vec<String> = vec![];
    start_path.push(start.to_string());

    frontier.push_back(
        Path{
            next: start.to_string(), 
            path: start_path, 
            transitions: HashSet::new(), 
            small_visited: HashSet::new(),
            small_cave_visited_twice: false,
        });

    let mut solutions: Vec<Path> = vec![];

    while ! frontier.is_empty() {
        let mut current: Path = frontier.pop_front().unwrap();

        // Mark small cave as visited
        if current.next.chars().all(|c| c.is_ascii_lowercase()) {
            current.small_visited.insert(current.next.to_string());
        }

        if current.next == end {
            solutions.push(current.clone());
            continue;
        }

        let connected_caves = graph.get(&current.next).unwrap();
        for connected in connected_caves {

            let next = connected.to_string();
            let transition_id = current.next.to_string() + "-" + &next;

            // Allow one small cave to be visited twice unless start
            if current.small_visited.contains(&next) && ! current.small_cave_visited_twice && next != start {
                current.transitions.insert(transition_id);

                let mut path = current.path.clone();
                path.push(next.to_string());
                
                frontier.push_back(
                    Path{next: next, 
                        path: path, 
                        transitions: current.transitions.clone(), 
                        small_visited: current.small_visited.clone(),
                        small_cave_visited_twice: true,
                    });

            }
            // Don't visit again small visited.
            else if ! current.small_visited.contains(&next) {
                current.transitions.insert(transition_id);

                let mut path = current.path.clone();
                path.push(next.to_string());
                
                frontier.push_back(
                    Path{next: next, 
                        path: path, 
                        transitions: current.transitions.clone(), 
                        small_visited: current.small_visited.clone(),
                        small_cave_visited_twice: current.small_cave_visited_twice,
                    });
            }
        
        }
    }
    
    return solutions;

}

fn create_graph_map(input: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for entry in input {
        graph.entry(entry[0].to_string()).or_default().push(entry[1].to_string());
        graph.entry(entry[1].to_string()).or_default().push(entry[0].to_string());
    }

    return graph;
}

fn read_input() -> Vec<Vec<String>> {
    let stdin = io::stdin();

    let mut results: Vec<Vec<String>> = vec![];

    let mut eof = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                let split = buffer.trim().split("-");
                let entry: Vec<String> = split.map(|x| x.to_string()).collect();
                results.push(entry);
            }
            _ => { panic!("Error reading line."); }
        }
    }

    return results;
}
