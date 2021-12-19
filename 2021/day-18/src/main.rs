use std::io;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: u32
}

#[derive(Debug, PartialEq)]
enum Dir {
    LEFT,
    RIGHT,
    UNK
}

#[derive(Debug, PartialEq)]
struct Bubble {
    left: Option<u32>,
    right: Option<u32>,
    depth: usize,
    turned: bool,
    lastdir: Dir,
    originalpos: Dir,
}

fn main() {

    let mut stack: VecDeque<Node> = VecDeque::new();


    let lines: Vec<String> = 
        "[1,[[3,6],[0,[6,3]]]]
[[[5,2],[[5,0],6]],[6,[5,1]]]
[[5,[[2,3],[7,1]]],[4,[9,[8,3]]]]
[[8,[[3,4],[8,7]]],[[[4,0],[3,5]],[[0,1],6]]]
[[1,[6,[9,0]]],[[7,[5,7]],[[8,9],3]]]
[[[[6,7],[4,9]],7],9]
[[7,3],[[8,9],[7,[4,2]]]]
[[[4,[2,9]],[0,3]],[[4,[0,8]],[[4,4],3]]]
[[[[6,9],9],8],[[[4,0],[1,6]],[4,[3,6]]]]
[[4,[4,[3,3]]],[[2,1],[[6,1],[9,4]]]]
[[5,[6,7]],[[[5,8],[4,3]],[4,[0,8]]]]
[[6,[[9,6],5]],[0,[6,6]]]
[[[[1,5],9],[[5,3],5]],[[[2,0],3],9]]
[4,3]
[[1,8],[[[1,0],[3,8]],3]]
[[[[2,0],[6,5]],4],[[[9,8],[0,1]],3]]
[[8,[7,8]],[[6,[3,2]],[[8,1],[7,5]]]]
[[[[1,4],2],[[4,8],[3,2]]],[[[2,2],6],6]]
[[[4,[0,5]],[[8,8],[7,2]]],[[4,[4,1]],2]]
[[1,[4,[4,0]]],[2,[[2,3],1]]]
[[[[2,1],0],[[3,4],1]],[[2,4],3]]
[[9,[8,7]],[7,[0,[8,0]]]]
[[[9,9],7],[[0,[2,1]],[[7,1],4]]]
[[6,[[3,2],[0,0]]],[[[4,1],9],[7,3]]]
[[[5,[5,6]],[[7,7],[7,8]]],4]
[[8,[[4,1],4]],[[[6,4],[0,3]],[5,[6,4]]]]
[[[9,0],[2,8]],[[6,5],5]]
[[[3,[1,6]],[[5,3],6]],[[[7,4],[4,9]],[[2,3],[6,5]]]]
[[8,[6,7]],6]
[[[[6,0],[1,3]],[0,0]],[[[4,7],[7,8]],[[7,2],2]]]
[[6,[[9,6],[1,1]]],7]
[[[2,3],[6,0]],[3,[[9,3],9]]]
[[[3,0],0],[[[6,0],3],[[1,5],4]]]
[[8,[[0,3],8]],[[[0,8],[4,3]],[8,[3,4]]]]
[[[4,4],0],[[1,[8,0]],[[9,6],3]]]
[8,[[6,[6,7]],[8,7]]]
[[8,[0,[1,4]]],3]
[[[[9,5],0],[[5,3],[1,9]]],[[[7,3],5],[[4,3],9]]]
[[[[9,0],[4,2]],[0,[3,2]]],1]
[[[6,[4,2]],[[5,5],9]],[[[6,1],9],[[3,8],[8,1]]]]
[[[3,[5,0]],[[5,2],[2,2]]],[[0,2],[7,4]]]
[[3,[[5,7],[2,8]]],4]
[[[4,8],5],0]
[[[6,9],[[7,0],7]],[8,7]]
[[7,[[1,3],[0,2]]],[[[4,8],0],[[7,0],6]]]
[[[1,7],[6,6]],[[6,[4,0]],[0,4]]]
[[[[2,2],[3,9]],9],3]
[0,[[4,9],[[5,5],[5,9]]]]
[[[[4,4],2],[6,4]],[[[4,1],[2,0]],[[9,4],0]]]
[[[0,[3,4]],[2,3]],[[7,[2,3]],[3,3]]]
[[[[0,3],9],7],[7,[4,[9,6]]]]
[[9,[[0,8],4]],[5,[2,[4,9]]]]
[[6,2],[[1,7],0]]
[[[[1,6],[8,3]],1],[[[6,7],2],[[4,4],8]]]
[[[[7,1],[0,3]],0],[5,[4,[8,4]]]]
[[[[4,2],[6,2]],[[5,7],8]],[[7,9],4]]
[[[0,[0,4]],5],2]
[[2,[[0,6],6]],[[[3,4],3],[4,5]]]
[[[[6,4],9],[[7,1],0]],[[[8,2],[3,2]],[[1,9],7]]]
[7,[[7,8],[[5,5],0]]]
[[[1,2],[8,5]],[[5,4],[8,0]]]
[[4,[1,3]],[[[4,5],[1,2]],[5,1]]]
[[[[0,7],[4,5]],[9,[2,2]]],[4,[[1,8],[7,5]]]]
[[4,[[0,4],[8,8]]],[[[9,2],[7,1]],[8,[9,5]]]]
[1,3]
[[[[8,9],5],0],[1,6]]
[[[[6,6],[3,5]],[[2,8],[3,3]]],[[[5,3],[5,9]],[0,[1,4]]]]
[[7,[7,[5,5]]],[4,[4,[9,9]]]]
[[[7,[6,7]],[4,2]],[0,[[7,8],1]]]
[[[[6,0],4],[3,[6,9]]],5]
[[[[9,8],6],[[7,4],[3,4]]],[[[8,8],[2,1]],0]]
[9,[[1,7],[7,1]]]
[6,[[3,[3,6]],[2,9]]]
[[[6,9],[[1,4],2]],[7,3]]
[[1,[6,[8,5]]],[[[0,0],0],[7,2]]]
[[[4,[2,7]],[[0,0],8]],[[4,[4,5]],[[4,8],[3,3]]]]
[[[[7,4],[7,5]],[[5,8],3]],[[[6,9],[0,9]],[[7,2],[4,0]]]]
[4,[4,[9,[5,7]]]]
[[[[8,7],3],[6,[0,5]]],[[[7,8],[5,1]],[[0,4],2]]]
[6,[0,[4,3]]]
[[[[6,5],3],7],[[[0,1],5],[6,[2,6]]]]
[[[9,1],[[8,8],[8,2]]],0]
[[[[3,4],1],3],[8,[[1,5],[5,6]]]]
[[[[6,8],2],4],[[5,8],[[1,5],[7,0]]]]
[[3,8],[[[9,0],2],[7,[5,8]]]]
[[[[7,5],6],[[4,4],[5,0]]],[4,[3,[3,0]]]]
[[[7,9],[1,[8,8]]],[[[6,8],4],4]]
[[4,[[6,7],[5,7]]],[6,7]]
[[[[8,8],[0,4]],[[5,5],1]],6]
[[[7,7],[[5,8],[3,4]]],[[0,[7,4]],5]]
[8,[[1,2],[6,9]]]
[[[[9,5],[0,6]],[2,[8,7]]],[[[9,2],4],6]]
[[[1,[5,2]],5],[[1,0],3]]
[[7,[7,[3,7]]],[[2,6],3]]
[1,[[8,[7,1]],[3,8]]]
[[[[3,2],[5,6]],2],[7,[0,0]]]
[[[7,[4,6]],[9,[7,8]]],9]
[[[[4,3],9],8],[[8,5],6]]
[3,[[3,1],[[8,4],8]]]
[[[9,[3,5]],[[0,9],[8,5]]],5]".split("\n").map(|x| x.to_string()).collect(); //4289

// let lines: Vec<String> = 
//         "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
// [[[5,[2,8]],4],[5,[[9,9],0]]]
// [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
// [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
// [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
// [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
// [[[[5,4],[7,7]],8],[[8,3],8]]
// [[9,3],[[9,9],[6,[4,9]]]]
// [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
// [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".split("\n").map(|x| x.to_string()).collect();

//     let lines: Vec<String> = 
//         "[1,1]
// [2,2]
// [3,3]
// [4,4]
// [5,5]".split("\n").map(|x| x.to_string()).collect();

    let mut results: Vec<u32> = vec![];
    let M = lines.len();
    for idx1 in 0..M {
        for idx2 in 0..M {

            if idx1 == idx2 {
                continue;
            }

            let mut stack: VecDeque<Node> = VecDeque::new();

            let mut pair: Vec<String> = vec![];
            pair.push(lines[idx1].to_string());
            pair.push(lines[idx2].to_string());

            results.push(calculate(pair, &mut stack));
        }
    }

    println!("{:?}", results.iter().max());
}

