fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

    let mut invalid_id_sum = 0;
    let mut valid_id_sum = 0;

    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    for line in input.lines() {

        let mut valid = true;
        
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
                            valid = false;
                        }
                    },
                    "green" => {
                        if count > max_green_cubes {
                            valid = false;
                        }
                    },
                    "blue" => {
                        if count > max_blue_cubes {
                            valid = false;
                        }
                    },
                    _ => {},
                }
            }
        }


        if valid {
            valid_id_sum += game.parse::<i32>().unwrap();
        } else {
            invalid_id_sum += game.parse::<i32>().unwrap();
        }
    }

    return valid_id_sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
");
        assert_eq!(result, "8".to_string());
    }
}