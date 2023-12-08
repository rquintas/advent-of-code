use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn calculate(original_pos: &str, tree: &HashMap<&str,Vec<String>>, instructions: &str) -> usize {

    let mut pos = original_pos;
    let mut steps: usize = 0;

    while pos.chars().last().unwrap() != 'Z' {
        let instruction = instructions.chars().nth(steps % instructions.len()).unwrap();

        match instruction {
            'L' => {
                pos = &tree[pos][0];
            },
            'R' => {
                pos = &tree[pos][1];
            },
            _ => {
                panic!("Unknown instruction: {}", instruction);
            }
        }

        steps += 1;
    }

    return steps;
}

fn part2(input: &str) -> String {

    let mut line_iter = input.lines();

    let instructions = line_iter.next().unwrap();
    let _empty_line = line_iter.next().unwrap();

    println!("{}", instructions);
    
    let mut tree = HashMap::new();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
    
        let mut key_value = line.split(" = ");
        let key = key_value.next().unwrap();
        let value = key_value.next().unwrap();
        let value = value[1..value.len()-1].to_string();
        let value_list = value.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();

        println!("{} = {:?}", key, value_list);

        tree.insert(key, value_list);
    }

    dbg!(&tree);

    let mut frontier = Vec::new();
    for (key, value) in &tree {
        if key.chars().last().unwrap() == 'A' {
            frontier.push(*key);
        }
    }

    let mut all_steps = Vec::new();
    for pos in &frontier {
        let steps = calculate(*pos, &tree, instructions);
        all_steps.push(steps);
    }

    let result = lcm(&all_steps);

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
");
        assert_eq!(result, "2".to_string());
    }    
    
    #[test]
    fn it_works2() {
        let result = part2("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
");
        assert_eq!(result, "6".to_string());
    }    
    
    #[test]
    fn it_works3() {
        let result = part2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
");
        assert_eq!(result, "6".to_string());
    }
}