fn calculate(lines: Vec<String>, stack: &mut VecDeque<Node>) -> u32 {

    for line in lines {
        for c in line.chars() {
            match c {
                '[' => {},
                ']' => {
    
                    let node2 = stack.pop_back().unwrap();
                    let node1 = stack.pop_back().unwrap();
    
                    let new_node = Node{ left: Some(Box::new(node1)), right: Some(Box::new(node2)), value: 0 };
                    stack.push_back(new_node)
                },
                ',' => {},
                x => {
                    let value = x.to_digit(10).unwrap();
                    
                    let new_node = Node{ left: None, right: None, value: value };
                    stack.push_back(new_node);
                },
            }
        }

        if stack.len() == 2 {
            let mut node2 = stack.pop_back().unwrap();
            let mut node1 = stack.pop_back().unwrap();

            converge(&mut node2);
            converge(&mut node1);

            let mut new_node = Node{ left: Some(Box::new(node1)), right: Some(Box::new(node2)), value: 0 };

            converge(&mut new_node);
            
            stack.push_back(new_node)
        }

    }

    let mut tree = stack.pop_back().unwrap();
    converge(&mut tree);

    
    print_tree(&mut tree, 0);
    let result = magnitude(&tree);
    println!("Magnitude: {}", result);

    return result;
}

