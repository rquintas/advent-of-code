use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

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

    let mut pos = "AAA";
    let mut steps = 0;

    while pos != "ZZZ" {
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

    steps.to_string()    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("RL

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
        let result = part1("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
");
        assert_eq!(result, "6".to_string());
    }
}