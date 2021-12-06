use std::io;
use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Hash, Debug)]
struct Point {
    x: i16,
    y: i16,
}

fn main() {

    let mut overlap_counters: HashMap<Point, u32> = HashMap::new();

    let stdin = io::stdin();
    let mut eof = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {                         
                // Split numbers.
                let split = buffer.trim().replace(" -> ", ",");
                let result: Vec<i16> = split.split(",").map(|x| x.parse::<i16>().unwrap()).collect();

                let x1 = result[0];
                let y1 = result[1];
                let x2 = result[2];
                let y2 = result[3];

                println!("{},{} -> {},{}", x1, y1, x2, y2);

                for point in get_all_points(x1, y1, x2, y2) {
                    *overlap_counters.entry(point).or_default() += 1;
                }

            },
            Err(_error) => { panic!("something went wrong"); }
        }
    }
    
    // for (key, value) in &overlap_counters {
    //     println!("{:?} / {}", key, value);
    // }

    let mut counter = 0;
    for n in overlap_counters.values() {
        if *n >= 2 {
            counter += 1;
        }
    }
    println!("Found {} overlapping points > 2.", counter);

}

fn get_all_points(x1: i16, y1: i16, x2: i16, y2: i16) -> Vec<Point> {
    let mut points = Vec::new();
    
    if x1 == x2 {
        // vertical line
        for n in cmp::min(y1,y2)..=cmp::max(y1, y2) {
            points.push(Point{ x: x1, y: n });
        }
    } else if y1 == y2 {
        // horizontal line

        for n in cmp::min(x1,x2)..=cmp::max(x1, x2) {
            points.push(Point{ x: n, y: y1 });
        }
    } else {
        // diagonal line
        let dx = if x2 - x1 > 0 { 1 } else { -1 };
        let dy = if y2 - y1 > 0 { 1 } else { -1 };
        let mut x = x1;
        let mut y = y1;
        points.push(Point{ x: x, y: y});
        while x != x2 {
            x += dx;
            y += dy;
            points.push(Point{ x: x, y: y});
        }
    }

    return points;
}