fn converge(tree: &mut Node) {
    let mut changed: bool = true;
    while changed {
        // Convege explosions.
        while changed {
            changed = false;
            explode_tree(tree, &mut changed, 0);
        }

        // split
        changed = false;
        split_tree(tree, &mut changed, 0);
    }
}

fn magnitude(tree: &Node) -> u32 {
    if tree.left.is_some() {
        return 3*magnitude(tree.left.as_ref().unwrap()) + 2*magnitude(tree.right.as_ref().unwrap());
    } else {
        return tree.value;
    }
}

fn split_tree(tree: &mut Node, changed: &mut bool, depth: usize) {
    
    if ! *changed {
        match &mut tree.left {
            Some(l) => {
                split_tree(l, changed, depth+1);
            },
            _ => {

                if tree.value >= 10 {

                    tree.left = Some(Box::new(Node{ left: None, right: None, value: ( tree.value as f32 / 2.0 ).floor() as u32 }));
                    tree.right = Some(Box::new(Node{ left: None, right: None, value: ( tree.value as f32 / 2.0 ).ceil() as u32  }));

                    tree.value = 0;

                    *changed = true;

                    return;
                }    
            },
        }
    }
    if ! *changed {
        match &mut tree.right {
            Some(r) => {
                split_tree(r, changed, depth+1);
            },
            _ => (),
        }
    }
}


