use std::io;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Eq, Hash, Debug)]
struct Point {
    x: u16,
    y: u16
}

#[derive(Debug)]
struct Fold {
    axis: u16,
    value: u16
}

fn main() {

    // Read Input 

    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = vec![];

    let stdin = io::stdin();
    let mut eof = false;
    let mut read_fold = false;
    while !eof {
        
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                eof = true;
            }
            Ok(1) => {
                read_fold = true;
            }
            Ok(_) => {           
                
                if ! read_fold {
                    // Split numbers.
                    let split = buffer.trim().split(",");
                    let result: Vec<u16> = split.map(|x| x.parse::<u16>().unwrap()).collect();

                    let x = result[0];
                    let y = result[1];
                
                    points.insert(Point{x: x, y: y});

                } else {
                    let split = buffer.trim().replace("fold along ", "");
                    let fold_instruction: Vec<&str> = split.split("=").collect();

                    let value = fold_instruction[1].parse::<u16>().unwrap();

                    match fold_instruction[0] {
                        "x" => {
                            folds.push(Fold{axis: 0, value: value})
                        },
                        "y" => {
                            folds.push(Fold{axis: 1, value: value})
                        },
                        _ => ()
                    }
                }
                
            },
            Err(_error) => { panic!("something went wrong"); }
        }
    }
    
    // Solution

    let width: u16 = points.iter().map(|p| p.x).max().unwrap();
    let height: u16 = points.iter().map(|p| p.y).max().unwrap();

    println!("Page Size {}x{}", width,height);

    // Fold delta = distance between fold and edge of page
    for fold in folds {

        match fold.axis {
            0 => {
                let mut new_points: HashSet<Point> = HashSet::new();
                for point in &points {
                    if point.x > fold.value {
                        new_points.insert(Point{x: point.x - 2 * (point.x - fold.value), y: point.y });
                    } else {
                        new_points.insert(Point{x: point.x, y: point.y });
                    }
                }
                points = new_points;
            },
            1 => {
                let mut new_points: HashSet<Point> = HashSet::new();
                for point in &points {
                    if point.y > fold.value {

                        // println!("Moving {} to {}", point.y, point.y - 2 * (point.y - fold.value));

                        new_points.insert(Point{x: point.x, y: point.y - 2 * (point.y - fold.value) });
                    } else {
                        new_points.insert(Point{x: point.x, y: point.y });
                    }
                }
                points = new_points;
            },
            _ => ()
        }

        println!("Number of visible points after {:?} : {}", fold, points.len());
    }

    println!("Number of visible points: {}", points.len());


    // Print Code

    let mut vector: Vec<Point> = points.into_iter().collect();


    vector.sort_by(|a,b| 
        match a.y.cmp(&b.y) {
            Ordering::Equal => a.x.cmp(&b.x),
            other => other,
        }
    );

    println!("{:?}", vector);

    let mut last_x = -1;
    let mut last_y = -1;

    for point in &vector{

        if last_y != point.y as i16 {
            println!();
            last_y = point.y as i16;
            last_x = -1 as i16;
        }

        for i in last_x .. point.x as i16 - 1 {
            print!(" ");
            last_x += 1;
        }

        if last_x != point.x as i16{
            print!("X");
            last_x = point.x as i16;
        } 
        
    }

}
