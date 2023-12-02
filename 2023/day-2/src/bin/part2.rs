fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {

    let mut sum = 0;

    for line in input.lines() {

        let mut max_red_cubes = 0;
        let mut max_green_cubes = 0;
        let mut max_blue_cubes = 0;

        let mut parts = line.split(":");
        let game = parts.next().unwrap().split(" ").nth(1).unwrap();

        let mut sets = parts.next().unwrap().split(";");

        for set in sets {
            let mut cubes = set.split(",");

            for cube in cubes {
                let mut parts = cube.trim().split(" ");
                let count = parts.next().unwrap().parse::<i32>().unwrap();
                let color = parts.next().unwrap();

                match color {
                    "red" => {
                        if count > max_red_cubes {
                            max_red_cubes = count;
                        }
                    },
                    "green" => {
                        if count > max_green_cubes {
                            max_green_cubes = count;
                        }
                    },
                    "blue" => {
                        if count > max_blue_cubes {
                            max_blue_cubes = count;
                        }
                    },
                    _ => {},
                }
            }
        }

        println!("{} {} {}", max_red_cubes, max_green_cubes, max_blue_cubes);

        let power = max_red_cubes * max_green_cubes * max_blue_cubes;
        sum += power;

    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
");
        assert_eq!(result, "2286".to_string());
    }
}