fn explode_tree(tree: &mut Node, changed: &mut bool, depth: usize) -> Option<Bubble> {

    if tree.left.is_some() && depth == 4 {

        if *changed == true {
            return None;
        }

        let bubble = Some(Bubble {
            left: Some(tree.left.as_ref().unwrap().value), 
            right: Some(tree.right.as_ref().unwrap().value),
            depth: depth,
            turned: false,
            lastdir: Dir::UNK,
            originalpos: Dir::UNK,
        });

        tree.left = None;
        tree.right = None;
        tree.value = 0;

        *changed = true;

        return bubble;

    } else {

        match &mut tree.left {
            Some(l) => {
                if let Some(mut bubble) = explode_tree(l, changed, depth+1) {

                    if bubble.left.is_none() && bubble.right.is_none() {
                        return None;
                    }

                    if bubble.lastdir != Dir::UNK && bubble.lastdir != Dir::LEFT {
                        bubble.turned = true;
                    }

                    if bubble.turned {

                        match bubble.left {
                            Some(left_value) => 
                                add_to_rightmost(tree.left.as_deref_mut().unwrap(), left_value),
                            _ => (),
                        }

                        match bubble.right {
                            Some(left_value) => 
                                add_to_leftmost(tree.right.as_deref_mut().unwrap(), left_value),
                            _ => (),
                        }

                    } else {
                        
                        match bubble.right {
                            Some(right_value) => 
                                add_to_leftmost(tree.right.as_deref_mut().unwrap(), right_value),
                            _ => (),
                        }

                        if bubble.originalpos == Dir::UNK {
                            bubble.originalpos = Dir::LEFT;
                        }

                        return Some(Bubble { left: bubble.left, right: None, depth: bubble.depth, turned: bubble.turned, lastdir: Dir::LEFT, originalpos:  bubble.originalpos });
                    }
                }
            },
            _ => (),
        }

        match &mut tree.right {
            Some(r) => {
                if let Some(mut bubble) = explode_tree(r, changed, depth+1) {

                    if bubble.left.is_none() && bubble.right.is_none() {
                        return None;
                    }

                    if bubble.lastdir != Dir::UNK && bubble.lastdir != Dir::RIGHT {
                        bubble.turned = true;
                    }

                    if bubble.turned {

                        match bubble.right {
                            Some(right_value) => 
                                add_to_leftmost(tree.right.as_deref_mut().unwrap(), right_value),
                            _ => (),
                        }     

                        match bubble.left {
                            Some(left_value) => 
                                add_to_rightmost(tree.left.as_deref_mut().unwrap(), left_value),
                            _ => (),
                        }

                    } else {
                        match bubble.left {
                            Some(left_value) => 
                                add_to_rightmost(tree.left.as_deref_mut().unwrap(), left_value),
                            _ => (),
                        }

                        if bubble.originalpos == Dir::UNK {
                            bubble.originalpos = Dir::RIGHT;
                        }

                        return Some(Bubble { left: None, right: bubble.right, depth: bubble.depth, turned: bubble.turned, lastdir: Dir::RIGHT, originalpos:  bubble.originalpos });
                    }
                }
            },
            _ => (),
        }
    
    }
    
    return None
}

fn add_to_leftmost(tree: &mut Node, value: u32) {
    match &mut tree.left {
        Some(l) => {
            add_to_leftmost(l, value);
        },
        _ => {
            tree.value += value;
        },
    }
}

fn add_to_rightmost(tree: &mut Node, value: u32) {
    match &mut tree.right {
        Some(r) => {
            add_to_rightmost(r, value);
        },
        _ => tree.value += value,
    }
}


fn print_tree(tree: &mut Node, depth: usize) {
    match &mut tree.left {
        Some(l) => {
            println!("{}["," ".repeat(depth));
            print_tree(l, depth+1);
        },
        _ => {
            print!("{}", " ".repeat(depth));
            println!("{}", tree.value);        
        },
    }
    match &mut tree.right {
        Some(r) => {
            print_tree(r, depth+1);
            println!("{}]"," ".repeat(depth));
        },
        _ => (),
    }
    
}

fn print_tree2(tree: &mut Node, depth: usize) {
    match &mut tree.left {
        Some(l) => {
            print!("[");
            print_tree2(l, depth+1);
        },
        _ => {
            print!("{},", tree.value);        
        },
    }
    match &mut tree.right {
        Some(r) => {
            print_tree2(r, depth+1);
            print!("]");
        },
        _ => (),
    }
    
}
